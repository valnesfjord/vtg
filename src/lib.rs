//! # VTG - dual-platform bots library
//!
//! VTG is a fully functional library for creating bots for both VK and Telegram. Presents unified context and methods, for comfortable work with dual-platform bots.
//!
//! ## Features
//! - Support callback and longpoll updates
//! - 90% VK and TG API coverage (messages)
//! - Unified context for both platforms
//! - Unified context methods for both platforms
//! - Unified attachments and file uploads for both platforms
//! - Unified keyboard for both platforms
//! - Easy to use
//! - Easy to understand
//! - Easy to contribute
//!
//! ## Usage
//!
//! Example using longpoll client:
//!```
//!use std::env;
//!use vtg::{
//!    client::start_longpoll_client,
//!    structs::{
//!        config::Config,
//!        context::UnifyedContext,
//!        middleware::MiddlewareChain,
//!    },
//!};
//!async fn catch_new_message(ctx: UnifyedContext) -> UnifyedContext {
//!    ctx
//!}
//!#[tokio::main]
//!async fn main() {
//!let config = Config {
//!        vk_access_token: env::var("VK_ACCESS_TOKEN").unwrap(),
//!        vk_group_id: env::var("VK_GROUP_ID").unwrap().parse().unwrap(),
//!        tg_access_token: env::var("TG_ACCESS_TOKEN").unwrap(),
//!        vk_api_version: "5.199".to_owned(),
//!        ..Default::default()
//!    };
//!    let mut middleware_chain = MiddlewareChain::new();
//!    middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));
//!
//!    start_longpoll_client(middleware_chain, config).await;
//!}
//!```
//!
//! Example using callback server:
//!```
//!use std::env;
//!
//!use vtg::{
//!    server::start_callback_server,
//!    structs::{
//!        config::{CallbackSettings, Config},
//!        context::UnifyedContext,
//!        middleware::MiddlewareChain,
//!    },
//!};
//!async fn catch_new_message(ctx: UnifyedContext) -> UnifyedContext {
//!    ctx
//!}
//!#[tokio::main]
//!async fn main() {
//!    let config = Config {
//!        vk_access_token: env::var("VK_ACCESS_TOKEN").unwrap(),
//!        vk_group_id: env::var("VK_GROUP_ID").unwrap().parse().unwrap(),
//!        tg_access_token: env::var("TG_ACCESS_TOKEN").unwrap(),
//!        vk_api_version: "5.199".to_owned(),
//!        callback: Some(CallbackSettings {
//!            port: 1234,
//!            callback_url: "https://valnesfjord.com".to_string(),
//!            secret: "secret".to_string(),
//!            path: "yourcallbacksecretpathwithoutslashinstartandend".to_string(),
//!        }),
//!    };
//!    let mut middleware_chain = MiddlewareChain::new();
//!    middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));
//!
//!    start_callback_server(middleware_chain, config).await;
//!}
//!```
//!
//! ## Context
//! Context is a struct that contains all the information about the event that happened in the chat
//!
//! You can see context documentation [here](structs/context/struct.UnifyedContext.html)
//!
//! ## Examples
//! You can find example bot in the examples folder
//!
//! ## It's not finished yet:
//!
//! - [ ] Add more tests
//! - [ ] Add more examples
//! - [ ] Add VK and TG API documentation
//! - [ ] Add more features (like more API coverage, etc)

/// Longpoll client for getting updates from VK and Telegram
///
/// This module contains function for getting updates from VK and Telegram
pub mod client;

/// Module for keyboard creation
///
/// This module contains function for creating keyboards for VK and Telegram
pub mod keyboard;

/// Module for callback (webhook) server for getting updates from VK and Telegram
///
/// This module contains function for starting callback server
pub mod server;
#[cfg(test)]
mod tests;

/// Module for all the structures used in the library
///
/// This module contains all the structures used in the library
pub mod structs;

/// Module for all the functions for working with uploads
///
/// This module contains all the functions for working with uploads
pub mod upload;
