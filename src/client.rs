/// Module for making requests to VK and Telegram
///  
/// This module contains function for requests to VK and TG API
pub mod api_requests;

/// Low level module for making requests to VK and Telegram, like file requests and etc
///  
/// This module contains low level functions for file requests and etc
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

async fn get_vk_updates(
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
async fn get_vk_settings(config: &Config) -> VKGetServerResponse {
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
async fn get_tg_updates(offset: &mut i64, tx: &Sender<UnifyedContext>, config: &Config) {
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

///Starts longpoll client for getting updates from VK and Telegram
///
///Accepts middleware chain and config
///
///# Examples
///
///```
///use std::env;
///use vtg::{
///    server::start_longpoll_client,
///    structs::{
///        config::Config,
///        context::UnifyedContext,
///        middleware::MiddlewareChain,
///    },
///}
///async fn catch_new_message(ctx: UnifyedContext) -> UnifyedContext {
///    ctx
///}
///#[tokio::main]
///async fn main() {
///    let vk_access_token = env::var("VK_ACCESS_TOKEN").unwrap();
///    let vk_group_id = env::var("VK_GROUP_ID").unwrap();
///    let tg_access_token = env::var("TG_ACCESS_TOKEN").unwrap();
///    let config = Config {
///            vk_access_token,
///            vk_group_id: vk_group_id.parse().unwrap(),
///            tg_access_token,
///            vk_api_version: "5.199".to_owned(),
///            ..Default::default()
///    };
///    let mut middleware_chain = MiddlewareChain::new();
///    middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));
///
///    start_longpoll_client(middleware_chain, config).await;
///}
///```
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
                        return debug!("Processing time: {:?}", elapsed_time);
                    }
                    middleware_clone.execute(update).await
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
