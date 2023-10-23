use crate::client::api_requests::api_call;
use crate::client::structs::{
    Config, MiddlewareChain, Platform, TGUpdate, UnifyContext, UnifyedContext, VKUpdate,
};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::{debug, error, info, log_enabled};
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr, process};
use tokio::signal;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::time::Instant;
async fn handle_request(
    req: Request<Body>,
    config: Config,
    tx: Arc<Sender<UnifyedContext>>,
) -> Result<Response<Body>, Infallible> {
    let uri = req.uri();
    match uri.path() {
        path if path == config.callback.clone().unwrap().path.clone() + "/vk" => {
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
        path if path == config.callback.clone().unwrap().path.clone() + "/telegram" => {
            let headers = req.headers();
            let secret_token = match headers.get("X-Telegram-Bot-Api-Secret-Token") {
                Some(value) => value.to_str().unwrap(),
                None => "",
            };
            if *secret_token != config.callback.clone().unwrap().secret.clone() {
                return Ok(Response::builder()
                    .status(403)
                    .header("Content-Type", "text/plain")
                    .body(Body::from("Forbidden"))
                    .unwrap());
            }
            let bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let update: TGUpdate =
                serde_json::from_str(String::from_utf8(bytes.to_vec()).unwrap().as_str())
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
    let settings = config.clone().callback.unwrap();
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
    api_call(
        Platform::Telegram,
        "setWebhook",
        vec![
            (
                "url",
                (settings.callback_url.clone() + "/telegram").as_str(),
            ),
            ("secret_token", settings.secret.clone().as_str()),
        ],
        &config,
    )
    .await
    .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.clone().port));

    let cfg = config.clone();
    let tx = Arc::new(tx);

    let make_svc = make_service_fn(move |_| {
        let tx = tx.clone();
        let cfg = cfg.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, cfg.clone(), tx.clone())
            }))
        }
    });

    tokio::task::spawn(async move {
        if let Err(err) = Server::bind(&addr).serve(make_svc).await {
            error!("Error serving connection: {:?}", err);
        }
    });

    debug!(
        "Callback server started on http://0.0.0.0:{}/{}/telegram and http://0.0.0.0:{}/{}/vk",
        settings.port, settings.path, settings.port, settings.path
    );

    tokio::task::spawn(async move {
        let cfg = config.clone();
        match signal::ctrl_c().await {
            Ok(()) => {
                api_call(Platform::Telegram, "deleteWebhook", vec![], &cfg)
                    .await
                    .unwrap();
                info!("Shutting down...");
                process::exit(0);
            }
            Err(err) => {
                error!("Unable to listen for shutdown signal: {}", err);
            }
        }
    });
}
