use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::client::api_requests::api_call;

use super::{config::Config, context::Platform, struct_to_vec::struct_to_vec, tg::TGMessageEntity};

pub struct Api {}
impl Api {
    pub fn send_message(options: TGSendMessageOptions, config: Config) {
        tokio::task::spawn(async move {
            api_call(
                Platform::Telegram,
                "sendMessage",
                struct_to_vec(options),
                &config,
            )
            .await
            .unwrap()
        });
    }
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
}
