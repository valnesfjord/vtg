use hyper::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Body, Client, Method, Request,
};
pub mod structs;
use lazy_static::lazy_static;
use std::{collections::HashMap, panic, sync::Arc};
use structs::*;
use tokio::{select, sync::Mutex};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    time::Instant,
};
lazy_static! {
    static ref CLIENT: Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body> =
        get_client();
}
pub async fn request(
    url: String,
    access_token: String,
    body: HashMap<&str, &str>,
) -> Result<String, HyperRequestError> {
    let form_body = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(body.iter())
        .finish();
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header(CONTENT_LENGTH, form_body.len())
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(form_body))
        .unwrap();
    let res = CLIENT
        .request(req)
        .await
        .map_err(HyperRequestError::RequestError)?;

    let bytes = hyper::body::to_bytes(res.into_body())
        .await
        .map_err(|e| HyperRequestError::ResponseError(e.to_string()))?;
    Ok(String::from_utf8(bytes.to_vec())
        .map_err(|e| HyperRequestError::ResponseError(e.to_string()))?)
}
fn get_client() -> Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body> {
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();

    Client::builder().build(https)
}
pub async fn get_vk_updates(
    server: &mut String,
    key: &mut String,
    ts: &mut String,
    tx: &Sender<UnifyedContext>,
    config: &Config,
) {
    let mut req_body = HashMap::new();
    req_body.insert("act", "a_check");
    req_body.insert("key", key.as_str());
    req_body.insert("ts", ts.as_str());
    req_body.insert("wait", "25");
    let get_updates = request(
        format!("{}", server),
        config.vk_access_token.clone(),
        req_body,
    )
    .await;
    let updates: VKGetUpdates = serde_json::from_str(
        get_updates.unwrap_or("".to_string()).as_str(),
    )
    .unwrap_or(VKGetUpdates {
        ts: ts.clone(),
        updates: vec![],
    });
    for update in updates.updates {
        let message = update.object.message;
        let unified = message.unify(config.clone());
        tx.send(unified).await.unwrap();
    }
    let new_ts = ts.parse::<i64>().unwrap() + 1;
    *ts = new_ts.to_string();
}
pub async fn get_vk_settings(config: &Config) -> VKGetServerResponse {
    let mut req_body = HashMap::new();
    let vk_group_id = config.vk_group_id.to_string();
    req_body.insert("group_id", vk_group_id.as_str());
    req_body.insert("v", "5.131");
    let get_server = request(
        "https://api.vk.com/method/groups.getLongPollServer".to_owned(),
        config.vk_access_token.clone(),
        req_body,
    )
    .await;
    let server: VKGetServerResponse =
        serde_json::from_str(get_server.unwrap_or("".to_string()).as_str()).unwrap();

    server
}
pub async fn get_tg_updates(offset: &mut i64, tx: &Sender<UnifyedContext>, config: &Config) {
    let mut req_body = HashMap::new();
    let off = offset.to_string();
    if off == "0" {
        req_body.insert("limit", "1");
    } else {
        req_body.insert("offset", off.as_str());
        req_body.insert("limit", "100");
    }
    req_body.insert("timeout", "25");
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
    for update in updates.result.clone() {
        if let Some(message) = update.message {
            let unified = message.unify(config.clone());
            tx.send(unified).await.unwrap();
        }
        *offset = update.update_id + 1;
    }
}

pub async fn start_longpoll_client(middleware: MiddlewareChain, config: Config) {
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
                    let start_time = Instant::now();
                    middleware_clone.execute(update).await;
                    let end_time = Instant::now();
                    let elapsed_time = end_time.duration_since(start_time);
                    println!("Время обработки: {:?}", elapsed_time);
                }
            }
        });
    }
    loop {
        let vk_task = get_vk_updates(&mut server, &mut key, &mut ts, &tx, &config);
        let tg_task = get_tg_updates(&mut offset, &tx, &config);
        select! {
            _ = vk_task => {},
            _ = tg_task => {},
        }
    }
}
