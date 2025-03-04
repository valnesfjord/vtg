use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt;
use std::sync::Arc;

use super::config::Config;
use super::context::{Event, EventType, Platform, UnifyContext, UnifyedContext};
use super::vk_attachments::{VKAttachment, unify_attachments};

#[derive(Deserialize, Debug)]
pub struct VKGetServer {
    pub key: String,
    pub server: String,
    #[serde(deserialize_with = "deserialize_ts")]
    pub ts: String,
}
#[derive(Deserialize, Debug)]
pub struct VKGetServerResponse {
    pub response: VKGetServer,
}

#[derive(Deserialize, Debug)]
pub struct VKGetUpdates {
    pub failed: Option<i16>,
    #[serde(deserialize_with = "deserialize_ts")]
    pub ts: String,
    pub updates: Option<Vec<VKUpdate>>,
}

#[derive(Deserialize, Debug)]
pub struct VKTs {
    #[serde(deserialize_with = "deserialize_ts")]
    pub ts: String,
}

fn deserialize_ts<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct TSVisitor;

    impl Visitor<'_> for TSVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or number")
        }

        fn visit_str<E>(self, value: &str) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_string<E>(self, value: String) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    deserializer.deserialize_any(TSVisitor)
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VKMessageEvent {
    pub user_id: i64,
    pub peer_id: i64,
    pub event_id: String,
    pub payload: String,
    pub conversation_message_id: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VKMessageNew {
    pub message: VKMessage,
}

#[skip_serializing_none]
#[derive(Deserialize, Clone, Debug, Serialize, Default)]
/// VK Message struct, contains all the information about the new message
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
    pub r#type: Option<String>,
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
    pub r#type: Option<i64>,
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

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKProfile {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub is_closed: bool,
    pub deactivated: bool,
    pub can_access_closed: bool,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKGroup {
    pub id: i64,
    pub name: String,
    pub screen_name: String,
    pub is_closed: bool,
    pub deactivated: String,
    pub is_admin: bool,
    pub admin_level: i64,
    pub is_member: bool,
    pub is_advertiser: bool,
    pub invited_by: i64,
    pub r#type: String,
    pub photo_50: String,
    pub photo_100: String,
    pub photo_200: String,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKConversation {
    pub peer: VKPeer,
    pub in_read: i64,
    pub out_read: i64,
    pub unread_count: i64,
    pub important: bool,
    pub unanswered: bool,
    pub push_settings: VKPushSettings,
    pub can_write: VKCanWrite,
    pub chat_settings: Option<VKChatSettings>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKPeer {
    pub id: i64,
    pub r#type: String,
    pub local_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKPushSettings {
    pub disabled_until: i64,
    pub disabled_forever: bool,
    pub no_sound: bool,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKCanWrite {
    pub allowed: bool,
    pub reason: i64,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKChatSettings {
    pub owner_id: i64,
    pub title: String,
    pub pinned_message: Option<VKMessage>,
    pub state: String,
    pub photo: Option<VKChatPhoto>,
    pub active_ids: Option<Vec<i64>>,
    pub is_group_channel: Option<bool>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct VKChatPhoto {
    pub photo_50: String,
    pub photo_100: String,
    pub photo_200: String,
}

impl UnifyContext for VKUpdate {
    fn unify(&self, config: Arc<Config>) -> UnifyedContext {
        let event: Event;
        let (r#type, text, chat_id, message_id, from_id, attachments) = match self.object.clone() {
            Some(VKObject::MessageNew(message)) => {
                event = Event::VKMessageNew(message.clone());
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
                event = Event::VKMessageEvent(message.clone());
                (
                    EventType::CallbackQuery,
                    message.payload,
                    message.peer_id,
                    message.conversation_message_id,
                    message.user_id,
                    unify_attachments(None),
                )
            }
            None => {
                event = Event::Unknown;
                (
                    EventType::Unknown,
                    String::new(),
                    0,
                    0,
                    0,
                    unify_attachments(None),
                )
            }
        };
        UnifyedContext {
            text,
            from_id,
            peer_id: chat_id,
            id: message_id,
            r#type,
            platform: Platform::VK,
            data: String::new(),
            config,
            event,
            attachments,
        }
    }
}
