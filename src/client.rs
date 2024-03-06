pub mod api_requests;
pub mod requests;
use log::{debug, info, log_enabled};
use requests::*;
use std::time::Duration;
use std::{panic, sync::Arc};
use tokio::time::interval;
use tokio::{select, sync::Mutex};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    time::Instant,
};

use crate::structs::config::Config;
use crate::structs::context::{UnifyContext, UnifyedContext};
use crate::structs::middleware::MiddlewareChain;
use crate::structs::tg::TGGetUpdates;
use crate::structs::vk::{VKGetServerResponse, VKGetUpdates};
pub async fn get_vk_updates(
    server: &mut str,
    key: &mut str,
    ts: &mut i64,
    tx: &Sender<UnifyedContext>,
    config: &Config,
) {
    let get_updates = request(
        server,
        &config.vk_access_token,
        vec![
            ("act", "a_check"),
            ("key", key),
            ("ts", &ts.to_string()),
            ("wait", "25"),
        ],
    )
    .await;
    let updates: VKGetUpdates = serde_json::from_str(&get_updates.unwrap_or("".to_string()))
        .unwrap_or(VKGetUpdates {
            ts: ts.to_string(),
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
    *ts += 1;
}
pub async fn get_vk_settings(config: &Config) -> VKGetServerResponse {
    let vk_group_id = config.vk_group_id.to_string();
    let get_server = request(
        "https://api.vk.com/method/groups.getLongPollServer",
        &config.vk_access_token,
        vec![("group_id", &vk_group_id), ("v", &config.vk_api_version)],
    )
    .await;
    let server: VKGetServerResponse =
        serde_json::from_str(&get_server.unwrap_or("".to_string())).unwrap();
    debug!(
        "[LONGPOLL] [VK] Got longpoll server: {}",
        server.response.server
    );
    server
}
pub async fn get_tg_updates(offset: &mut i64, tx: &Sender<UnifyedContext>, config: &Config) {
    let get_updates = request(
        &format!(
            "https://api.telegram.org/{}/getUpdates",
            config.tg_access_token
        ),
        "",
        vec![
            ("timeout", "25"),
            ("offset", &offset.to_string()),
            ("limit", "100"),
        ],
    )
    .await;

    let updates: TGGetUpdates = serde_json::from_str(&get_updates.unwrap_or("".to_string()))
        .unwrap_or(TGGetUpdates {
            ok: false,
            result: vec![],
        });
    debug!(
        "[LONGPOLL] [TELEGRAM] Got {} updates, processing",
        updates.result.len()
    );
    for update in updates.result {
        let unified = update.unify(config);
        tx.send(unified).await.unwrap();
        *offset = update.update_id + 1;
    }
}

pub async fn start_longpoll_client(middleware: MiddlewareChain, config: Config) {
    info!("Start getting updates...");
    let vk_settings = get_vk_settings(&config).await;
    let mut server = vk_settings.response.server;
    let mut key = vk_settings.response.key;
    let mut ts = vk_settings.response.ts.parse::<i64>().unwrap();
    let mut offset: i64 = 0;

    let (tx, rx): (Sender<UnifyedContext>, Receiver<UnifyedContext>) = channel(100);
    let rx = Arc::new(Mutex::new(rx));
    let middleware = Arc::new(middleware);
    for _i in 0..4 {
        let rx_clone = Arc::clone(&rx);
        let middleware_clone = Arc::clone(&middleware);
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
    let mut interval = interval(Duration::from_secs(600));
    loop {
        let vk_task = get_vk_updates(&mut server, &mut key, &mut ts, &tx, &config);
        let tg_task = get_tg_updates(&mut offset, &tx, &config);
        select! {
            _ = vk_task => {
            },
            _ = tg_task => {
            },
            _ = interval.tick() => {
            let vk_settings = get_vk_settings(&config).await;
            server = vk_settings.response.server;
            key = vk_settings.response.key;
            ts = vk_settings.response.ts.parse::<i64>().unwrap();
            },
        }
    }
}
