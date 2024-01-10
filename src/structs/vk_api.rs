use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use serde_with::skip_serializing_none;
use tokio::task::JoinHandle;

use crate::client::requests::{files_request, FileType};
use crate::client::{api_requests::api_call, requests::File};
use crate::structs::vk::VKMessage;
use crate::upload::{download_files, Attachment};

use super::{
    config::Config,
    context::Platform,
    struct_to_vec::struct_to_vec,
    upload::VKGetUploadServerResponse,
    vk::{VKConversation, VKGroup, VKProfile},
    vk_attachments::VKAttachment,
};
pub fn vk_api_call(
    method: &'static str,
    params: Vec<(&'static str, &'static str)>,
    config: Arc<Config>,
) -> JoinHandle<Value> {
    tokio::task::spawn(async move {
        api_call(Platform::VK, method, params, &config)
            .await
            .unwrap()
    })
}
pub struct Messages {}

impl Messages {
    pub async fn send(
        options: VKMessagesSendOptions,
        config: Arc<Config>,
    ) -> Result<Vec<VKMessagesSendResponse>, serde_json::Error> {
        let response = vk_api_call("messages.send", struct_to_vec(options), config)
            .await
            .unwrap();
        let response = response.get("response").unwrap();
        match response.as_i64() {
            Some(response) => Ok(vec![VKMessagesSendResponse {
                message_id: response,
                conversation_message_id: None,
                peer_id: None,
                error: None,
            }]),
            None => serde_json::from_value(response.clone()),
        }
    }
    pub async fn create_chat(
        options: VKMessagesCreateChatOptions,
        config: Arc<Config>,
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
        options: VKMessagesDeleteOptions,
        config: Arc<Config>,
    ) -> Result<Vec<VKMessagesDeleteResponse>, serde_json::Error> {
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
        options: VKMessagesDeleteChatPhotoOptions,
        config: Arc<Config>,
    ) -> Result<VKMessagesDeleteChatPhotoResponse, serde_json::Error> {
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
        options: VKMessagesDeleteConversationOptions,
        config: Arc<Config>,
    ) -> Result<VKMessagesDeleteConversationResponse, serde_json::Error> {
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
        options: VKMessagesDeleteReactionOptions,
        config: Arc<Config>,
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
        options: VKMessagesEditOptions,
        config: Arc<Config>,
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
        options: VKMessagesEditChatOptions,
        config: Arc<Config>,
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
        options: VKMessagesGetByConversationMessageIdOptions,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetByIdResponse, serde_json::Error> {
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
    pub async fn get_by_id(
        options: VKMessagesGetByIdOptions,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetByIdResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.getById", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_conversation_members(
        options: VKMessagesGetConversationMembers,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetConversationMembersResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call(
                "messages.getConversationMembers",
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
    pub async fn get_conversations(
        options: VKMessagesGetConversationsOptions,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetConversationsResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.getConversations", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_conversations_by_id(
        options: VKMessagesGetConversationsByIdOptions,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetConversationsByIdResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call(
                "messages.getConversationsById",
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
    pub async fn get_history(
        options: VKMessagesGetHistoryOptions,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetHistoryResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.getHistory", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_history_attachments(
        options: VKMessagesGetHistoryAttachmentsOptions,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetHistoryAttachmentsResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call(
                "messages.getHistoryAttachments",
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
    pub async fn get_important_messages(
        options: VKMessagesGetImportantMessages,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetImportantMessagesResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call(
                "messages.getImportantMessages",
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
    pub async fn get_intent_users(
        options: VKMessagesGetIntentUsers,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetIntentUsersResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.getIntentUsers", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_invite_link(
        options: VKMessagesGetInviteLink,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetInviteLinkResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.getInviteLink", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_messages_reactions(
        options: VKMessagesGetMessagesReactions,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetMessagesReactionsResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call(
                "messages.getMessagesReactions",
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
    pub async fn get_reacted_peers(
        options: VKMessagesGetReactedPeers,
        config: Arc<Config>,
    ) -> Result<VKMessagesGetReactedPeersResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.getReactedPeers", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn is_messages_from_group_allowed(
        options: VKMessagesIsMessagesFromGroupAllowed,
        config: Arc<Config>,
    ) -> Result<VKMessagesIsMessagesFromGroupAllowedResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call(
                "messages.isMessagesFromGroupAllowed",
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
    pub async fn mark_as_answered_conversation(
        options: VKMessagesMarkAsAnsweredConversation,
        config: Arc<Config>,
    ) {
        vk_api_call(
            "messages.markAsAnsweredConversation",
            struct_to_vec(options),
            config,
        )
        .await
        .unwrap();
    }
    pub async fn mark_as_important(
        options: VKMessagesMarkAsImportantOptions,
        config: Arc<Config>,
    ) -> Result<Vec<VKMessagesMarkAsImportantResponse>, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.markAsImportant", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn mark_as_important_conversation(
        options: VKMessagesMarkAsImportantConversationOptions,
        config: Arc<Config>,
    ) {
        vk_api_call(
            "messages.markAsImportantConversation",
            struct_to_vec(options),
            config,
        )
        .await
        .unwrap();
    }
    pub async fn mark_as_read(
        options: VKMessagesMarkAsRead,
        config: Arc<Config>,
    ) -> Result<i8, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.markAsRead", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn pin(
        options: VKMessagesPin,
        config: Arc<Config>,
    ) -> Result<VKMessage, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.pin", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn remove_chat_user(options: VKMessagesRemoveChatUser, config: Arc<Config>) {
        vk_api_call("messages.removeChatUser", struct_to_vec(options), config)
            .await
            .unwrap();
    }
    pub async fn restore(options: VKMessagesRestore, config: Arc<Config>) {
        vk_api_call("messages.restore", struct_to_vec(options), config)
            .await
            .unwrap();
    }
    pub async fn search(
        options: VKMessagesSearch,
        config: Arc<Config>,
    ) -> Result<VKMessagesSearchResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call("messages.search", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("response")
                .unwrap()
                .clone(),
        )
    }
    pub async fn search_conversations(
        options: VKMessagesSearchConversations,
        config: Arc<Config>,
    ) -> Result<VKMessagesSearchConversationsResponse, serde_json::Error> {
        serde_json::from_value(
            vk_api_call(
                "messages.searchConversations",
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
    pub async fn send_message_event_answer(
        options: VKMessagesSendMessageEventAnswer,
        config: Arc<Config>,
    ) {
        vk_api_call(
            "messages.sendMessageEventAnswer",
            struct_to_vec(options),
            config,
        )
        .await
        .unwrap();
    }
    pub async fn send_reaction(options: VKMessagesSendReaction, config: Arc<Config>) {
        vk_api_call("messages.sendReaction", struct_to_vec(options), config)
            .await
            .unwrap();
    }
    pub async fn set_activity(options: VKMessagesSetActivity, config: Arc<Config>) {
        vk_api_call("messages.setActivity", struct_to_vec(options), config)
            .await
            .unwrap();
    }
    pub async fn unpin(options: VKMessagesUnpin, config: Arc<Config>) {
        vk_api_call("messages.unpin", struct_to_vec(options), config)
            .await
            .unwrap();
    }
    pub async fn set_chat_photo_file(
        options: VKMessagesSetChatPhoto,
        photo: File,
        config: Arc<Config>,
    ) {
        let resp = api_call(
            Platform::VK,
            "photos.getChatUploadServer",
            struct_to_vec(options),
            &config,
        )
        .await
        .unwrap();
        let val: VKGetUploadServerResponse = from_value(resp).unwrap();
        let upload_url = val.response.upload_url;
        let server_resp = files_request(&upload_url, &[photo], None, Platform::VK)
            .await
            .unwrap();
        let server_resp: VKMessageChatPhotoUploaded = serde_json::from_str(&server_resp).unwrap();
        vk_api_call(
            "messages.setChatPhoto",
            struct_to_vec(VKMessagesSetChatPhotoFile {
                file: server_resp.file,
            }),
            config,
        );
    }
    pub async fn set_chat_photo(
        options: VKMessagesSetChatPhoto,
        photo_url: String,
        config: Arc<Config>,
    ) {
        let attachments = download_files(vec![Attachment {
            url: photo_url,
            ftype: FileType::Photo,
        }])
        .await;
        Self::set_chat_photo_file(options, attachments[0].clone(), config).await;
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesSetChatPhoto {
    pub chat_id: i64,
    pub crop_x: Option<i64>,
    pub crop_y: Option<i64>,
    pub crop_width: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesSetChatPhotoFile {
    pub file: String,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessageChatPhotoUploaded {
    pub file: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesSendOptions {
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
pub struct VKMessagesSendResponse {
    pub peer_id: Option<i64>,
    pub message_id: i64,
    pub conversation_message_id: Option<i64>,
    pub error: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesCreateChatOptions {
    pub user_ids: Option<String>,
    pub title: Option<String>,
    pub group_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesDeleteOptions {
    pub message_ids: Option<String>,
    pub spam: Option<bool>,
    pub group_id: Option<i64>,
    pub delete_for_all: Option<bool>,
    pub reason: Option<i64>,
    pub peer_id: Option<i64>,
    pub cmids: Option<String>,
}
#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesDeleteResponse {
    pub peer_id: Option<i64>,
    pub message_id: Option<i64>,
    pub conversation_message_id: Option<i64>,
    pub response: i8,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesDeleteChatPhotoOptions {
    pub chat_id: i64,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesDeleteChatPhotoResponse {
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
pub struct VKMessagesDeleteConversationOptions {
    pub user_id: Option<i64>,
    pub peer_id: Option<i64>,
    pub group_id: Option<i64>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesDeleteConversationResponse {
    pub peer_id: Option<i64>,
    pub last_deleted_id: i64,
    pub response: Option<i8>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesDeleteReactionOptions {
    pub peer_id: i64,
    pub cmid: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesEditOptions {
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
pub struct VKMessagesEditChatOptions {
    pub chat_id: i64,
    pub title: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetByConversationMessageIdOptions {
    pub peer_id: i64,
    pub conversation_message_ids: String,
    pub extended: Option<bool>,
    pub fields: Option<String>,
    pub group_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetByIdResponse {
    pub count: i32,
    pub items: Vec<VKMessage>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetByIdOptions {
    pub message_ids: String,
    pub preview_length: Option<i64>,
    pub extended: Option<bool>,
    pub fields: Option<String>,
    pub group_id: Option<i64>,
    pub cmids: Option<i64>,
    pub peer_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetConversationMembers {
    pub peer_id: i64,
    pub offset: Option<i64>,
    pub count: Option<i64>,
    pub fields: Option<String>,
    pub extended: Option<bool>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetConversationMembersResponse {
    pub count: i64,
    pub items: Vec<VKMessagesGetConversationMembersResponseItem>,
    pub chat_restrictions: Option<VKChatRestrictions>,
    pub profiles: Option<Vec<VKProfile>>,
    pub groups: Option<Vec<VKGroup>>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetConversationMembersResponseItem {
    pub member_id: i64,
    pub invited_by: i64,
    pub join_date: i64,
    pub is_admin: bool,
    pub can_kick: bool,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKChatRestrictions {
    pub admins_promote_users: bool,
    pub only_admins_edit_info: bool,
    pub only_admins_edit_pin: bool,
    pub only_admins_invite: bool,
    pub only_admins_kick: bool,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetConversationsOptions {
    pub offset: Option<i64>,
    pub count: Option<i64>,
    pub filter: Option<String>,
    pub extended: Option<bool>,
    pub start_message_id: Option<i64>,
    pub fields: Option<String>,
    pub group_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetConversationsResponse {
    pub count: i64,
    pub items: Vec<VKMessagesGetConversationsResponseItem>,
    pub profiles: Option<Vec<VKProfile>>,
    pub groups: Option<Vec<VKGroup>>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetConversationsResponseItem {
    pub conversation: VKConversation,
    pub last_message: VKMessage,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetConversationsByIdOptions {
    pub peer_ids: String,
    pub extended: Option<bool>,
    pub fields: Option<String>,
    pub group_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetConversationsByIdResponse {
    pub count: i64,
    pub items: Vec<VKConversation>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetHistoryOptions {
    pub offset: Option<i64>,
    pub count: Option<i64>,
    pub user_id: Option<i64>,
    pub peer_id: Option<i64>,
    pub start_message_id: Option<i64>,
    pub rev: Option<i64>,
    pub extended: Option<bool>,
    pub fields: Option<String>,
    pub group_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetHistoryResponse {
    pub count: i64,
    pub items: Vec<VKMessage>,
    pub profiles: Option<Vec<VKProfile>>,
    pub groups: Option<Vec<VKGroup>>,
    pub skipped: Option<VKMessagesSkipped>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesSkipped {
    pub count: i64,
    pub items: Vec<VKMessage>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetHistoryAttachmentsOptions {
    pub peer_id: i64,
    pub media_type: String,
    pub start_from: Option<String>,
    pub count: Option<i64>,
    pub photo_sizes: Option<bool>,
    pub fields: Option<String>,
    pub group_id: Option<i64>,
    pub cmid: Option<i64>,
    pub attachment_position: Option<i64>,
    pub offset: Option<i64>,
    pub preserve_order: Option<bool>,
    pub attachment_types: Option<String>,
    pub max_forwards_level: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetHistoryAttachmentsResponse {
    pub items: Vec<VKMessagesGetHistoryAttachmentsResponseItem>,
    pub next_from: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetHistoryAttachmentsResponseItem {
    pub message_id: i64,
    pub attachment: VKAttachment,
    pub conversation_message_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetImportantMessages {
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub start_message_id: Option<i64>,
    pub preview_length: Option<i64>,
    pub extended: Option<bool>,
    pub fields: Option<String>,
    pub group_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetImportantMessagesResponse {
    pub messages: Vec<VKMessage>,
    pub profiles: Option<Vec<VKProfile>>,
    pub groups: Option<Vec<VKGroup>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetIntentUsers {
    pub intent: String,
    pub subscribe_id: i64,
    pub offset: Option<i64>,
    pub count: Option<i64>,
    pub extended: Option<bool>,
    pub fields: Option<String>,
    pub group_id: Option<i64>,
    pub name_case: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetIntentUsersResponse {
    pub count: i64,
    pub items: Vec<i64>,
    pub profiles: Option<Vec<VKProfile>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetInviteLink {
    pub peer_id: i64,
    pub reset: Option<bool>,
    pub group_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetInviteLinkResponse {
    pub link: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetMessagesReactions {
    pub peer_id: i64,
    pub cmids: i64,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetMessagesReactionsResponse {
    pub count: i64,
    pub items: Vec<VKMessagesGetMessagesReactionsResponseItem>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetMessagesReactionsResponseItem {
    pub reaction: String,
    pub users: Vec<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesGetReactedPeers {
    pub peer_id: i64,
    pub cmid: i64,
    pub reaction_id: Option<i32>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesGetReactedPeersResponse {
    pub count: i64,
    pub items: Vec<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesIsMessagesFromGroupAllowed {
    pub group_id: i64,
    pub user_id: i64,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesIsMessagesFromGroupAllowedResponse {
    pub is_allowed: i8,
    pub is_allowed_in_pm: i8,
    pub reason: i8,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesMarkAsAnsweredConversation {
    pub peer_id: i64,
    pub answered: bool,
    pub group_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesMarkAsImportantOptions {
    pub message_ids: String,
    pub important: Option<bool>,
    pub cmids: Option<String>,
    pub group_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesMarkAsImportantResponse {
    pub message_id: Option<i64>,
    pub peer_id: Option<i64>,
    pub cmids: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesMarkAsImportantConversationOptions {
    pub peer_id: i64,
    pub important: Option<bool>,
    pub group_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesMarkAsRead {
    pub message_ids: String,
    pub peer_id: Option<i64>,
    pub start_message_id: Option<i64>,
    pub group_id: Option<i64>,
    pub mark_conversation_as_read: Option<bool>,
    pub up_to_cmid: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesPin {
    pub peer_id: i64,
    pub message_id: Option<i64>,
    pub conversation_message_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesRemoveChatUser {
    pub chat_id: i64,
    pub user_id: i64,
    pub member_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesRestore {
    pub message_id: i64,
    pub group_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesSearch {
    pub q: String,
    pub peer_id: Option<i64>,
    pub date: Option<i64>,
    pub preview_length: Option<i64>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
    pub extended: Option<bool>,
    pub fields: Option<String>,
    pub group_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesSearchResponse {
    pub count: i64,
    pub items: Vec<VKMessage>,
    pub profiles: Option<Vec<VKProfile>>,
    pub groups: Option<Vec<VKGroup>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesSearchConversations {
    pub q: String,
    pub count: Option<i64>,
    pub extended: Option<bool>,
    pub fields: Option<String>,
    pub group_id: Option<i64>,
}

#[derive(Deserialize, Clone, Debug, Default, Serialize)]
pub struct VKMessagesSearchConversationsResponse {
    pub count: i64,
    pub items: Vec<VKConversation>,
    pub profiles: Option<Vec<VKProfile>>,
    pub groups: Option<Vec<VKGroup>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesSendMessageEventAnswer {
    pub user_id: i64,
    pub peer_id: i64,
    pub event_id: String,
    pub event_data: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesSendReaction {
    pub peer_id: i64,
    pub cmid: i64,
    pub reaction_id: i32,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesSetActivity {
    pub user_id: Option<i64>,
    pub r#type: String,
    pub peer_id: Option<i64>,
    pub group_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct VKMessagesUnpin {
    pub peer_id: i64,
    pub group_id: Option<i64>,
}
// TODO: Chat photo
