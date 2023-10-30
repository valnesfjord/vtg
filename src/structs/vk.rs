use serde::Deserialize;
use std::any::Any;
use std::sync::{Arc, Mutex};

use super::config::Config;
use super::context::{EventType, Platform, UnifyContext, UnifyedContext};

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
    pub object: Option<VKObject>,
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
    pub payload: Option<String>,
    pub fwd_messages: Option<Vec<VKMessage>>,
    pub reply_message: Option<Box<VKMessage>>,
    pub ref_: Option<String>,
    pub ref_source: Option<String>,
}

impl UnifyContext for VKUpdate {
    fn unify(&self, config: &Config) -> UnifyedContext {
        let event: Arc<Mutex<Box<dyn Any + Send + Sync>>>;
        let (r#type, text, chat_id, message_id, from_id) = match self.object.clone() {
            Some(VKObject::MessageNew(message)) => {
                event = Arc::new(Mutex::new(Box::new(message.clone())));
                (
                    EventType::MessageNew,
                    message.message.text.clone(),
                    message.message.peer_id,
                    message.message.id,
                    message.message.from_id,
                )
            }
            Some(VKObject::MessageEvent(message)) => {
                event = Arc::new(Mutex::new(Box::new(message.clone())));
                (
                    EventType::CallbackQuery,
                    message.payload.clone(),
                    message.peer_id,
                    message.conversation_message_id,
                    message.user_id,
                )
            }
            None => {
                event = Arc::new(Mutex::new(Box::new(())));
                (EventType::Unknown, "".to_owned(), 0, 0, 0)
            }
        };
        UnifyedContext {
            text: text.clone(),
            from_id,
            peer_id: chat_id,
            id: message_id,
            r#type,
            platform: Platform::VK,
            data: Arc::new(Mutex::new(Box::new(()))),
            config: config.to_owned(),
            event,
        }
    }
}
