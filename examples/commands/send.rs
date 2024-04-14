use vtg::structs::{
    context::{EventType, Platform, UnifyedContext},
    tg::TGMessage,
    vk::VKMessageNew,
};

use crate::commands::get_potential_matches;
use regex_automata::util::captures::Captures;
pub async fn test_matches(ctx: UnifyedContext, caps: Captures) {
    println!("{:?}", get_potential_matches(ctx.clone().text, caps));
    ctx.send("test matches (check console)");
}

pub async fn test_data(ctx: UnifyedContext) {
    let data = ctx.get_data::<i32>().unwrap();
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
                let event = ctx.get_event::<TGMessage>().unwrap();
                println!("{:?}", event);
            }
            Platform::VK => {
                let event = ctx.get_event::<VKMessageNew>().unwrap();
                println!("{:?}", event);
            }
        }
    }
    ctx.send("test event (check console)");
}
