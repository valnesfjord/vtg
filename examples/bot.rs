use commands::KeyboardData;
use lazy_static::lazy_static;
use regex_automata::Input;
use std::env;
use vtg::structs::{
    config::Config,
    context::{Event, EventType, UnifyedContext},
    middleware::MiddlewareChain,
    struct_to_vec::param,
};
use vtg::{client::start_longpoll_client, structs::context::Platform};

extern crate vtg;
mod commands;

lazy_static! {
    static ref COMMAND_VEC: Vec<commands::Command> = commands::command_vec();
}

async fn catch_new_message(mut ctx: UnifyedContext) -> UnifyedContext {
    if ctx.r#type != EventType::MessageNew {
        return ctx;
    }

    ctx.set_data(54.to_string());

    if ctx.platform == Platform::VK {
        let Event::VKMessageNew(event) = &ctx.event else {
            return ctx;
        };

        if let Some(payload) = &event.message.payload {
            let k: KeyboardData = serde_json::from_str(payload).unwrap_or(KeyboardData {
                text: "".to_string(),
            });
            if !k.text.is_empty() {
                ctx.text = k.text;
            }
        }
    }

    ctx
}

async fn catch_tg_callback(mut ctx: UnifyedContext) -> UnifyedContext {
    if ctx.r#type != EventType::CallbackQuery {
        return ctx;
    }

    let Event::TGCallbackQuery(event) = &ctx.event else {
        return ctx;
    };

    if let Some(data) = &event.data {
        let k: KeyboardData = serde_json::from_str(data).unwrap_or(KeyboardData {
            text: "".to_string(),
        });
        if !k.text.is_empty() {
            ctx.text = k.text;
        }
    }

    ctx.api_call(
        Platform::Telegram,
        "answerCallbackQuery",
        vec![param("callback_query_id", event.id.as_str())],
    )
    .await;

    ctx
}

async fn hears_middleware(ctx: UnifyedContext) -> UnifyedContext {
    if ctx.r#type != EventType::MessageNew && ctx.r#type != EventType::CallbackQuery {
        return ctx;
    }

    let input = Input::new(ctx.text.as_str());
    for command in COMMAND_VEC.iter() {
        if command.regex.is_match(input.clone()) {
            let mut caps = command.regex.create_captures();
            command.regex.captures(input.clone(), &mut caps);
            (command.function)(ctx.clone(), caps).await;
            return ctx;
        }
    }

    ctx
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "vtg");
    env_logger::init();

    let config = Config {
        vk_access_token: env::var("VK_ACCESS_TOKEN").unwrap(),
        vk_group_id: env::var("VK_GROUP_ID").unwrap().parse().unwrap(),
        tg_access_token: env::var("TG_ACCESS_TOKEN").unwrap(),
        ..Default::default()
    };

    let mut middleware_chain = MiddlewareChain::new();
    middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));
    middleware_chain.add_middleware(|ctx| Box::pin(catch_tg_callback(ctx)));
    middleware_chain.add_middleware(|ctx| Box::pin(hears_middleware(ctx)));

    start_longpoll_client(middleware_chain, config).await;
}
