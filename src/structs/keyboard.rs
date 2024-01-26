use serde::Serialize;
use serde_json::Value;
use serde_with::skip_serializing_none;

use super::tg_api::TGChatAdministratorRights;

#[derive(Serialize, Clone, Debug)]
pub struct Keyboard {
    pub inline: bool,
    pub one_time: bool,
    pub vk_buttons: VKKeyboard,
    pub tg_buttons: Vec<Vec<TGKeyboardButton>>,
}
#[derive(Serialize, Clone, Debug)]
pub struct VKKeyboard {
    pub one_time: bool,
    pub inline: bool,
    pub buttons: Vec<Vec<VKKeyboardButton>>,
}
#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum KeyboardButton {
    Text {
        label: String,
        color: Color,
        data: Option<Value>,
    },
    Url {
        label: String,
        link: String,
        color: Color,
        data: Option<Value>,
    },
    Location {
        label: String,
        color: Color,
        data: Option<Value>,
    },
    VKPay {
        label: String,
        hash: String,
        color: Color,
        data: Option<Value>,
    },
    VKApp {
        label: String,
        app_id: u32,
        owner_id: u32,
        hash: String,
        color: Color,
        data: Option<Value>,
    },
    Callback {
        label: String,
        color: Color,
        data: Option<Value>,
    },
    TGInline {
        text: String,
        url: Option<String>,
        callback_data: Option<Value>,
        web_app: Option<TGInlineWebApp>,
        login: Option<TGInlineLogin>,
        switch_inline_query: Option<String>,
        switch_inline_query_current_chat: Option<String>,
        switch_inline_query_choosen_chat: Option<TGSwitchInlineQueryChoosenChat>,
        callback_game: Option<TGCallbackGame>,
        pay: Option<bool>,
    },
    TGKeyboardButton {
        text: String,
        request_contact: Option<bool>,
        request_location: Option<bool>,
        request_users: Option<TGKeyboardButtonRequestUsers>,
        request_chat: Option<TGKeyboardButtonRequestChat>,
        request_poll: Option<TGKeyboardButtonPollType>,
        web_app: Option<TGInlineWebApp>,
    },
    TGKeyboardRemove {
        remove_keyboard: bool,
        selective: Option<bool>,
    },
    TGForceReply {
        force_reply: bool,
        selective: Option<bool>,
        input_field_placeholder: Option<String>,
    },
}

#[derive(Serialize, Clone, Debug)]
pub struct TGInlineWebApp {
    pub url: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct TGInlineLogin {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>,
}

#[derive(Serialize, Clone, Debug)]
pub struct TGSwitchInlineQueryChoosenChat {
    pub query: Option<String>,
    pub allow_user_chats: Option<bool>,
    pub allow_bot_chats: Option<bool>,
    pub allow_group_chats: Option<bool>,
    pub allow_channel_chats: Option<bool>,
}

#[derive(Serialize, Clone, Debug)]
pub struct TGCallbackGame {
    pub user_id: Option<i64>,
    pub score: Option<i64>,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct TGKeyboardButtonRequestUsers {
    pub request_id: i64,
    pub user_is_bot: Option<bool>,
    pub user_is_premium: Option<bool>,
    pub max_quanity: Option<i8>,
}

#[derive(Serialize, Clone, Debug)]
pub struct TGKeyboardButtonRequestChat {
    pub request_id: i64,
    pub chat_is_channel: Option<bool>,
    pub chat_is_forum: Option<bool>,
    pub chat_has_username: Option<bool>,
    pub chat_is_created: Option<bool>,
    pub user_administrator_rights: Option<TGChatAdministratorRights>,
    pub bot_administrator_rights: Option<TGChatAdministratorRights>,
    pub bot_is_member: Option<bool>,
}

#[derive(Serialize, Clone, Debug)]
pub struct TGKeyboardButtonPollType {
    pub r#type: String,
}

#[derive(Serialize, Clone, Debug)]
pub enum Color {
    Negative,
    Positive,
    Secondary,
    Primary,
}
impl Color {
    pub fn as_string(&self) -> String {
        match self {
            Color::Negative => "negative".to_string(),
            Color::Positive => "positive".to_string(),
            Color::Secondary => "secondary".to_string(),
            Color::Primary => "primary".to_string(),
        }
    }
}
#[skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct TGKeyboardButton {
    pub text: String,
    pub callback_data: Option<Value>,
    pub url: Option<String>,
    pub web_app: Option<TGInlineWebApp>,
    pub login: Option<TGInlineLogin>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_choosen_chat: Option<TGSwitchInlineQueryChoosenChat>,
    pub callback_game: Option<TGCallbackGame>,
    pub pay: Option<bool>,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
    pub request_users: Option<TGKeyboardButtonRequestUsers>,
    pub request_chat: Option<TGKeyboardButtonRequestChat>,
    pub request_poll: Option<TGKeyboardButtonPollType>,
    pub remove_keyboard: Option<bool>,
    pub selective: Option<bool>,
    pub force_reply: Option<bool>,
    pub input_field_placeholder: Option<String>,
}
#[derive(Serialize, Clone, Debug)]
pub struct VKKeyboardButton {
    pub action: KeyboardButtonAction,
    pub color: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct KeyboardButtonAction {
    pub r#type: String,
    pub payload: Option<Value>,
    pub label: Option<String>,
    pub link: Option<String>,
    pub hash: Option<String>,
    pub app_id: Option<u32>,
    pub owner_id: Option<u32>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<TGKeyboardButton>>,
    pub one_time_keyboard: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<TGKeyboardButton>>,
}
