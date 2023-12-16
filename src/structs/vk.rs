use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::{Arc, Mutex};

use super::config::Config;
use super::context::{EventType, Platform, UnifyContext, UnifyedContext};
use super::vk_attachments::{unify_attachments, VKAttachment};

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

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
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
    pub attachments: Option<Vec<VKAttachment>>,
    pub important: Option<bool>,
    pub random_id: Option<i64>,
    pub conversation_message_id: Option<i64>,
    pub action: Option<VKMessageAction>,
    pub admin_author_id: Option<i64>,
    pub conversation_chat_id: Option<i64>,
    pub is_hidden: Option<bool>,
    pub update_time: Option<i64>,
    pub was_listened: Option<bool>,
    pub pinned_at: Option<i64>,
    pub message_tag: Option<String>,
    pub is_cropped: Option<bool>,
    pub members_count: Option<i64>,
    pub geo: Option<VKGeo>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKGeo {
    pub coordinates: Option<VKCoordinates>,
    pub place: Option<VKPlace>,
    pub showmap: Option<bool>,
    pub type_: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKCoordinates {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKPlace {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created: Option<i64>,
    pub icon: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub type_: Option<i64>,
    pub group_id: Option<i64>,
    pub group_photo: Option<String>,
    pub checkins: Option<i64>,
    pub updated: Option<i64>,
    pub address: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKMessageAction {
    pub r#type: String,
    pub member_id: Option<i64>,
    pub text: Option<String>,
    pub email: Option<String>,
    pub photo: Option<VKPhoto>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKPhoto {
    pub photo_50: Option<String>,
    pub photo_100: Option<String>,
    pub photo_200: Option<String>,
}

impl UnifyContext for VKUpdate {
    fn unify(&self, config: &Config) -> UnifyedContext {
        let event: Arc<Mutex<Box<dyn Any + Send + Sync>>>;
        let (r#type, text, chat_id, message_id, from_id, attachments) = match self.object.clone() {
            Some(VKObject::MessageNew(message)) => {
                event = Arc::new(Mutex::new(Box::new(message.clone())));
                (
                    EventType::MessageNew,
                    message.message.text.clone(),
                    message.message.peer_id,
                    message.message.id,
                    message.message.from_id,
                    unify_attachments(Some(message.message)),
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
                    unify_attachments(None),
                )
            }
            None => {
                event = Arc::new(Mutex::new(Box::new(())));
                (
                    EventType::Unknown,
                    "".to_owned(),
                    0,
                    0,
                    0,
                    unify_attachments(None),
                )
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
            attachments,
        }
    }
}
