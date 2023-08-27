use client::{
    structs::{EventType, MiddlewareChain},
    *,
};
use regex_automata::dfa::Automaton;
use std::env;
pub mod client;
use crate::client::structs::{Config, UnifyedContext};
mod commands;
use lazy_static::lazy_static;
lazy_static! {
    static ref COMMAND_VEC: Vec<commands::Command> = commands::command_vec();
}

async fn catch_new_message(ctx: UnifyedContext) -> UnifyedContext {
    if ctx.r#type != EventType::MessageNew {
        return ctx;
    }
    ctx
}
use regex_automata::Input;
async fn hears_middleware(ctx: UnifyedContext) -> UnifyedContext {
    println!("{:?}", ctx.text);
    for command in COMMAND_VEC.iter() {
        if command
            .regex
            .try_search_fwd(&Input::new(ctx.text.as_str()))
            .unwrap()
            .is_some()
        {
            return (command.function)(ctx).await;
        }
    }

    default_function(ctx).await
}
async fn default_function(ctx: UnifyedContext) -> UnifyedContext {
    println!("Default function called");
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
    };
    let mut middleware_chain = MiddlewareChain::new();
    middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));
    middleware_chain.add_middleware(|ctx| Box::pin(hears_middleware(ctx)));

    start_longpoll_client(middleware_chain, config).await;
}
