use regex_automata::dfa::dense::DFA;
use std::{future::Future, pin::Pin};

use crate::client::structs::UnifyedContext;
pub struct Command {
    pub regex: DFA<Vec<u32>>,
    pub function:
        fn(UnifyedContext) -> Pin<Box<dyn Future<Output = UnifyedContext> + Send + 'static>>,
}

pub async fn hello_function(ctx: UnifyedContext) -> UnifyedContext {
    ctx.send("Hello");
    ctx
}

pub async fn bye_function(ctx: UnifyedContext) -> UnifyedContext {
    ctx.send("Goodbye");
    ctx
}

pub fn command_vec() -> Vec<Command> {
    vec![
        Command {
            regex: DFA::new(r"hello|hi").unwrap(),
            function: |ctx| Box::pin(hello_function(ctx)),
        },
        Command {
            regex: DFA::new(r"bye|goodbye").unwrap(),
            function: |ctx| Box::pin(bye_function(ctx)),
        },
    ]
}
