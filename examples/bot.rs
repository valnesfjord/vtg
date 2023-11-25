use std::env;
use vtg::{
    client::start_longpoll_client,
    structs::{context::Platform, tg_attachments::TGAttachment, vk_attachments::VKAttachment},
};
extern crate vtg;
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
    if ctx.platform == Platform::VK {
        let attachments = ctx.get_attachments::<VKAttachment>().unwrap_or_default();
        println!("{:?}", attachments);
    } else {
        let attachments = ctx.get_attachments::<TGAttachment>().unwrap_or_default();
        println!("{:?}", attachments);
    }
    ctx
}
use regex_automata::Input;
use vtg::structs::{
    config::Config,
    context::{EventType, UnifyedContext},
    middleware::MiddlewareChain,
};

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
    env::set_var("RUST_LOG", "vtg");
    env_logger::init();
    let vk_access_token = env::var("VK_ACCESS_TOKEN").unwrap();
    let vk_group_id = env::var("VK_GROUP_ID").unwrap();
    let tg_access_token = env::var("TG_ACCESS_TOKEN").unwrap();
    let config = Config {
        vk_access_token,
        vk_group_id: vk_group_id.parse().unwrap(),
        tg_access_token,
        vk_api_version: "5.131".to_owned(),
        ..Default::default()
    };
    let mut middleware_chain = MiddlewareChain::new();
    middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));
    middleware_chain.add_middleware(|ctx| Box::pin(hears_middleware(ctx)));

    start_longpoll_client(middleware_chain, config).await;
}
