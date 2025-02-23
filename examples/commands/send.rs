use regex_automata::util::captures::Captures;
use vtg::structs::context::{Event, EventType, Platform, UnifyedContext};

use crate::commands::get_potential_matches;

pub async fn test_matches(ctx: UnifyedContext, caps: Captures) {
    println!("{:?}", get_potential_matches(ctx.clone().text, caps));

    ctx.send("test matches (check console)");
}

pub async fn test_data(ctx: UnifyedContext) {
    let data: i32 = ctx.data.parse().unwrap();

    println!("Data: {:?}", data);

    ctx.send("test data (check console)");
}

pub async fn test_ctx(ctx: UnifyedContext) {
    println!("{:?}", ctx);

    ctx.send("test ctx (check console)");
}

pub async fn ping_function(ctx: UnifyedContext) {
    ctx.send("Pong!");
}

pub async fn test_event(ctx: UnifyedContext) {
    if ctx.r#type == EventType::MessageNew {
        match ctx.platform {
            Platform::Telegram => {
                let Event::TGMessage(event) = &ctx.event else {
                    return;
                };
                println!("{:?}", event);
            }
            Platform::VK => {
                let Event::VKMessageNew(event) = &ctx.event else {
                    return;
                };
                println!("{:?}", event);
            }
        }
    }

    ctx.send("test event (check console)");
}
