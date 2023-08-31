use regex_automata::{meta::Regex, util::captures::Captures};
use std::{future::Future, pin::Pin};

use crate::client::structs::UnifyedContext;

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
    ctx.send("Hello");
    println!("{:?}", get_potential_matches(ctx.clone().text, caps));
    ctx
}
pub async fn ping_function(ctx: UnifyedContext) -> UnifyedContext {
    ctx.send("Pong");
    ctx
}

pub async fn bye_function(ctx: UnifyedContext) -> UnifyedContext {
    ctx.send("Goodbye");
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
