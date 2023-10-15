pub mod api_requests;
pub mod requests;
pub mod structs;
use log::{debug, info, log_enabled};
use requests::*;
use std::{panic, sync::Arc};
use structs::*;
use tokio::{select, sync::Mutex};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    time::Instant,
};
pub async fn get_vk_updates(
    server: &mut String,
    key: &mut String,
    ts: &mut String,
    tx: &Sender<UnifyedContext>,
    config: &Config,
) {
    let get_updates = request(
        format!("{}", server),
        config.vk_access_token.clone(),
        vec![
            ("act", "a_check"),
            ("key", key.as_str()),
            ("ts", ts.as_str()),
            ("wait", "25"),
        ],
    )
    .await;
    let updates: VKGetUpdates = serde_json::from_str(
        get_updates.unwrap_or("".to_string()).as_str(),
    )
    .unwrap_or(VKGetUpdates {
        ts: ts.clone(),
        updates: vec![],
    });
    debug!(
        "[LONGPOLL] [VK] Got {} updates, processing",
        updates.updates.len()
    );
    for update in updates.updates {
        let unified = update.unify(config);
        tx.send(unified).await.unwrap();
    }
    let new_ts = ts.parse::<i64>().unwrap() + 1;
    *ts = new_ts.to_string();
}
pub async fn get_vk_settings(config: &Config) -> VKGetServerResponse {
    let vk_group_id = config.vk_group_id.to_string();
    let get_server = request(
        "https://api.vk.com/method/groups.getLongPollServer".to_owned(),
        config.vk_access_token.clone(),
        vec![("group_id", vk_group_id.as_str()), ("v", "5.131")],
    )
    .await;
    let server: VKGetServerResponse =
        serde_json::from_str(get_server.unwrap_or("".to_string()).as_str()).unwrap();
    debug!(
        "[LONGPOLL] [VK] Got longpoll server: {}",
        server.response.server
    );
    server
}
pub async fn get_tg_updates(offset: &mut i64, tx: &Sender<UnifyedContext>, config: &Config) {
    let mut req_body = vec![("timeout", "25")];
    let off = offset.to_string();
    if off == "0" {
        req_body.push(("limit", "1"));
    } else {
        req_body.push(("offset", off.as_str()));
        req_body.push(("limit", "100"));
    }
    let get_updates = request(
        format!(
            "https://api.telegram.org/{}/getUpdates",
            config.tg_access_token.clone()
        ),
        "".to_owned(),
        req_body,
    )
    .await;

    let updates: TGGetUpdates = serde_json::from_str(
        get_updates.unwrap_or("".to_string()).as_str(),
    )
    .unwrap_or(TGGetUpdates {
        ok: false,
        result: vec![],
    });
    debug!(
        "[LONGPOLL] [TELEGRAM] Got {} updates, processing",
        updates.result.len()
    );
    for update in updates.result.clone() {
        let unified = update.unify(&config);
        tx.send(unified).await.unwrap();
        *offset = update.update_id + 1;
    }
}

pub async fn start_longpoll_client(middleware: MiddlewareChain, config: Config) {
    info!("Start getting updates...");
    let vk_settings = get_vk_settings(&config).await;
    let mut server = vk_settings.response.server;
    let mut key = vk_settings.response.key;
    let mut ts = vk_settings.response.ts;
    let mut offset: i64 = 0;

    let (tx, rx): (Sender<UnifyedContext>, Receiver<UnifyedContext>) = channel(100);
    let rx = Arc::new(Mutex::new(rx));
    for _i in 0..4 {
        let rx_clone = Arc::clone(&rx);
        let middleware_clone = middleware.clone();
        tokio::task::spawn(async move {
            loop {
                if let Some(update) = rx_clone.lock().await.recv().await {
                    if log_enabled!(log::Level::Debug) {
                        let start_time = Instant::now();
                        middleware_clone.execute(update).await;
                        let end_time = Instant::now();
                        let elapsed_time = end_time.duration_since(start_time);
                        debug!("Processing time: {:?}", elapsed_time);
                    } else {
                        middleware_clone.execute(update).await;
                    }
                }
            }
        });
    }
    loop {
        let vk_task = get_vk_updates(&mut server, &mut key, &mut ts, &tx, &config);
        let tg_task = get_tg_updates(&mut offset, &tx, &config);
        select! {
            _ = vk_task => {
            },
            _ = tg_task => {
            },
        }
    }
}
