use regex_automata::{meta::Regex, util::captures::Captures};
use std::{future::Future, pin::Pin};

use vtg::{
    client::structs::{EventType, Platform, TGMessage, UnifyedContext, VKMessageNew},
    keyboard::KeyboardButton,
};

pub struct Command {
    pub regex: Regex,
    pub function: fn(
        UnifyedContext,
        Captures,
    ) -> Pin<Box<dyn Future<Output = UnifyedContext> + Send + 'static>>,
}
pub fn get_potential_matches(text: String, caps: Captures) -> Vec<String> {
    caps.iter()
        .map(|a| text.as_str().get(a.unwrap().range()).unwrap().to_string())
        .collect()
}

pub async fn hello_function(ctx: UnifyedContext, caps: Captures) -> UnifyedContext {
    println!("{:?}", get_potential_matches(ctx.clone().text, caps));
    ctx.send_with_keyboard(
        "Hello",
        vtg::keyboard::Keyboard::new(
            vec![vec![KeyboardButton {
                color: Some("positive".to_string()),
                text: "Посмотреть баланс".to_string(),
                data: Some("{\"text\": \"balance\"}".to_string()),
                url: None,
            }]],
            true,
            None,
        ),
    );
    ctx
}
pub async fn ping_function(ctx: UnifyedContext) -> UnifyedContext {
    ctx.send("Pong");
    println!("{:?}", ctx);
    let data = ctx.get_data::<i32>().unwrap();
    println!("{:?}", data);
    ctx
}

pub async fn bye_function(ctx: UnifyedContext) -> UnifyedContext {
    ctx.send("Goodbye");
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
    ctx
}

pub fn command_vec() -> Vec<Command> {
    vec![
        Command {
            regex: Regex::new(r"(?:hello|hi)\s(.*)").unwrap(),
            function: |ctx, caps| Box::pin(hello_function(ctx, caps)),
        },
        Command {
            regex: Regex::new(r"!ping").unwrap(),
            function: |ctx, _| Box::pin(ping_function(ctx)),
        },
        Command {
            regex: Regex::new(r"bye|goodbye").unwrap(),
            function: |ctx, _| Box::pin(bye_function(ctx)),
        },
    ]
}
#[allow(dead_code)]
fn main() {}