use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use tokio::task::JoinHandle;

use crate::client::requests::files_request;
use crate::client::{api_requests::api_call, requests::File};
use crate::upload::Attachment;

use super::{
    config::Config,
    context::Platform,
    struct_to_vec::struct_to_vec,
    tg::{TGChat, TGMessage, TGMessageEntity, TGUser},
    tg_attachments::TGPhotoSize,
};
pub fn tg_api_call(
    method: &'static str,
    params: Vec<(&'static str, &'static str)>,
    config: Arc<Config>,
) -> JoinHandle<Value> {
    tokio::task::spawn(async move {
        api_call(Platform::Telegram, method, params, &config)
            .await
            .unwrap()
    })
}

pub struct Api {}
impl Api {
    pub async fn send_message(
        options: TGSendMessageOptions,
        config: Arc<Config>,
    ) -> Result<TGMessage, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("sendMessage", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn forward_message(
        options: TGForwardMessageOptions,
        config: Arc<Config>,
    ) -> Result<TGMessage, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("forwardMessage", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn forwards_messages(
        options: TGForwardsMessagesOptions,
        config: Arc<Config>,
    ) -> Result<Vec<TGMessageId>, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("forwardsMessages", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn copy_message(
        options: TGCopyMessage,
        config: Arc<Config>,
    ) -> Result<TGMessageId, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("copyMessage", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn copy_messages(
        options: TGCopyMessages,
        config: Arc<Config>,
    ) -> Result<Vec<TGMessageId>, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("copyMessages", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn send_location(
        options: TGSendLocation,
        config: Arc<Config>,
    ) -> Result<TGMessage, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("sendLocation", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn send_venue(
        options: TGSendVenue,
        config: Arc<Config>,
    ) -> Result<TGMessage, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("sendVenue", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn send_contact(
        options: TGSendContact,
        config: Arc<Config>,
    ) -> Result<TGMessage, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("sendContact", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn send_poll(
        options: TGSendPoll,
        config: Arc<Config>,
    ) -> Result<TGMessage, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("sendPoll", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn send_dice(
        options: TGSendDice,
        config: Arc<Config>,
    ) -> Result<TGMessage, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("sendDice", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn send_chat_action(
        options: TGSendChatAction,
        config: Arc<Config>,
    ) -> Result<Option<bool>, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("sendChatAction", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_message_reaction(
        options: TGSetMessageReaction,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("setMessageReaction", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_user_profile_photos(
        options: TGGetUserProfilePhotos,
        config: Arc<Config>,
    ) -> Result<TGUserProfilePhotos, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getUserProfilePhotos", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_me(config: Arc<Config>) -> Result<TGUser, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getMe", vec![], config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_file(
        options: TGGetFile,
        config: Arc<Config>,
    ) -> Result<TGFile, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getFile", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn ban_chat_member(
        options: TGBanChatMember,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("banChatMember", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn unban_chat_member(
        options: TGUnbanChatMember,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("unbanChatMember", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn restrict_chat_member(
        options: TGRestrictChatMember,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("restrictChatMember", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn promote_chat_member(
        options: TGPromoteChatMember,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("promoteChatMember", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_chat_administrator_custom_title(
        options: TGSetChatAdministratorCustomTitle,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call(
                "setChatAdministratorCustomTitle",
                struct_to_vec(options),
                config,
            )
            .await
            .unwrap()
            .get("result")
            .unwrap()
            .clone(),
        )
    }
    pub async fn ban_chat_sender_chat(
        options: TGBanChatSenderChat,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("banChatSenderChat", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn unban_chat_sender_chat(
        options: TGUnbanChatSenderChat,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("unbanChatSenderChat", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_chat_permissions(
        options: TGSetChatPermissions,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("setChatPermissions", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn export_chat_invite_link(
        options: TGExportChatInviteLink,
        config: Arc<Config>,
    ) -> Result<String, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("exportChatInviteLink", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn create_chat_invite_link(
        options: TGCreateChatInviteLink,
        config: Arc<Config>,
    ) -> Result<TGChatInviteLink, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("createChatInviteLink", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn edit_chat_invite_link(
        options: TGEditChatInviteLink,
        config: Arc<Config>,
    ) -> Result<TGChatInviteLink, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("editChatInviteLink", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn revoke_chat_invite_link(
        options: TGRevokeChatInviteLink,
        config: Arc<Config>,
    ) -> Result<TGChatInviteLink, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("revokeChatInviteLink", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn aprove_chat_join_request(
        options: TGAproveChatJoinRequest,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("aproveChatJoinRequest", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn decline_chat_join_request(
        options: TGDeclineChatJoinRequest,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("declineChatJoinRequest", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn delete_chat_photo(
        options: TGDeleteChatPhoto,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("deleteChatPhoto", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_chat_title(
        options: TGSetChatTitle,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("setChatTitle", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_chat_description(
        options: TGSetChatDescription,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("setChatDescription", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn pin_chat_message(
        options: TGPinChatMessage,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("pinChatMessage", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn unpin_chat_message(
        options: TGUnpinChatMessage,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("unpinChatMessage", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn leave_chat(
        options: TGLeaveChat,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("leaveChat", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn unpin_all_chat_messages(
        options: TGUnpinAllChatMessages,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("unpinAllChatMessages", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_chat(
        options: TGGetChat,
        config: Arc<Config>,
    ) -> Result<TGChat, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getChat", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_chat_administrators(
        options: TGGetChatAdministrators,
        config: Arc<Config>,
    ) -> Result<Vec<TGChatMember>, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getChatAdministrators", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_chat_member_count(
        options: TGGetChatMemberCount,
        config: Arc<Config>,
    ) -> Result<i64, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getChatMemberCount", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_chat_member(
        options: TGGetChatMember,
        config: Arc<Config>,
    ) -> Result<TGChatMember, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getChatMember", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_chat_sticker_set(
        options: TGSetChatStickerSet,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("setChatStickerSet", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn delete_chat_sticker_set(
        options: TGDeleteChatStickerSet,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("deleteChatStickerSet", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_forum_topic_icon_stickers(
        config: Arc<Config>,
    ) -> Result<Vec<TGSticker>, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getForumTopicIconStickers", vec![], config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn create_forum_topic(
        options: TGForumTopic,
        config: Arc<Config>,
    ) -> Result<TGForumTopic, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("createForumTopic", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn edit_forum_topic(
        options: TGForumTopic,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("editForumTopic", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn close_forum_topic(
        options: TGForumTopic,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("closeForumTopic", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn reopen_forum_topic(
        options: TGForumTopic,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("reopenForumTopic", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn delete_forum_topic(
        options: TGForumTopic,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("deleteForumTopic", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn unpin_all_forum_topic_messages(
        options: TGForumTopic,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("unpinAllForumTopicMessages", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn edit_general_forum_topic(
        options: TGGeneralForumTopic,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("editGeneralForumTopic", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn close_general_forum_topic(
        options: TGGeneralForumTopic,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("closeGeneralForumTopic", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn reopen_general_forum_topic(
        options: TGGeneralForumTopic,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("reopenGeneralForumTopic", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn hide_general_forum_topic(
        options: TGGeneralForumTopic,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("hideGeneralForumTopic", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn unhide_general_forum_topic(
        options: TGGeneralForumTopic,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("unhideGeneralForumTopic", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn unpin_all_general_forum_topic_messages(
        options: TGGeneralForumTopic,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call(
                "unpinAllGeneralForumTopicMessages",
                struct_to_vec(options),
                config,
            )
            .await
            .unwrap()
            .get("result")
            .unwrap()
            .clone(),
        )
    }
    pub async fn answer_callback_query(
        options: TGAnswerCallbackQuery,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("answerCallbackQuery", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_user_chat_boosts(
        options: TGGetUserChatBoosts,
        config: Arc<Config>,
    ) -> Result<Vec<TGUserChatBoosts>, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getUserChatBoosts", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_my_commands(
        options: TGGetMyCommands,
        config: Arc<Config>,
    ) -> Result<Vec<TGBotCommand>, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getMyCommands", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_my_name(
        options: TGSetMyName,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("setMyName", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_my_name(config: Arc<Config>) -> Result<TGBotName, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getMyName", vec![], config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_my_description(
        options: TGSetMyDescription,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("setMyDescription", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_my_description(
        options: TGGetMyDescription,
        config: Arc<Config>,
    ) -> Result<TGBotDescription, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getMyDescription", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_my_short_description(
        options: TGSetMyShortDescription,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("setMyShortDescription", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_my_short_description(
        options: TGGetMyShortDescription,
        config: Arc<Config>,
    ) -> Result<TGBotShortDescription, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getMyShortDescription", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_chat_menu_button(
        options: TGSetChatMenuButton,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("setChatMenuButton", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn get_chat_menu_button(
        options: TGGetChatMenuButton,
        config: Arc<Config>,
    ) -> Result<TGMenuButton, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("getChatMenuButton", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_my_default_administrator_rights(
        options: TGSetMyDefaultAdministratorRights,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call(
                "setMyDefaultAdministratorRights",
                struct_to_vec(options),
                config,
            )
            .await
            .unwrap()
            .get("result")
            .unwrap()
            .clone(),
        )
    }
    pub async fn get_my_default_administrator_rights(
        options: TGGetMyDefaultAdministratorRights,
        config: Arc<Config>,
    ) -> Result<TGChatAdministratorRights, serde_json::Error> {
        serde_json::from_value(
            tg_api_call(
                "getMyDefaultAdministratorRights",
                struct_to_vec(options),
                config,
            )
            .await
            .unwrap()
            .get("result")
            .unwrap()
            .clone(),
        )
    }
    pub async fn answer_inline_query(
        options: TGAnswerInlineQuery,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("answerInlineQuery", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn answer_web_app_query(
        options: TGAnswerWebAppQuery,
        config: Arc<Config>,
    ) -> Result<TGSentWebAppMessage, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("answerWebAppQuery", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn edit_message_text(options: TGEditMessageText, config: Arc<Config>) -> TGMessage {
        serde_json::from_value(
            tg_api_call("editMessageText", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
        .unwrap_or(TGMessage {
            ..Default::default()
        })
    }
    pub async fn edit_message_caption(
        options: TGEditMessageCaption,
        config: Arc<Config>,
    ) -> TGMessage {
        serde_json::from_value(
            tg_api_call("editMessageCaption", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
        .unwrap_or(TGMessage {
            ..Default::default()
        })
    }
    pub async fn edit_message_live_location(
        options: TGEditMessageLiveLocation,
        config: Arc<Config>,
    ) -> TGMessage {
        serde_json::from_value(
            tg_api_call("editMessageLiveLocation", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
        .unwrap_or(TGMessage {
            ..Default::default()
        })
    }
    pub async fn stop_message_live_location(
        options: TGStopMessageLiveLocation,
        config: Arc<Config>,
    ) -> TGMessage {
        serde_json::from_value(
            tg_api_call("stopMessageLiveLocation", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
        .unwrap_or(TGMessage {
            ..Default::default()
        })
    }
    pub async fn edit_message_reply_markup(
        options: TGEditMessageReplyMarkup,
        config: Arc<Config>,
    ) -> TGMessage {
        serde_json::from_value(
            tg_api_call("editMessageReplyMarkup", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
        .unwrap_or(TGMessage {
            ..Default::default()
        })
    }
    pub async fn stop_poll(
        options: TGStopPoll,
        config: Arc<Config>,
    ) -> Result<TGPoll, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("stopPoll", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn delete_message(
        options: TGDeleteMessage,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("deleteMessage", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn delete_messages(
        options: TGDeleteMessages,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        serde_json::from_value(
            tg_api_call("deleteMessages", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub async fn set_chat_photo(
        photo: Attachment,
        chat_id: i64,
        config: Arc<Config>,
    ) -> Result<bool, serde_json::Error> {
        let options = TGSetChatPhoto {
            chat_id,
            photo: photo.url,
        };
        serde_json::from_value(
            tg_api_call("setChatPhoto", struct_to_vec(options), config)
                .await
                .unwrap()
                .get("result")
                .unwrap()
                .clone(),
        )
    }
    pub fn set_chat_photo_file(photo: File, chat_id: i64, config: Arc<Config>) {
        tokio::task::spawn(async move {
            files_request(
                &format!(
                    "https://api.telegram.org/{}/setChatPhoto",
                    config.tg_access_token,
                ),
                &vec![photo],
                Some(vec![("chat_id", &chat_id.to_string())]),
                Platform::Telegram,
            )
            .await
            .unwrap();
        });
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetChatPhoto {
    pub chat_id: i64,
    pub photo: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSendMessageOptions {
    pub chat_id: Option<i64>,
    pub text: Option<String>,
    pub parse_mode: Option<String>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<String>,
    pub allow_sending_without_reply: Option<bool>,
    pub entities: Option<Vec<TGMessageEntity>>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<TGReplyParameters>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGReplyParameters {
    pub selective: Option<bool>,
    pub force_reply: Option<bool>,
    pub input_field_placeholder: Option<String>,
    pub input_field_visibility: Option<String>,
    pub is_personal: Option<bool>,
    pub next_step_chat_member: Option<i64>,
    pub next_step_data: Option<String>,
    pub next_step_date: Option<i64>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>,
    pub switch_pm_text: Option<String>,
    pub switch_pm_parameter: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGForwardMessageOptions {
    pub chat_id: i64,
    pub from_chat_id: i64,
    pub disable_notification: Option<bool>,
    pub message_id: i64,
    pub message_thread_id: Option<i64>,
    pub protect_content: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGForwardsMessagesOptions {
    pub chat_id: i64,
    pub from_chat_id: i64,
    pub message_ids: Vec<i64>,
    pub disable_notification: Option<bool>,
    pub message_thread_id: Option<i64>,
    pub protect_content: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGMessageId {
    pub message_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGCopyMessage {
    pub chat_id: i64,
    pub from_chat_id: i64,
    pub message_id: i64,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<TGMessageEntity>>,
    pub disable_notification: Option<bool>,
    pub reply_parameters: Option<TGReplyParameters>,
    pub reply_markup: Option<String>,
    pub message_thread_id: Option<i64>,
    pub protect_content: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGCopyMessages {
    pub chat_id: i64,
    pub from_chat_id: i64,
    pub message_ids: Vec<i64>,
    pub message_thread_id: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub remove_caption: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSendLocation {
    pub chat_id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<String>,
    pub disable_notification: Option<bool>,
    pub message_thread_id: Option<i64>,
    pub protect_content: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSendVenue {
    pub chat_id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<String>,
    pub message_thread_id: Option<i64>,
    pub protect_content: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSendContact {
    pub chat_id: i64,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<String>,
    pub message_thread_id: Option<i64>,
    pub protect_content: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSendPoll {
    pub chat_id: i64,
    pub question: String,
    pub options: Vec<String>,
    pub is_anonymous: Option<bool>,
    pub type_: Option<String>,
    pub allows_multiple_answers: Option<bool>,
    pub correct_option_id: Option<i64>,
    pub explanation: Option<String>,
    pub explanation_parse_mode: Option<String>,
    pub explanation_entities: Option<Vec<TGMessageEntity>>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>,
    pub is_closed: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<String>,
    pub message_thread_id: Option<i64>,
    pub protect_content: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSendDice {
    pub chat_id: i64,
    pub emoji: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<String>,
    pub message_thread_id: Option<i64>,
    pub protect_content: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSendChatAction {
    pub chat_id: i64,
    pub action: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetMessageReaction {
    pub chat_id: i64,
    pub message_id: i64,
    pub reaction: Option<TGReactionType>,
    pub is_big: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum TGReactionType {
    ReactionTypeEmoji(Option<TGReactionTypeEmoji>),
    ReactionTypeCustomEmoji(Option<TGReactionTypeCustomEmoji>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGReactionTypeEmoji {
    pub emoji: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGReactionTypeCustomEmoji {
    pub custom_emoji_id: String,
    pub r#type: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetUserProfilePhotos {
    pub user_id: i64,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGUserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<TGPhotoSize>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetFile {
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<i64>,
    pub file_path: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGBanChatMember {
    pub chat_id: i64,
    pub user_id: i64,
    pub until_date: Option<i64>,
    pub revoke_messages: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGUnbanChatMember {
    pub chat_id: i64,
    pub user_id: i64,
    pub only_if_banned: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGRestrictChatMember {
    pub chat_id: i64,
    pub user_id: i64,
    pub permissions: TGChatPermissions,
    pub until_date: Option<i64>,
    pub use_independent_chat_permissions: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatPermissions {
    pub can_send_messages: Option<bool>,
    pub can_send_audios: Option<bool>,
    pub can_send_documents: Option<bool>,
    pub can_send_videos: Option<bool>,
    pub can_send_video_notes: Option<bool>,
    pub can_send_voice_notes: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_manage_topics: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGPromoteChatMember {
    pub chat_id: i64,
    pub user_id: i64,
    pub is_anonymous: Option<bool>,
    pub can_manage_chat: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_manage_voice_chats: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_post_stories: Option<bool>,
    pub can_edit_stories: Option<bool>,
    pub can_delete_stories: Option<bool>,
    pub can_manage_topics: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetChatAdministratorCustomTitle {
    pub chat_id: i64,
    pub user_id: i64,
    pub custom_title: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGBanChatSenderChat {
    pub chat_id: i64,
    pub sender_chat_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGUnbanChatSenderChat {
    pub chat_id: i64,
    pub sender_chat_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetChatPermissions {
    pub chat_id: i64,
    pub permissions: TGChatPermissions,
    pub use_independent_chat_permissions: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGExportChatInviteLink {
    pub chat_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGCreateChatInviteLink {
    pub chat_id: i64,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub name: Option<String>,
    pub creates_join_request: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGEditChatInviteLink {
    pub chat_id: i64,
    pub invite_link: String,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub name: Option<String>,
    pub creates_join_request: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatInviteLink {
    pub invite_link: String,
    pub creator: TGUser,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub member_count: Option<i64>,
    pub name: Option<String>,
    pub creates_join_request: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGRevokeChatInviteLink {
    pub chat_id: i64,
    pub invite_link: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGAproveChatJoinRequest {
    pub chat_id: i64,
    pub user_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGDeclineChatJoinRequest {
    pub chat_id: i64,
    pub user_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGDeleteChatPhoto {
    pub chat_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetChatTitle {
    pub chat_id: i64,
    pub title: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetChatDescription {
    pub chat_id: i64,
    pub description: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGPinChatMessage {
    pub chat_id: i64,
    pub message_id: i64,
    pub disable_notification: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGUnpinChatMessage {
    pub chat_id: i64,
    pub message_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGUnpinAllChatMessages {
    pub chat_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGLeaveChat {
    pub chat_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetChat {
    pub chat_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetChatAdministrators {
    pub chat_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum TGChatMember {
    ChatMemberOwner(TGChatMemberOwner),
    ChatMemberAdministrator(TGChatMemberAdministrator),
    ChatMemberMember(TGChatMemberMember),
    ChatMemberRestricted(TGChatMemberRestricted),
    ChatMemberLeft(TGChatMemberLeft),
    ChatMemberBanned(TGChatMemberBanned),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatMemberOwner {
    pub status: String,
    pub user: TGUser,
    pub is_anonymous: Option<bool>,
    pub custom_title: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatMemberAdministrator {
    pub status: String,
    pub user: TGUser,
    pub can_be_edited: Option<bool>,
    pub is_anonymous: Option<bool>,
    pub custom_title: Option<String>,
    pub can_manage_chat: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_manage_voice_chats: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_post_stories: Option<bool>,
    pub can_edit_stories: Option<bool>,
    pub can_delete_stories: Option<bool>,
    pub can_manage_topics: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatMemberMember {
    pub status: String,
    pub user: TGUser,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatMemberRestricted {
    pub status: String,
    pub user: TGUser,
    pub is_member: Option<bool>,
    pub can_send_messages: Option<bool>,
    pub can_send_audios: Option<bool>,
    pub can_send_documents: Option<bool>,
    pub can_send_videos: Option<bool>,
    pub can_send_video_notes: Option<bool>,
    pub can_send_voice_notes: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_manage_topics: Option<bool>,
    pub until_date: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatMemberLeft {
    pub status: String,
    pub user: TGUser,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatMemberBanned {
    pub status: String,
    pub user: TGUser,
    pub until_date: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetChatMemberCount {
    pub chat_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetChatMember {
    pub chat_id: i64,
    pub user_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetChatStickerSet {
    pub chat_id: i64,
    pub sticker_set_name: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGDeleteChatStickerSet {
    pub chat_id: i64,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSticker {
    pub premium_animation: TGFile,
    pub emoji: String,
    pub is_animated: bool,
    pub set_name: String,
    pub mask_position: Option<TGMaskPosition>,
    pub file_size: Option<i64>,
    pub file_id: Option<String>,
    pub file_unique_id: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub is_video: Option<bool>,
    pub thumbnail: Option<TGPhotoSize>,
    pub custom_emoji_id: Option<String>,
    pub needs_repainting: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGMaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGForumTopic {
    pub message_thread_id: i64,
    pub name: String,
    pub icon_color: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGeneralForumTopic {
    pub chat_id: i64,
    pub name: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGAnswerCallbackQuery {
    pub callback_query_id: String,
    pub text: Option<String>,
    pub show_alert: Option<bool>,
    pub url: Option<String>,
    pub cache_time: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetUserChatBoosts {
    pub chat_id: i64,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGUserChatBoosts {
    pub boosts: Vec<TGChatBoost>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGChatBoost {
    pub boost_id: String,
    pub add_date: i64,
    pub expiration_date: i64,
    pub source: TGChatBoostSource,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum TGChatBoostSource {
    ChatBoostSourcePremium(TGChatBoostSourcePremium),
    ChatBoostSourceGiftCode(TGChatBoostSourceGiftCode),
    ChatBoostSourceGiveaway(TGChatBoostSourceGiveaway),
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatBoostSourcePremium {
    pub source: String,
    pub user: TGUser,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatBoostSourceGiftCode {
    pub source: String,
    pub user: TGUser,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatBoostSourceGiveaway {
    pub source: String,
    pub user: TGUser,
    pub is_unclaimed: bool,
    pub giveaway_message_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetMyCommands {
    pub scope: Option<TGGetMyCommandsScope>,
    pub language_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetMyCommandsScope {
    pub chat_id: Option<i64>,
    pub user_id: Option<i64>,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGBotCommand {
    pub command: String,
    pub description: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetMyName {
    pub name: String,
    pub language_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetMyName {
    pub language_code: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGBotName {
    pub name: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetMyDescription {
    pub description: String,
    pub language_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetMyDescription {
    pub language_code: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGBotDescription {
    pub description: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetMyShortDescription {
    pub short_description: String,
    pub language_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetMyShortDescription {
    pub language_code: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGBotShortDescription {
    pub short_description: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetChatMenuButton {
    pub chat_id: i64,
    pub menu_button: TGMenuButton,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetChatMenuButton {
    pub chat_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGMenuButton {
    pub text: Option<String>,
    pub r#type: String,
    pub web_app: Option<TGWebApp>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGWebApp {
    pub url: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGGetMyDefaultAdministratorRights {
    pub for_channels: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSetMyDefaultAdministratorRights {
    pub rights: TGChatAdministratorRights,
    pub for_channels: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGChatAdministratorRights {
    pub is_anonymous: Option<bool>,
    pub can_manage_chat: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_manage_voice_chats: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_post_stories: Option<bool>,
    pub can_edit_stories: Option<bool>,
    pub can_delete_stories: Option<bool>,
    pub can_manage_topics: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGAnswerInlineQuery {
    pub inline_query_id: String,
    pub results: Vec<Value>,
    pub cache_time: Option<i64>,
    pub is_personal: Option<bool>,
    pub next_offset: Option<String>,
    pub button: Option<TGInlineQueryResultsButton>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGInlineQueryResultsButton {
    pub text: String,
    pub web_app: Option<TGWebApp>,
    pub start_parameter: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGAnswerWebAppQuery {
    pub web_app_query_id: String,
    pub result: Value,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGSentWebAppMessage {
    pub inline_message_id: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGEditMessageText {
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub text: String,
    pub parse_mode: Option<String>,
    pub disable_web_page_preview: Option<bool>,
    pub reply_markup: Option<String>,
    pub entities: Option<Vec<TGMessageEntity>>,
    pub link_preview_options: Option<TGLinkPreviewOptions>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGLinkPreviewOptions {
    pub is_disabled: Option<bool>,
    pub url: Option<String>,
    pub prefer_small_media: Option<bool>,
    pub prefer_large_media: Option<bool>,
    pub show_above_text: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGEditMessageCaption {
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<TGMessageEntity>>,
    pub reply_markup: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGEditMessageLiveLocation {
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
    pub reply_markup: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGStopMessageLiveLocation {
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGEditMessageReplyMarkup {
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGStopPoll {
    pub chat_id: i64,
    pub message_id: i64,
    pub reply_markup: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGDeleteMessage {
    pub chat_id: i64,
    pub message_id: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGDeleteMessages {
    pub chat_id: i64,
    pub message_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGPoll {
    pub id: String,
    pub question: String,
    pub options: Vec<TGPollOption>,
    pub total_voter_count: i64,
    pub is_closed: bool,
    pub is_anonymous: bool,
    pub r#type: String,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<i64>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<TGMessageEntity>>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TGPollOption {
    pub text: String,
    pub voter_count: i64,
}
