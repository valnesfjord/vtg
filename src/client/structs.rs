use serde::Deserialize;
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use crate::keyboard::{self, Keyboard};

use super::api_requests::{api_call, ApiResponse};
#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Clone, Debug)]
pub struct VKUpdate {
    pub r#type: String,
    pub object: VKObject,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum VKObject {
    MessageNew(VKMessageNew),
    MessageEvent(VKMessageEvent),
}
#[derive(Deserialize, Clone, Debug)]
pub struct VKMessageEvent {
    pub user_id: i64,
    pub peer_id: i64,
    pub event_id: String,
    pub payload: String,
    pub conversation_message_id: i64,
}
#[derive(Deserialize, Clone, Debug)]
pub struct VKMessageNew {
    pub message: VKMessage,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VKMessage {
    pub text: String,
    pub from_id: i64,
    pub peer_id: i64,
    pub id: i64,
    pub payload: Option<String>
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

#[derive(Deserialize, Clone, Debug)]
pub struct TGMessage {
    pub text: Option<String>,
    pub from: TGFrom,
    pub chat: TGChat,
    pub message_id: i64,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct TGFrom {
    pub id: i64,
}

#[derive(Deserialize, Clone, Copy, Debug)]
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
    pub event: Arc<Mutex<Box<dyn Any + Send + Sync>>>,
    config: Config,
}

#[derive(Debug, Clone, PartialEq)]
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
        let peer_id = self.peer_id.to_string();
        let config = self.config.clone();
        let message_str = message.to_owned();
        match self.platform {
            Platform::VK => {
                tokio::task::spawn(async move {
                    api_call(
                        Platform::VK,
                        "messages.send".to_string(),
                        vec![
                            ("peer_id", peer_id.as_str()),
                            ("message", message_str.as_str()),
                            ("random_id", "0"),
                            ("v", "5.131"),
                        ],
                        &config,
                    )
                    .await
                    .unwrap()
                });
            }
            Platform::Telegram => {
                tokio::task::spawn(async move {
                    api_call(
                        Platform::Telegram,
                        "sendMessage".to_string(),
                        vec![
                            ("chat_id", peer_id.as_str()),
                            ("text", message_str.as_str()),
                        ],
                        &config,
                    )
                    .await
                    .unwrap()
                });
            }
        }
    }
    pub fn send_with_keyboard(&self, message: &str, keyboard: Keyboard) {
        let peer_id = self.peer_id.to_string();
        let config = self.config.clone();
        let message_str = message.to_owned();
        match self.platform {
            Platform::VK => {
                let j = serde_json::to_string(&keyboard.vk_buttons).unwrap();
                println!("{}", j);
                tokio::task::spawn(async move {
                    api_call(
                        Platform::VK,
                        "messages.send".to_string(),
                        vec![
                            ("peer_id", peer_id.as_str()),
                            ("message", message_str.as_str()),
                            ("random_id", "0"),
                            ("v", "5.131"),
                            ("keyboard", j.as_str()),
                        ],
                        &config,
                    )
                    .await
                    .unwrap()
                });
            }
            Platform::Telegram => {
                let j: String;
                if !keyboard.inline {
                    j = serde_json::to_string(&keyboard::ReplyKeyboardMarkup {
                        keyboard: keyboard.tg_buttons,
                        one_time_keyboard: keyboard.one_time.unwrap(),
                    })
                    .unwrap();
                } else {
                    j = serde_json::to_string(&keyboard::InlineKeyboardMarkup {
                        inline_keyboard: keyboard.tg_buttons,
                    })
                    .unwrap();
                }
                println!("{}", j);
                tokio::task::spawn(async move {
                    api_call(
                        Platform::Telegram,
                        "sendMessage".to_string(),
                        vec![
                            ("chat_id", peer_id.as_str()),
                            ("text", message_str.as_str()),
                            ("reply_markup", j.as_str()),
                        ],
                        &config,
                    )
                    .await
                    .unwrap()
                });
            }
        }
    }
    pub async fn api_call(
        &self,
        platform: Platform,
        method: String,
        params: Vec<(&str, &str)>,
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
    pub fn get_event<T: Any + Send + Sync + Clone>(&self) -> Option<T> {
        let event = self.event.lock().unwrap();
        event.downcast_ref::<T>().cloned()
    }
}

impl UnifyContext for VKUpdate {
    fn unify(&self, config: Config) -> UnifyedContext {
        let event: Arc<Mutex<Box<dyn Any + Send + Sync>>>;
        let (r#type, text, chat_id, message_id, from_id) = match self.object.clone() {
            VKObject::MessageNew(message) => {
                event = Arc::new(Mutex::new(Box::new(message.clone())));
                (
                    EventType::MessageNew,
                    message.message.text.clone(),
                    message.message.peer_id,
                    message.message.id,
                    message.message.from_id,
                )
            }
            VKObject::MessageEvent(message) => {
                event = Arc::new(Mutex::new(Box::new(message.clone())));
                (
                    EventType::CallbackQuery,
                    Some(message.payload.clone()).unwrap_or("".to_owned()),
                    message.peer_id,
                    message.conversation_message_id,
                    message.user_id,
                )
            }
        };
        UnifyedContext {
            text: text.clone(),
            from_id: from_id,
            peer_id: chat_id,
            id: message_id,
            r#type: r#type,
            platform: Platform::VK,
            data: Arc::new(Mutex::new(Box::new(()))),
            config: config,
            event: event,
        }
    }
}

impl UnifyContext for TGUpdate {
    fn unify(&self, config: Config) -> UnifyedContext {
        let event: Arc<Mutex<Box<dyn Any + Send + Sync>>>;
        let (r#type, text, chat_id, message_id, from_id) = match self {
            TGUpdate {
                message: Some(message),
                ..
            } => {
                event = Arc::new(Mutex::new(Box::new(message.clone())));
                (
                    EventType::MessageNew,
                    message.text.clone(),
                    message.chat.id,
                    message.message_id,
                    message.from.id,
                )
            }
            TGUpdate {
                edited_message: Some(message),
                ..
            } => {
                event = Arc::new(Mutex::new(Box::new(message.clone())));
                (
                    EventType::MessageEdit,
                    message.text.clone(),
                    message.chat.id,
                    message.message_id,
                    message.from.id,
                )
            }
            TGUpdate {
                inline_query: Some(query),
                ..
            } => {
                event = Arc::new(Mutex::new(Box::new(query.clone())));
                (
                    EventType::InlineQuery,
                    Some(query.query.clone()),
                    0,
                    0,
                    query.from.id,
                )
            }
            TGUpdate {
                chosen_inline_result: Some(result),
                ..
            } => {
                event = Arc::new(Mutex::new(Box::new(result.clone())));
                (
                    EventType::ChosenInlineResult,
                    Some(result.query.clone()),
                    0,
                    0,
                    result.from.id,
                )
            }
            TGUpdate {
                callback_query: Some(query),
                ..
            } => {
                event = Arc::new(Mutex::new(Box::new(query.clone())));
                (
                    EventType::CallbackQuery,
                    query.data.clone(),
                    query.message.as_ref().unwrap().chat.id,
                    query.message.as_ref().unwrap().message_id,
                    query.from.id,
                )
            }

            _ => {
                event = Arc::new(Mutex::new(Box::new(0)));
                (EventType::Unknown, None, 0, 0, 0)
            }
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
            event: event,
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
