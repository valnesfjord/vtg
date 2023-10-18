use std::env;
use vtg::client::{
    structs::{EventType, MiddlewareChain},
    *,
};
extern crate vtg;
use vtg::client::structs::{Config, UnifyedContext};
mod commands;
use lazy_static::lazy_static;
lazy_static! {
    static ref COMMAND_VEC: Vec<commands::Command> = commands::command_vec();
}

async fn catch_new_message(ctx: UnifyedContext) -> UnifyedContext {
    if ctx.r#type != EventType::MessageNew {
        return ctx;
    }
    ctx.set_data(54);
    ctx
}
use regex_automata::Input;
use vtg::server::start_callback_server;

async fn hears_middleware(ctx: UnifyedContext) -> UnifyedContext {
    if ctx.r#type != EventType::MessageNew {
        return ctx;
    }
    println!("{:?}", ctx.text);
    let input = Input::new(ctx.text.as_str());
    for command in COMMAND_VEC.iter() {
        if command.regex.is_match(input.clone()) {
            let mut caps = command.regex.create_captures();
            command.regex.captures(input.clone(), &mut caps);
            return (command.function)(ctx, caps).await;
        }
    }

    ctx
}
#[tokio::main]
async fn main() {
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
        secret: Some("87411319".to_string())
    };
    let mut middleware_chain = MiddlewareChain::new();
    middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));
    middleware_chain.add_middleware(|ctx| Box::pin(hears_middleware(ctx)));

    start_callback_server(middleware_chain, config).await;
}
