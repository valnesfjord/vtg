use serde::Deserialize;
use std::any::Any;

use std::sync::{Arc, Mutex};

use super::config::Config;
use super::context::{EventType, Platform, UnifyContext, UnifyedContext};

#[derive(Deserialize, Clone, Debug)]
pub struct TGGetUpdates {
    pub ok: bool,
    pub result: Vec<TGUpdate>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct TGUpdate {
    pub message: Option<TGMessage>,
    pub edited_message: Option<TGMessage>,
    pub inline_query: Option<TGInlineQuery>,
    pub chosen_inline_result: Option<TGChosenInlineResult>,
    pub callback_query: Option<TGCallbackQuery>,
    pub update_id: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TGCallbackQuery {
    pub id: String,
    pub from: TGFrom,
    pub message: Option<TGMessage>,
    pub chat_instance: String,
    pub data: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TGChosenInlineResult {
    pub result_id: String,
    pub from: TGFrom,
    pub query: String,
}

#[derive(Deserialize, Clone, Debug)]
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
    pub reply_to_message: Option<Box<TGMessage>>,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct TGFrom {
    pub id: i64,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct TGChat {
    pub id: i64,
}

impl UnifyContext for TGUpdate {
    fn unify(&self, config: &Config) -> UnifyedContext {
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
            from_id,
            peer_id: chat_id,
            id: message_id,
            r#type,
            platform: Platform::Telegram,
            data: Arc::new(Mutex::new(Box::new(()))),
            config: config.to_owned(),
            event,
        }
    }
}
