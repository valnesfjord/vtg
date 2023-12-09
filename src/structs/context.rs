use serde::Deserialize;
use serde_json::Value;
use std::any::Any;
use std::sync::{Arc, Mutex};

use crate::client::requests::File;
use crate::structs::keyboard::{self, Keyboard};

use crate::client::api_requests::api_call;
use crate::upload::{
    download_files, send_tg_attachment_files, send_tg_attachments, upload_vk_attachments,
    Attachment,
};

use super::config::Config;
use super::struct_to_vec::struct_to_vec;
use super::tg_api::TGSendMessageOptions;
use super::vk_api::VKMessageSendOptions;

#[derive(Debug, Clone)]
pub struct UnifyedContext {
    pub text: String,
    pub from_id: i64,
    pub peer_id: i64,
    pub id: i64,
    pub r#type: EventType,
    pub platform: Platform,
    pub data: Arc<Mutex<Box<dyn Any + Send + Sync>>>,
    pub event: Arc<Mutex<Box<dyn Any + Send + Sync>>>,
    pub attachments: Arc<Mutex<Vec<Box<dyn Any + Send + Sync>>>>,
    pub config: Config,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    VK,
    Telegram,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    MessageNew,
    MessageEdit,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    Unknown,
}

pub trait UnifyContext {
    fn unify(&self, config: &Config) -> UnifyedContext;
}
#[derive(Deserialize, Clone, Copy)]
pub struct VKNewMessageResponse {
    pub response: i64,
}

#[derive(Clone, Debug)]
pub struct SendOptions {
    pub vk: VKMessageSendOptions,
    pub tg: TGSendMessageOptions,
}

impl UnifyedContext {
    pub fn send(&self, message: &str) {
        let peer_id = self.peer_id.to_string();
        let config = self.config.clone();
        let message_str = message.to_owned();
        match self.platform {
            Platform::VK => {
                tokio::task::spawn(async move {
                    api_call(
                        Platform::VK,
                        "messages.send",
                        vec![
                            ("peer_id", peer_id.as_str()),
                            ("message", message_str.as_str()),
                            ("random_id", "0"),
                            ("v", "5.199"),
                        ],
                        &config,
                    )
                    .await
                    .unwrap()
                });
            }
            Platform::Telegram => {
                tokio::task::spawn(async move {
                    api_call(
                        Platform::Telegram,
                        "sendMessage",
                        vec![
                            ("chat_id", peer_id.as_str()),
                            ("text", message_str.as_str()),
                        ],
                        &config,
                    )
                    .await
                    .unwrap()
                });
            }
        }
    }
    pub fn send_with_keyboard(&self, message: &str, keyboard: Keyboard) {
        let peer_id = self.peer_id.to_string();
        let config = self.config.clone();
        let message_str = message.to_owned();
        match self.platform {
            Platform::VK => {
                let j = serde_json::to_string(&keyboard.vk_buttons).unwrap();
                tokio::task::spawn(async move {
                    api_call(
                        Platform::VK,
                        "messages.send",
                        vec![
                            ("peer_id", peer_id.as_str()),
                            ("message", message_str.as_str()),
                            ("random_id", "0"),
                            ("v", "5.199"),
                            ("keyboard", j.as_str()),
                        ],
                        &config,
                    )
                    .await
                    .unwrap()
                });
            }
            Platform::Telegram => {
                let j: String = if !keyboard.inline {
                    serde_json::to_string(&keyboard::ReplyKeyboardMarkup {
                        keyboard: keyboard.tg_buttons,
                        one_time_keyboard: keyboard.one_time.unwrap(),
                    })
                    .unwrap()
                } else {
                    serde_json::to_string(&keyboard::InlineKeyboardMarkup {
                        inline_keyboard: keyboard.tg_buttons,
                    })
                    .unwrap()
                };
                tokio::task::spawn(async move {
                    api_call(
                        Platform::Telegram,
                        "sendMessage",
                        vec![
                            ("chat_id", peer_id.as_str()),
                            ("text", message_str.as_str()),
                            ("reply_markup", j.as_str()),
                        ],
                        &config,
                    )
                    .await
                    .unwrap()
                });
            }
        }
    }
    pub fn send_with_options(&self, message: &'static str, options: SendOptions) {
        println!("{:?}", options);
        let config = self.config.clone();
        match self.platform {
            Platform::VK => {
                let mut vk = struct_to_vec(options.vk);
                if !vk.contains(&("message", message)) {
                    vk.push(("message", message));
                }
                vk.push(("random_id", "0"));
                vk.push(("v", "5.199"));
                tokio::task::spawn(async move {
                    api_call(Platform::VK, "messages.send", vk, &config)
                        .await
                        .unwrap()
                });
            }
            Platform::Telegram => {
                let mut tg = struct_to_vec(options.tg);
                if !tg.contains(&("text", message)) {
                    tg.push(("text", message));
                }
                tokio::task::spawn(async move {
                    api_call(Platform::Telegram, "sendMessage", tg, &config)
                        .await
                        .unwrap()
                });
            }
        }
    }
    pub async fn send_attachment_files(&self, message: &str, attachments: Vec<File>) {
        let peer_id = self.peer_id;
        let config = self.config.clone();
        let message_str = message.to_owned();
        match self.platform {
            Platform::VK => {
                tokio::task::spawn(async move {
                    api_call(
                        Platform::VK,
                        "messages.send",
                        vec![
                            ("peer_id", &peer_id.to_string()),
                            ("message", &message_str),
                            ("random_id", "0"),
                            ("v", "5.199"),
                            (
                                "attachment",
                                &upload_vk_attachments(attachments, &config, peer_id)
                                    .await
                                    .unwrap(),
                            ),
                        ],
                        &config,
                    )
                    .await
                    .unwrap();
                });
            }
            Platform::Telegram => {
                tokio::task::spawn(async move {
                    send_tg_attachment_files(attachments, &config, peer_id, message_str.as_str())
                        .await;
                });
            }
        }
    }
    pub async fn send_attachments(&self, message: &str, attachments: Vec<Attachment>) {
        let peer_id = self.peer_id;
        let config = self.config.clone();
        let message_str = message.to_owned();
        match self.platform {
            Platform::VK => {
                tokio::task::spawn(async move {
                    let attachments = download_files(attachments).await;
                    api_call(
                        Platform::VK,
                        "messages.send",
                        vec![
                            ("peer_id", &peer_id.to_string()),
                            ("message", &message_str),
                            ("random_id", "0"),
                            ("v", "5.199"),
                            (
                                "attachment",
                                &upload_vk_attachments(attachments, &config, peer_id)
                                    .await
                                    .unwrap(),
                            ),
                        ],
                        &config,
                    )
                    .await
                    .unwrap();
                });
            }
            Platform::Telegram => {
                tokio::task::spawn(async move {
                    send_tg_attachments(attachments, &config, peer_id, message_str.as_str()).await;
                });
            }
        }
    }
    pub async fn api_call(
        &self,
        platform: Platform,
        method: &str,
        params: Vec<(&str, &str)>,
    ) -> Value {
        api_call(platform, method, params, &self.config)
            .await
            .unwrap()
    }
    pub fn set_data<T: Any + Send + Sync>(&self, data: T) {
        let mut data_to_edit = self.data.lock().unwrap();
        *data_to_edit = Box::new(data);
    }
    pub fn get_data<T: Any + Send + Sync + Clone>(&self) -> Option<T> {
        let data = self.data.lock().unwrap();
        data.downcast_ref::<T>().cloned()
    }
    pub fn get_event<T: Any + Send + Sync + Clone>(&self) -> Option<T> {
        let event = self.event.lock().unwrap();
        event.downcast_ref::<T>().cloned()
    }
    pub fn get_attachments<T: Any + Send + Sync + Clone>(&self) -> Option<Vec<T>> {
        let attachments = self.attachments.lock().unwrap();
        let result: Option<Vec<T>> = attachments
            .iter()
            .map(|attachment| attachment.downcast_ref::<T>().cloned())
            .collect();
        result
    }
}
