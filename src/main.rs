use client::{
    structs::{EventType, MiddlewareChain},
    *,
};
use regex_automata::dfa::Automaton;

pub mod client;
use crate::client::structs::UnifyedContext;
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
    let mut middleware_chain = MiddlewareChain::new();
    middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));
    middleware_chain.add_middleware(|ctx| Box::pin(hears_middleware(ctx)));

    start_longpoll_client(middleware_chain).await;
}
