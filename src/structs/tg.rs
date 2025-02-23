use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use std::sync::Arc;

use super::config::Config;
use super::context::{Event, EventType, Platform, UnifyContext, UnifyedContext};
use super::tg_attachments::*;

#[derive(Deserialize, Clone, Debug)]
pub struct TGGetUpdates {
    pub ok: bool,
    pub result: Vec<TGUpdate>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TGUpdate {
    pub message: Option<TGMessage>,
    pub edited_message: Option<TGMessage>,
    pub inline_query: Option<TGInlineQuery>,
    pub chosen_inline_result: Option<TGChosenInlineResult>,
    pub callback_query: Option<TGCallbackQuery>,
    pub update_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGCallbackQuery {
    pub id: String,
    pub from: TGFrom,
    pub message: Option<TGMessage>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub inline_message_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGChosenInlineResult {
    pub result_id: String,
    pub from: TGFrom,
    pub query: String,
    pub inline_message_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGInlineQuery {
    pub id: String,
    pub from: TGFrom,
    pub query: String,
    pub offset: String,
    pub chat_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct TGMessage {
    pub text: Option<String>,
    pub from: TGFrom,
    pub chat: TGChat,
    pub message_id: i64,
    pub reply_to_message: Option<Box<TGMessage>>,
    pub forward_from: Option<TGFrom>,
    pub forward_from_chat: Option<TGChat>,
    pub forward_from_message_id: Option<i64>,
    pub forward_signature: Option<String>,
    pub forward_date: Option<i64>,
    pub entities: Option<Vec<TGMessageEntity>>,
    pub caption_entities: Option<Vec<TGMessageEntity>>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub new_chat_members: Option<Vec<TGUser>>,
    pub left_chat_member: Option<TGUser>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<TGPhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<TGMessage>>,
    pub invoice: Option<TGInvoice>,
    pub successful_payment: Option<TGSuccessfulPayment>,
    pub connected_website: Option<String>,
    pub reply_to_message_id: Option<i64>,
    pub web_app_data: Option<WebAppData>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct TGMessageEntity {
    pub r#type: String,
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<TGUser>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct TGUser {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TGFrom {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
    pub added_to_attachment_menu: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TGChat {
    pub id: i64,
    pub r#type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
}

impl UnifyContext for TGUpdate {
    fn unify(&self, config: Arc<Config>) -> UnifyedContext {
        let event: Event;
        let (r#type, text, chat_id, message_id, from_id) = match self {
            TGUpdate {
                message: Some(message),
                ..
            } => {
                event = Event::TGMessage(message.clone());
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
                event = Event::TGMessage(message.clone());
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
                event = Event::TGInlineQuery(query.clone());
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
                event = Event::TGChosenInlineResult(result.clone());
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
                event = Event::TGCallbackQuery(query.clone());
                (
                    EventType::CallbackQuery,
                    query.data.clone(),
                    query.message.as_ref().unwrap().chat.id,
                    query.message.as_ref().unwrap().message_id,
                    query.from.id,
                )
            }

            _ => {
                event = Event::Unknown;
                (EventType::Unknown, None, 0, 0, 0)
            }
        };
        UnifyedContext {
            text: text.unwrap_or(String::new()),
            from_id,
            peer_id: chat_id,
            id: message_id,
            r#type,
            platform: Platform::Telegram,
            data: String::new(),
            config,
            event,
            attachments: unify_attachments(self.message.clone()),
        }
    }
}
