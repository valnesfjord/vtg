use std::env;
use crate::client::requests::request;

    use httpmock::prelude::*;
use crate::client::structs::{Config, MiddlewareChain, UnifyedContext};
use crate::server::start_callback_server;

#[tokio::test]
async fn requests() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock = server.mock(|when, then| {
            when.method(POST).path("/test").query_param("test", "true");
            then.status(200).header("content-type", "").body("hello");
        });
        let resp = request(
            server.url("/test?test=true"),
            "".to_string(),
            vec![("test", "true")],
        )
        .await
        .unwrap();
        mock.assert();
        assert_eq!("hello".to_string(), resp);
}
#[tokio::test]
async fn webhook() {
    async fn default_middleware(ctx: UnifyedContext) -> UnifyedContext {


        ctx
    }
    let vk_access_token = env::var("VK_ACCESS_TOKEN").unwrap();
    let vk_group_id = env::var("VK_GROUP_ID").unwrap();
    let tg_access_token = env::var("TG_ACCESS_TOKEN").unwrap();
    let config = Config {
        vk_access_token,
        vk_group_id: vk_group_id.parse().unwrap(),
        tg_access_token,
        vk_api_version: "5.131".to_owned(),
        callback_url: Some("https://6dcd-94-253-109-231.ngrok-free.app".to_string()),
        port: Some(8080),
        secret: Some("pivovkusnoye".to_string())
    };
    let mut middleware_chain = MiddlewareChain::new();
    middleware_chain.add_middleware(|ctx| Box::pin(default_middleware(ctx)));

    start_callback_server(middleware_chain, config).await;
}