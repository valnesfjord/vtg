use serde::Deserialize;
use std::any::Any;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, future::Future};

use super::api_requests::{api_call, ApiResponse};
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
    pub edited_message: Option<TGMessage>,
    pub inline_query: Option<TGInlineQuery>,
    pub chosen_inline_result: Option<TGChosenInlineResult>,
    pub callback_query: Option<TGCallbackQuery>,
    pub update_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct TGCallbackQuery {
    pub id: String,
    pub from: TGFrom,
    pub message: Option<TGMessage>,
    pub chat_instance: String,
    pub data: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct TGChosenInlineResult {
    pub result_id: String,
    pub from: TGFrom,
    pub query: String,
}

#[derive(Deserialize, Clone)]
pub struct TGInlineQuery {
    pub id: String,
    pub from: TGFrom,
    pub query: String,
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
    pub data: Arc<Mutex<Box<dyn Any + Send + Sync>>>,
    config: Config,
}

#[derive(Debug, Clone)]
pub enum Platform {
    VK,
    Telegram,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    MessageNew,
    MessageEdit,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    Unknown,
}

pub trait UnifyContext {
    fn unify(&self, config: Config) -> UnifyedContext;
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
                let config = self.config.clone();
                let message_str = message.to_owned();
                tokio::task::spawn(async move {
                    let mut req_body = HashMap::new();
                    req_body.insert("peer_id", peer_id.as_str());
                    req_body.insert("message", message_str.as_str());
                    req_body.insert("random_id", "0");
                    req_body.insert("v", "5.131");
                    api_call(Platform::VK, "messages.send".to_string(), req_body, &config).await
                });
            }
            Platform::Telegram => {
                let peer_id = self.peer_id.to_string();
                let config = self.config.clone();
                let message_str = message.to_owned();
                tokio::task::spawn(async move {
                    let mut req_body = HashMap::new();
                    req_body.insert("chat_id", peer_id.as_str());
                    req_body.insert("text", message_str.as_str());
                    api_call(
                        Platform::Telegram,
                        "sendMessage".to_string(),
                        req_body,
                        &config,
                    )
                    .await
                });
            }
        }
    }
    pub async fn api_call(
        &self,
        platform: Platform,
        method: String,
        params: HashMap<&str, &str>,
    ) -> ApiResponse {
        api_call(platform, method, params, &self.config)
            .await
            .unwrap()
    }
    pub fn set_data<T: Any + Send + Sync>(&self, data: T) {
        let mut data_to_edit = self.data.lock().unwrap();
        *data_to_edit = Box::new(data);
    }
    pub fn get_data<T: Any + Send + Sync + Clone>(&self) -> Option<T> {
        let data = self.data.lock().unwrap();
        data.downcast_ref::<T>().cloned()
    }
}

impl UnifyContext for VKMessage {
    fn unify(&self, config: Config) -> UnifyedContext {
        UnifyedContext {
            text: self.text.clone(),
            from_id: self.from_id,
            peer_id: self.peer_id,
            id: self.id,
            r#type: EventType::MessageNew,
            platform: Platform::VK,
            data: Arc::new(Mutex::new(Box::new(()))),
            config: config,
        }
    }
}

impl UnifyContext for TGUpdate {
    fn unify(&self, config: Config) -> UnifyedContext {
        let (r#type, text, chat_id, message_id, from_id) = match self {
            TGUpdate {
                message: Some(message),
                ..
            } => (
                EventType::MessageNew,
                message.text.clone(),
                message.chat.id,
                message.message_id,
                message.from.id,
            ),
            TGUpdate {
                edited_message: Some(message),
                ..
            } => (
                EventType::MessageEdit,
                message.text.clone(),
                message.chat.id,
                message.message_id,
                message.from.id,
            ),
            TGUpdate {
                inline_query: Some(query),
                ..
            } => (
                EventType::InlineQuery,
                Some(query.query.clone()),
                0,
                0,
                query.from.id,
            ),
            TGUpdate {
                chosen_inline_result: Some(result),
                ..
            } => (
                EventType::ChosenInlineResult,
                Some(result.query.clone()),
                0,
                0,
                result.from.id,
            ),
            TGUpdate {
                callback_query: Some(query),
                ..
            } => (
                EventType::CallbackQuery,
                query.data.clone(),
                query.message.as_ref().unwrap().chat.id,
                query.message.as_ref().unwrap().message_id,
                query.from.id,
            ),

            _ => (EventType::Unknown, None, 0, 0, 0),
        };
        UnifyedContext {
            text: text.clone().unwrap_or("".to_owned()),
            from_id: from_id,
            peer_id: chat_id,
            id: message_id,
            r#type: r#type,
            platform: Platform::Telegram,
            data: Arc::new(Mutex::new(Box::new(()))),
            config: config,
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
#[derive(Debug, Clone)]
pub struct Config {
    pub vk_access_token: String,
    pub vk_group_id: i64,
    pub vk_api_version: String,
    pub tg_access_token: String,
}
