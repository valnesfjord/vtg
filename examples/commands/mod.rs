use regex_automata::{meta::Regex, util::captures::Captures, Span};
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin};
use vtg::structs::context::UnifyedContext;

mod api;
mod attachments;
mod keyboard;
mod send;

type CommandFunction = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
pub struct Command {
    pub regex: Regex,
    pub function: fn(UnifyedContext, Captures) -> CommandFunction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardData {
    pub text: String,
}

pub fn get_potential_matches(text: String, caps: Captures) -> Vec<String> {
    caps.iter()
        .map(|a| {
            text.get(a.unwrap_or(Span { start: 0, end: 0 }).range())
                .unwrap()
                .to_string()
        })
        .collect()
}

pub fn command_vec() -> Vec<Command> {
    vec![
        Command {
            regex: Regex::new(r"(?:testkeyboard)").unwrap(),
            function: |ctx, _| Box::pin(keyboard::keyboard_function(ctx)),
        },
        Command {
            regex: Regex::new(r"(?:testbuilder)").unwrap(),
            function: |ctx, _| Box::pin(api::send_with_builder(ctx)),
        },
        Command {
            regex: Regex::new(r"(?:testoptions)").unwrap(),
            function: |ctx, _| Box::pin(api::send_with_options(ctx)),
        },
        Command {
            regex: Regex::new(r"(?:testapi)").unwrap(),
            function: |ctx, _| Box::pin(api::send_with_api_request(ctx)),
        },
        Command {
            regex: Regex::new(r"(?:testmatches)").unwrap(),
            function: |ctx, caps| Box::pin(send::test_matches(ctx, caps)),
        },
        Command {
            regex: Regex::new(r"(?:testdata)").unwrap(),
            function: |ctx, _| Box::pin(send::test_data(ctx)),
        },
        Command {
            regex: Regex::new(r"(?:testctx)").unwrap(),
            function: |ctx, _| Box::pin(send::test_ctx(ctx)),
        },
        Command {
            regex: Regex::new(r"(?:testattachments)").unwrap(),
            function: |ctx, _| Box::pin(attachments::test_attachments(ctx)),
        },
        Command {
            regex: Regex::new(r"(?:sendfiles)").unwrap(),
            function: |ctx, _| Box::pin(attachments::send_files(ctx)),
        },
        Command {
            regex: Regex::new(r"(?:sendattachments)").unwrap(),
            function: |ctx, _| Box::pin(attachments::send_attachments(ctx)),
        },
        Command {
            regex: Regex::new(r"!ping").unwrap(),
            function: |ctx, _| Box::pin(send::ping_function(ctx)),
        },
        Command {
            regex: Regex::new(r"testevent").unwrap(),
            function: |ctx, _| Box::pin(send::test_event(ctx)),
        },
    ]
}
