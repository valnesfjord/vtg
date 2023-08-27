use serde::Deserialize;
use std::pin::Pin;
use std::{collections::HashMap, future::Future};

use super::request;
#[derive(Deserialize)]
pub struct VKGetServer {
    pub key: String,
    pub server: String,
    pub ts: String,
}
#[derive(Deserialize)]
pub struct VKGetServerResponse {
    pub response: VKGetServer,
}

#[derive(Deserialize)]
pub struct VKGetUpdates {
    pub ts: String,
    pub updates: Vec<VKUpdate>,
}

#[derive(Deserialize)]
pub struct VKUpdate {
    pub r#type: String,
    pub object: VKObject,
}

#[derive(Deserialize)]
pub struct VKObject {
    pub message: VKMessage,
}

#[derive(Deserialize)]
pub struct VKMessage {
    pub text: String,
    pub from_id: i64,
    pub peer_id: i64,
    pub id: i64,
}

#[derive(Deserialize, Clone)]
pub struct TGGetUpdates {
    pub ok: bool,
    pub result: Vec<TGUpdate>,
}

#[derive(Deserialize, Clone)]
pub struct TGUpdate {
    pub message: Option<TGMessage>,
    pub update_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct TGMessage {
    pub text: Option<String>,
    pub from: TGFrom,
    pub chat: TGChat,
    pub message_id: i64,
}

#[derive(Deserialize, Clone, Copy)]
pub struct TGFrom {
    pub id: i64,
}

#[derive(Deserialize, Clone, Copy)]
pub struct TGChat {
    pub id: i64,
}
#[derive(Debug, Clone)]
pub struct UnifyedContext {
    pub text: String,
    pub from_id: i64,
    pub peer_id: i64,
    pub id: i64,
    pub r#type: EventType,
    pub platform: Platform,
}
#[derive(Debug, Clone)]
pub enum Platform {
    VK,
    Telegram,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    MessageNew,
}
pub trait UnifyContext {
    fn unify(&self) -> UnifyedContext;
}
#[derive(Deserialize, Clone, Copy)]
pub struct VKNewMessageResponse {
    pub response: i64,
}

impl UnifyedContext {
    pub fn send(&self, message: &str) {
        match self.platform {
            Platform::VK => {
                let peer_id = self.peer_id.to_string();
                let message_str = message.to_owned();
                tokio::task::spawn(async move {
                    let mut req_body = HashMap::new();
                    req_body.insert("peer_id", peer_id.as_str());
                    req_body.insert("message", message_str.as_str());
                    req_body.insert("random_id", "0");
                    req_body.insert("v", "5.131");
                    let _ = request(
                    "https://api.vk.com/method/messages.send".to_owned(),
                    "11c14fe2a02b615e8561472eca997d5b0434623e1dfff3941ce180b5d5d5474cf25784e7a6b38eaed6b90"
                        .to_owned(),
                    req_body,
                ).await;
                });
            }
            Platform::Telegram => {
                let peer_id = self.peer_id.to_string();
                let message_str = message.to_owned();
                tokio::task::spawn(async move {
                    let mut req_body = HashMap::new();
                    req_body.insert("chat_id", peer_id.as_str());
                    req_body.insert("text", message_str.as_str());
                    request(
                    "https://api.telegram.org/bot6254199489:AAEi17LOSCkdnSBx50r8wGjRaBRDM8iVxIs/sendMessage".to_owned(),
                    ""
                        .to_owned(),
                    req_body,
                ).await;
                });
            }
        }
    }
}

impl UnifyContext for VKMessage {
    fn unify(&self) -> UnifyedContext {
        UnifyedContext {
            text: self.text.clone(),
            from_id: self.from_id,
            peer_id: self.peer_id,
            id: self.id,
            r#type: EventType::MessageNew,
            platform: Platform::VK,
        }
    }
}

impl UnifyContext for TGMessage {
    fn unify(&self) -> UnifyedContext {
        UnifyedContext {
            text: self.text.clone().unwrap_or("".to_owned()),
            from_id: self.from.id,
            peer_id: self.chat.id,
            id: self.message_id,
            r#type: EventType::MessageNew,
            platform: Platform::Telegram,
        }
    }
}
type Middleware =
    fn(UnifyedContext) -> Pin<Box<dyn Future<Output = UnifyedContext> + Send + 'static>>;

#[derive(Clone)]
pub struct MiddlewareChain {
    middlewares: Vec<Middleware>,
}

impl MiddlewareChain {
    pub fn new() -> Self {
        MiddlewareChain {
            middlewares: Vec::new(),
        }
    }

    pub fn add_middleware(&mut self, middleware: Middleware) {
        self.middlewares.push(middleware);
    }

    pub async fn execute(&self, ctx: UnifyedContext) -> UnifyedContext {
        let mut ctx = ctx;
        for middleware in &self.middlewares {
            ctx = middleware(ctx).await;
        }
        ctx
    }
}
