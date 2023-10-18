use crate::client::api_requests::api_call;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::{debug, error, info, log_enabled};
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr, process};
use tokio::signal;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::time::Instant;

use crate::client::structs::{
    Config, MiddlewareChain, Platform, TGUpdate, UnifyContext, UnifyedContext, VKUpdate,
};
async fn handle_request(
    req: Request<Body>,
    config: Config,
    tx: Arc<Sender<UnifyedContext>>,
) -> Result<Response<Body>, Infallible> {
    let uri = req.uri();
    match uri.path() {
        "/vk" => {
            let bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let update: VKUpdate =
                serde_json::from_str(String::from_utf8(bytes.to_vec()).unwrap().as_str()).unwrap();
            if update.r#type == "confirmation" {
                return Ok(Response::builder()
                    .status(200)
                    .header("Content-Type", "text/plain")
                    .body(Body::from(config.secret.as_ref().unwrap().clone()))
                    .unwrap());
            }
            let unified = update.unify(&config);
            tx.send(unified).await.unwrap();
            Response::builder().status(200)
        }
        "/telegram" => {
            let headers = req.headers();
            let secret_token = match headers.get("X-Telegram-Bot-Api-Secret-Token") {
                Some(value) => value.to_str().unwrap(),
                None => "",
            };
            if &secret_token.to_string() != config.secret.as_ref().unwrap() {
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
            let unified = update.unify(&config);
            tx.send(unified).await.unwrap();
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
    if config.port.is_none() || config.callback_url.is_none() || config.secret.is_none() {
        panic!("Port or callback url or secret or callback path is not set");
    }
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
    let mut callback_url = config.callback_url.clone().unwrap();
    callback_url.push_str("/telegram");
    api_call(
        Platform::Telegram,
        "setWebhook".to_string(),
        vec![("url", callback_url.as_str()), ("secret_token", config.secret.clone().unwrap().as_str())],
        &config,
    )
    .await
    .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.clone().port.unwrap()));

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
    let server = Server::bind(&addr).serve(make_svc);
    tokio::task::spawn(async move {
        let cfg = config.clone();
        match signal::ctrl_c().await {
            Ok(()) => {
                api_call(
                    Platform::Telegram,
                    "deleteWebhook".to_string(),
                    vec![],
                    &cfg,
                )
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
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
