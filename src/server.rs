use crate::client::api_requests::api_call;
use crate::structs::config::Config;
use crate::structs::context::{Platform, UnifyContext, UnifyedContext};
use crate::structs::middleware::MiddlewareChain;
use crate::structs::tg::TGUpdate;
use crate::structs::vk::VKUpdate;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
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
    req: Request<Body>,
    config: Arc<Config>,
    tx: Arc<Sender<UnifyedContext>>,
) -> Result<Response<Body>, Infallible> {
    let uri = req.uri();
    let settings = config.callback.as_ref().unwrap();
    match uri.path() {
        path if path == format!("/{}/vk", settings.path) => {
            let bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let update: VKUpdate =
                serde_json::from_str(String::from_utf8(bytes.to_vec()).unwrap().as_str()).unwrap();
            if update.r#type == "confirmation" {
                return Ok(Response::builder()
                    .status(200)
                    .header("Content-Type", "text/plain")
                    .body(Body::from(config.callback.clone().unwrap().secret.clone()))
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
                    .body(Body::from("Forbidden"))
                    .unwrap());
            }
            let bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
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
        .body(Body::from("OK"))
        .unwrap())
}
pub async fn start_callback_server(middleware: MiddlewareChain, config: Config) {
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
        /*
        panic::set_hook(Box::new(move |_| {
            let _ = api_call(
                Platform::Telegram,
                "deleteWebhook",
                vec![],
                &cleanup.config.clone(),
            );
            info!("Shutting down...");
        })); */
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
