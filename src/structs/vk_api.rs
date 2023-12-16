use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use tokio::task::JoinHandle;

use crate::client::api_requests::api_call;
use crate::structs::vk::VKMessage;

use super::{config::Config, context::Platform, struct_to_vec::struct_to_vec};
pub fn vk_api_call(
    method: &'static str,
    mut params: Vec<(&'static str, &'static str)>,
    config: Config,
) -> JoinHandle<Value> {
    tokio::task::spawn(async move {
        params.push(("v", "5.199"));
        api_call(Platform::VK, method, params, &config)
            .await
            .unwrap()
    })
}
pub struct Messages {}

impl Messages {
    pub async fn send(
        options: VKMessageSendOptions,
        config: Config,
    ) -> Result<Vec<VKMessageSendResponse>, serde_json::Error> {
        let response = vk_api_call("messages.send", struct_to_vec(options), config)
            .await
            .unwrap();
        let response = response.get("response").unwrap();
        match response.as_i64() {
            Some(response) => Ok(vec![VKMessageSendResponse {
                message_id: response,
                conversation_message_id: None,
                peer_id: None,
                error: None,
            }]),
            None => serde_json::from_value(response.clone()),
        }
    }
    pub async fn create_chat(
        options: VKMessageCreateChatOptions,
        config: Config,
    ) -> Result<i64, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.createChat", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn delete(
        options: VKMessageDeleteOptions,
        config: Config,
    ) -> Result<Vec<VKMessageDeleteResponse>, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.delete", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn delete_chat_photo(
        options: VKMessageDeleteChatPhotoOptions,
        config: Config,
    ) -> Result<VKMessageDeleteChatPhotoResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.deleteChatPhoto", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn delete_conversation(
        options: VKMessageDeleteConversationOptions,
        config: Config,
    ) -> Result<VKMessageDeleteConversationResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call(
                "messages.deleteConversation",
                struct_to_vec(options),
                config,
            )
            .await
            .unwrap()
            .get("response")
            .unwrap()
            .clone(),
        )
    }
    pub async fn delete_reaction(
        options: VKMessageDeleteReactionOptions,
        config: Config,
    ) -> Result<i8, serde_json::Error> {
        serde_json::from_value(
            vk_api_call(
                "messages.deleteConversation",
                struct_to_vec(options),
                config,
            )
            .await
            .unwrap()
            .get("response")
            .unwrap()
            .clone(),
        )
    }
    pub async fn edit(
        options: VKMessageEditOptions,
        config: Config,
    ) -> Result<i8, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.edit", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn edit_chat(
        options: VKMessageEditChatOptions,
        config: Config,
    ) -> Result<i8, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.editChat", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_by_conversation_message_id(
        options: VKMessageGetByConversationMessageIdOptions,
        config: Config,
    ) -> Result<VKMessageGetByConversationMessageIdResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call(
                "messages.getByConversationMessageId",
                struct_to_vec(options),
                config,
            )
            .await
            .unwrap()
            .get("response")
            .unwrap()
            .clone(),
        )
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessageSendOptions {
    pub peer_id: Option<i64>,
    pub peer_ids: Option<String>,
    pub domain: Option<String>,
    pub chat_id: Option<i64>,
    pub user_ids: Option<String>,
    pub guid: Option<i64>,
    pub lat: Option<String>,
    pub long: Option<String>,
    pub attachment: Option<String>,
    pub reply_to: Option<i64>,
    pub forward_messages: Option<String>,
    pub sticker_id: Option<i64>,
    pub forward: Option<String>,
    pub keyboard: Option<String>,
    pub payload: Option<String>,
    pub template: Option<String>,
    pub message: Option<String>,
    pub dont_parse_links: Option<bool>,
    pub disable_mentions: Option<bool>,
    pub intent: Option<String>,
    pub subscribe_id: Option<i64>,
    pub content_source: Option<String>,
    pub random_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessageSendResponse {
    pub peer_id: Option<i64>,
    pub message_id: i64,
    pub conversation_message_id: Option<i64>,
    pub error: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessageCreateChatOptions {
    pub user_ids: Option<String>,
    pub title: Option<String>,
    pub group_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessageDeleteOptions {
    pub message_ids: Option<String>,
    pub spam: Option<bool>,
    pub group_id: Option<i64>,
    pub delete_for_all: Option<bool>,
    pub reason: Option<i64>,
    pub peer_id: Option<i64>,
    pub cmids: Option<String>,
}
#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessageDeleteResponse {
    pub peer_id: Option<i64>,
    pub message_id: Option<i64>,
    pub conversation_message_id: Option<i64>,
    pub response: i8,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessageDeleteChatPhotoOptions {
    pub chat_id: i64,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessageDeleteChatPhotoResponse {
    pub message_id: i64,
    pub chat: Option<VKChat>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKChat {
    pub id: i64,
    pub type_: String,
    pub title: String,
    pub admin_id: i64,
    pub users: Vec<i64>,
    pub push_settings: VKPushSettings,
    pub photo_50: Option<String>,
    pub photo_100: Option<String>,
    pub photo_200: Option<String>,
    pub left: Option<i8>,
    pub kicked: Option<i8>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKPushSettings {
    pub sound: i8,
    pub disabled_until: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessageDeleteConversationOptions {
    pub user_id: Option<i64>,
    pub peer_id: Option<i64>,
    pub group_id: Option<i64>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessageDeleteConversationResponse {
    pub peer_id: Option<i64>,
    pub last_deleted_id: i64,
    pub response: Option<i8>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessageDeleteReactionOptions {
    pub peer_id: i64,
    pub cmid: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessageEditOptions {
    pub peer_id: i64,
    pub message: Option<String>,
    pub lat: Option<String>,
    pub long: Option<String>,
    pub attachment: Option<String>,
    pub keep_forward_messages: Option<bool>,
    pub keep_snippets: Option<bool>,
    pub group_id: Option<i64>,
    pub dont_parse_links: Option<bool>,
    pub disable_mentions: Option<bool>,
    pub message_id: Option<i64>,
    pub conversation_message_id: Option<i64>,
    pub template: Option<String>,
    pub keyboard: Option<String>,
    pub payload: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessageEditChatOptions {
    pub chat_id: i64,
    pub title: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessageGetByConversationMessageIdOptions {
    pub peer_id: i64,
    pub conversation_message_ids: String,
    pub extended: Option<bool>,
    pub fields: Option<String>,
    pub group_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessageGetByConversationMessageIdResponse {
    pub count: i32,
    pub items: Vec<VKMessage>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]