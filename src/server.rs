use crate::client::api_requests::api_call;
use crate::structs::config::Config;
use crate::structs::context::{Platform, UnifyContext, UnifyedContext};
use crate::structs::middleware::MiddlewareChain;
use crate::structs::tg::TGUpdate;
use crate::structs::vk::VKUpdate;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::service::service_fn;
use hyper::{Request, Response};
use log::{debug, error, info, log_enabled};
use std::panic;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};
use tokio::signal;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::time::Instant;
struct Cleanup {
    config: Arc<Config>,
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        let config = Arc::clone(&self.config);
        tokio::task::spawn(async move {
            api_call(Platform::Telegram, "deleteWebhook", vec![], &config)
                .await
                .unwrap();
            info!("Shutting down...");
        });
    }
}
async fn handle_request(
    req: Request<Full<Bytes>>,
    config: Arc<Config>,
    tx: Arc<Sender<UnifyedContext>>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let uri = req.uri();
    let settings = config.callback.as_ref().unwrap();
    match uri.path() {
        path if path == format!("/{}/vk", settings.path) => {
            let bytes = req.collect().await.unwrap().to_bytes();
            let update: VKUpdate =
                serde_json::from_str(String::from_utf8(bytes.to_vec()).unwrap().as_str()).unwrap();
            if update.r#type == "confirmation" {
                return Ok(Response::builder()
                    .status(200)
                    .header("Content-Type", "text/plain")
                    .body(Full::from(config.callback.clone().unwrap().secret.clone()))
                    .unwrap());
            }
            debug!("[CALLBACK] [VK] Got update, processing");
            tx.send(update.unify(&config)).await.unwrap();
            Response::builder().status(200)
        }
        path if path == format!("/{}/telegram", settings.path) => {
            let headers = req.headers();
            let secret_token = match headers.get("X-Telegram-Bot-Api-Secret-Token") {
                Some(value) => value.to_str().unwrap(),
                None => "",
            };
            if *secret_token != settings.secret {
                return Ok(Response::builder()
                    .status(403)
                    .header("Content-Type", "text/plain")
                    .body(Full::from("Forbidden"))
                    .unwrap());
            }
            let bytes = req.collect().await.unwrap().to_bytes();
            let update: TGUpdate = serde_json::from_str(
                &String::from_utf8(bytes.to_vec()).unwrap(),
            )
            .unwrap_or(TGUpdate {
                ..Default::default()
            });
            debug!("[WEBHOOK] [TELEGRAM] Got update, processing");
            tx.send(update.unify(&config)).await.unwrap();
            Response::builder().status(200)
        }
        _ => Response::builder().status(404),
    };

    Ok(Response::builder()
        .status(200)
        .body(Full::from("OK"))
        .unwrap())
}
///Starts callback server for getting updates from VK and Telegram
///
///Accepts middleware chain and config
///
///Note: Callback settings must be set in config, callback_url don't need to have slash in the end, path must be without slash in start and end
///
///# Examples
///
///```
///use std::env;
///
///use vtg::{
///    server::start_callback_server,
///    structs::{
///        config::{CallbackSettings, Config},
///        context::UnifyedContext,
///        middleware::MiddlewareChain,
///    },
///};
///
///async fn catch_new_message(ctx: UnifyedContext) -> UnifyedContext {
///    ctx
///}

///#[tokio::main]
///async fn main() {
///    let vk_access_token = env::var("VK_ACCESS_TOKEN").unwrap();
///    let vk_group_id = env::var("VK_GROUP_ID").unwrap();
///    let tg_access_token = env::var("TG_ACCESS_TOKEN").unwrap(); // token starts with "bot", like: bot1234567890:ABCDEFGHIJKL
///
///    let config = Config {
///        vk_access_token,
///        vk_group_id: vk_group_id.parse().unwrap(),
///        tg_access_token,
///        vk_api_version: "5.199".to_owned(),
///        callback: Some(CallbackSettings {
///            port: 1234,
///            callback_url: "https://valnesfjord.com".to_string(),
///            secret: "secret".to_string(),
///            path: "yourcallbacksecretpathwithoutslashinstartandend".to_string(),
///        }),
///    };
///
///    let mut middleware_chain = MiddlewareChain::new();
///    middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));
///
///    start_callback_server(middleware_chain, config).await;
///}
///```
pub async fn start_callback_server(middleware: MiddlewareChain, config: Config) {
    let config = config.check();
    if config.callback.is_none() {
        panic!("Callback settings don't exist in config");
    }
    let settings = config.callback.as_ref().unwrap();
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
    api_call(
        Platform::Telegram,
        "setWebhook",
        vec![
            (
                "url",
                &format!("{}/{}/telegram", settings.callback_url, settings.path),
            ),
            ("secret_token", &settings.secret),
        ],
        &config,
    )
    .await
    .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.port));
    debug!(
        "Callback server started on http://0.0.0.0:{}/{}/telegram and http://0.0.0.0:{}/{}/vk",
        settings.port, settings.path, settings.port, settings.path
    );

    let cfg = Arc::new(config);
    let cleanup = Cleanup {
        config: Arc::clone(&cfg),
    };
    tokio::task::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                api_call(Platform::Telegram, "deleteWebhook", vec![], &cleanup.config)
                    .await
                    .unwrap();
                info!("Shutting down...");
                std::process::exit(0);
            }
            Err(err) => {
                error!("Unable to listen for shutdown signal: {}", err);
            }
        }
    });

    let tx = Arc::new(tx);

    let make_svc = make_service_fn(move |_| {
        let tx = Arc::clone(&tx);
        let cfg = Arc::clone(&cfg);

        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, cfg.clone(), tx.clone())
            }))
        }
    });

    if let Err(err) = Server::bind(&addr).serve(make_svc).await {
        error!("Error serving connection: {:?}", err);
    }
}
