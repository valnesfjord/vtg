use serde::de::DeserializeOwned;
use serde_json::Value;
use std::any::Any;
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use crate::client::requests::File;
use crate::structs::keyboard::{self, Keyboard};

use crate::client::api_requests::api_call;
use crate::upload::{
    download_files, send_tg_attachment_files, send_tg_attachments, upload_vk_attachments,
    Attachment,
};

use super::config::Config;
use super::struct_to_vec::{param, struct_to_vec};
use super::tg_api::TGSendMessageOptions;
use super::vk_api::VKMessagesSendOptions;

/// Unified context for working with both VK and Telegram
/// Context is a struct that contains all the information about the event that happened in the chat
/// # Fields
/// * `text` - Text of the message
/// * `from_id` - ID of the user who sent the message
/// * `peer_id` - ID of the chat where the message was sent
/// * `id` - ID of the message
/// * `type` - Type of the event
/// * `platform` - Platform where the event was received
/// * `data` - Data to store
/// * `event` - Event data
/// * `attachments` - Attachments of the message
/// * `config` - Config to use
#[derive(Debug, Clone)]
pub struct UnifyedContext {
    pub text: String,
    pub from_id: i64,
    pub peer_id: i64,
    pub id: i64,
    pub r#type: EventType,
    pub platform: Platform,
    pub data: Arc<Mutex<Box<dyn Any + Send + Sync>>>,
    pub event: String,
    pub attachments: String,
    pub config: Arc<Config>,
}

/// Platform enum
///
/// # Variants
/// * `VK` - VK platform
/// * `Telegram` - Telegram platform
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Platform {
    #[default]
    VK,
    Telegram,
}

/// Event type enum
///
/// # Variants
/// * `MessageNew` - New message event
/// * `MessageEdit` - Message edit event
/// * `InlineQuery` - Inline query event
/// * `ChosenInlineResult` - Chosen inline result event
/// * `CallbackQuery` - Callback query event
/// * `Unknown` - Unknown event
#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    MessageNew,
    MessageEdit,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    Unknown,
}

/// Unify context trait
/// # Methods
/// * `unify` - Unify context
pub trait UnifyContext {
    fn unify(&self, config: Arc<Config>) -> UnifyedContext;
}
/// Options to send message
/// # Fields
/// * `vk` - VK options
/// * `tg` - Telegram options
#[derive(Clone, Debug)]
pub struct SendOptions {
    pub vk: VKMessagesSendOptions,
    pub tg: TGSendMessageOptions,
}

/// Message builder to send message
/// # Fields
/// * `message` - Message text
/// * `chat_id` - Chat ID
/// * `config` - Config to use
/// * `platform` - Platform to send message to
/// * `vk_options` - VK options
/// * `tg_options` - Telegram options
/// * `keyboard` - Keyboard to send
/// * `attachments` - Attachments to send
/// * `files` - Files to send
/// * `parse_mode` - Parse mode to use (Telegram)
#[derive(Clone, Debug, Default)]
pub struct MessageBuilder {
    pub message: String,
    pub chat_id: i64,
    pub config: Arc<Config>,
    pub platform: Platform,
    pub vk_options: Option<VKMessagesSendOptions>,
    pub tg_options: Option<TGSendMessageOptions>,
    pub keyboard: Option<Keyboard>,
    pub attachments: Option<Vec<Attachment>>,
    pub files: Option<Vec<File>>,
    pub parse_mode: Option<String>,
}

impl MessageBuilder {
    /// Set vk options for message
    /// # Arguments
    /// * `options` - VK options    
    pub fn vk_options(self, options: VKMessagesSendOptions) -> MessageBuilder {
        MessageBuilder {
            vk_options: Some(options),
            ..self
        }
    }
    /// Set telegram options for message
    /// # Arguments
    /// * `options` - Telegram options
    pub fn tg_options(self, options: TGSendMessageOptions) -> MessageBuilder {
        MessageBuilder {
            tg_options: Some(options),
            ..self
        }
    }
    /// Set keyboard for message
    /// # Arguments
    /// * `keyboard` - Keyboard to send
    /// # Keyboard button variants
    /// You can see keyboard button variants [here](https://docs.rs/vtg/latest/vtg/structs/keyboard/enum.KeyboardButton.html)
    pub fn keyboard(self, keyboard: Keyboard) -> MessageBuilder {
        MessageBuilder {
            keyboard: Some(keyboard),
            ..self
        }
    }
    /// Set attachments for message
    /// # Arguments
    /// * `attachments` - Attachments to send
    pub fn attachments(self, attachments: Vec<Attachment>) -> MessageBuilder {
        MessageBuilder {
            attachments: Some(attachments),
            ..self
        }
    }
    /// Set files for message
    /// # Arguments
    /// * `files` - Files to send
    pub fn files(self, files: Vec<File>) -> MessageBuilder {
        MessageBuilder {
            files: Some(files),
            ..self
        }
    }
    /// Set parse mode for message (for Telegram)
    /// # Arguments
    /// * `parse_mode` - Parse mode to use
    pub fn parse_mode(self, parse_mode: &str) -> MessageBuilder {
        MessageBuilder {
            parse_mode: Some(parse_mode.to_owned()),
            ..self
        }
    }
    /// Send message
    /// # Examples
    /// ```
    /// ctx.message("Привет")
    ///    .keyboard(vtg::structs::keyboard::Keyboard::new(
    ///        vec![vec![KeyboardButton::Text {
    ///            color: Color::Positive,
    ///            label: "Привет".to_string(),
    ///            data: Some(to_value("{\"text\": \"hello\"}".to_string()).unwrap()),
    ///        }]],
    ///        true,
    ///        false,
    ///        &ctx.platform,
    ///    ))
    ///    .vk_options(vk_api::VKMessagesSendOptions {
    ///        disable_mentions: Some(true),
    ///        ..Default::default()
    ///    })
    ///    .tg_options(tg_api::TGSendMessageOptions {
    ///        disable_notification: Some(true),
    ///        ..Default::default()
    ///    })
    ///    .send()
    ///    .await;
    ///```
    pub async fn send(self) {
        let peer_id = self.chat_id;
        let config = self.config.clone();
        match self.platform {
            Platform::VK => {
                let attachments = self.make_vk_attachments(config.clone(), peer_id).await;
                let vk_options = self.vk_options.unwrap_or_default();
                let keyboard = self.keyboard;
                let attachment = attachments.unwrap_or("".to_string());
                tokio::task::spawn(async move {
                    let mut vk = struct_to_vec(vk_options.clone());
                    if vk_options.message.is_none() || vk_options.message.unwrap().is_empty() {
                        vk.push(param("message", self.message));
                    }
                    if vk_options.peer_id.is_none() || vk_options.peer_id.unwrap() == 0 {
                        vk.push(param("peer_id", peer_id.to_string()));
                    }
                    vk.push(param("random_id", "0"));
                    let j;
                    if keyboard.is_some() && vk_options.keyboard.is_none() {
                        j = serde_json::to_string(&keyboard.unwrap().vk_buttons).unwrap();
                        vk.push(param("keyboard", j));
                    }
                    if !attachment.is_empty() {
                        vk.push(param("attachment", attachment));
                    }
                    api_call(Platform::VK, "messages.send", vk, &config.clone())
                        .await
                        .unwrap()
                });
            }
            Platform::Telegram => {
                let tg_options = self.tg_options.unwrap_or_default();
                let keyboard = self.keyboard;
                let parse_mode = self.parse_mode.unwrap_or("HTML".to_string());
                let attachments = self.attachments.unwrap_or_default();
                let files = self.files.unwrap_or_default();
                tokio::task::spawn(async move {
                    let mut tg = struct_to_vec(tg_options.clone());
                    if tg_options.text.is_none() || tg_options.text.unwrap().is_empty() {
                        tg.push(param("text", self.message.clone()));
                    }
                    if tg_options.chat_id.is_none() || tg_options.chat_id.unwrap() == 0 {
                        tg.push(param("chat_id", peer_id.to_string()));
                    }
                    let j: String;
                    if keyboard.is_some() && tg_options.reply_markup.is_none() {
                        let keyboard = keyboard.unwrap();
                        j = if !keyboard.inline {
                            serde_json::to_string(&keyboard::ReplyKeyboardMarkup {
                                keyboard: keyboard.tg_buttons.unwrap(),
                                one_time_keyboard: keyboard.one_time,
                            })
                            .unwrap()
                        } else {
                            serde_json::to_string(&keyboard::InlineKeyboardMarkup {
                                inline_keyboard: keyboard.tg_buttons.unwrap(),
                            })
                            .unwrap()
                        };
                        tg.push(param("reply_markup", j));
                    }
                    if parse_mode != "HTML" {
                        tg.push(param("parse_mode", parse_mode));
                    }
                    if attachments.is_empty() && files.is_empty() {
                        api_call(Platform::Telegram, "sendMessage", tg, &config.clone())
                            .await
                            .unwrap();
                        return;
                    }
                    if attachments.is_empty() {
                        send_tg_attachments(attachments, &config, peer_id, &self.message).await;
                        return;
                    }
                    send_tg_attachment_files(files, &config, peer_id, &self.message).await;
                });
            }
        }
    }
    async fn make_vk_attachments(&self, config: Arc<Config>, peer_id: i64) -> Option<String> {
        let attachments = self.attachments.clone().unwrap_or_default();
        let files = self.files.clone().unwrap_or_default();
        if attachments.is_empty() && files.is_empty() {
            return None;
        }
        if !attachments.is_empty() {
            let attachments = download_files(attachments).await;
            return Some(
                upload_vk_attachments(attachments, &config, peer_id)
                    .await
                    .unwrap(),
            );
        }
        Some(
            upload_vk_attachments(files, &config, peer_id)
                .await
                .unwrap(),
        )
    }
}

impl UnifyedContext {
    /// Create a message builder to send message, may be slower than ctx.send (work in progress)
    ///
    /// # Arguments
    /// * `message` - Message text
    ///
    /// # Examples
    ///```
    ///ctx.message("пива бы.")
    ///   .keyboard(vtg::structs::keyboard::Keyboard::new(
    ///       vec![vec![KeyboardButton::Text {
    ///           color: Color::Positive,
    ///           label: "Посмотреть".to_string(),
    ///           data: Some(to_value("{\"text\": \"hello\"}".to_string()).unwrap()),
    ///       }]],
    ///       true,
    ///       false,
    ///       &ctx.platform,
    ///   ))
    ///   .vk_options(vk_api::VKMessagesSendOptions {
    ///       disable_mentions: Some(true),
    ///       ..Default::default()
    ///   })
    ///   .tg_options(tg_api::TGSendMessageOptions {
    ///       disable_notification: Some(true),
    ///       ..Default::default()
    ///   })
    ///   .send()
    ///   .await;
    ///```
    pub fn message(&self, message: &str) -> MessageBuilder {
        MessageBuilder {
            message: message.to_owned(),
            chat_id: self.peer_id,
            config: self.config.clone(),
            platform: self.platform.clone(),
            ..Default::default()
        }
    }
    /// Send a message
    /// # Arguments
    /// * `message` - Message text
    /// # Examples
    /// ```
    /// ctx.send("Hello, world!");
    /// ```
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
                            param("peer_id", peer_id),
                            param("message", message_str),
                            param("random_id", "0"),
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
                        vec![param("chat_id", peer_id), param("text", message_str)],
                        &config,
                    )
                    .await
                    .unwrap()
                });
            }
        }
    }
    /// Send a message with HTML (for Telegram)
    /// # Arguments
    /// * `message` - Message text
    /// # Examples
    /// ```
    /// ctx.send_with_html("<b>Hello, world!</b>");
    /// ```
    pub fn send_with_html(&self, message: &str) {
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
                            param("peer_id", peer_id),
                            param("message", message_str),
                            param("random_id", "0"),
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
                            param("chat_id", peer_id),
                            param("text", message_str),
                            param("parse_mode", "HTML"),
                        ],
                        &config,
                    )
                    .await
                    .unwrap()
                });
            }
        }
    }

    /// Send a message with keyboard
    /// # Arguments
    /// * `message` - Message text
    /// * `keyboard` - Keyboard to send
    ///
    /// # Keyboard button variants
    /// You can see keyboard button variants [here](https://docs.rs/vtg/latest/vtg/structs/keyboard/enum.KeyboardButton.html)
    ///  
    /// # Examples
    /// ```
    /// use vtg::structs::keyboard::{Keyboard, KeyboardButton, Color};
    /// ctx.send_with_keyboard("Hello, world!", Keyboard::new(vec![vec![KeyboardButton::Text {
    ///   color: Color::Positive,
    ///   label: "Посмотреть".to_string(),
    ///   data: Some(to_value("{\"text\": \"hello\"}".to_string()).unwrap()),
    /// }]], true, false, &ctx.platform));
    /// ```
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
                            param("peer_id", peer_id),
                            param("message", message_str),
                            param("random_id", "0"),
                            param("keyboard", j),
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
                        keyboard: keyboard.tg_buttons.unwrap(),
                        one_time_keyboard: keyboard.one_time,
                    })
                    .unwrap()
                } else {
                    serde_json::to_string(&keyboard::InlineKeyboardMarkup {
                        inline_keyboard: keyboard.tg_buttons.unwrap(),
                    })
                    .unwrap()
                };
                tokio::task::spawn(async move {
                    api_call(
                        Platform::Telegram,
                        "sendMessage",
                        vec![
                            param("chat_id", peer_id),
                            param("text", message_str),
                            param("reply_markup", j),
                            param("parse_mode", "HTML"),
                        ],
                        &config,
                    )
                    .await
                    .unwrap()
                });
            }
        }
    }
    /// Send a message with options
    ///
    /// # Arguments
    /// * `message` - Message text
    /// * `options` - Options to send
    ///
    /// # Examples
    /// ```
    /// use vtg::structs::context::SendOptions;
    /// use vtg::vk_api::VKMessagesSendOptions;
    /// use vtg::tg_api::TGSendMessageOptions;
    /// ctx.send_with_options(
    ///    "@valnesfjord @cyournamec",
    ///    SendOptions {
    ///        vk: VKMessagesSendOptions {
    ///            disable_mentions: Some(true),
    ///            peer_id: Some(ctx.peer_id),
    ///            ..Default::default()
    ///        },
    ///        tg: TGSendMessageOptions {
    ///            disable_notification: Some(true),
    ///            chat_id: Some(ctx.peer_id),
    ///            ..Default::default()
    ///        },
    ///    },
    ///);
    ///```
    pub fn send_with_options(&self, message: &'static str, options: SendOptions) {
        let config = self.config.clone();
        match self.platform {
            Platform::VK => {
                let mut vk = struct_to_vec(options.vk);
                if !vk.contains(&param("message", message)) {
                    vk.push(param("message", message));
                }
                vk.push(param("random_id", "0"));
                tokio::task::spawn(async move {
                    api_call(Platform::VK, "messages.send", vk, &config)
                        .await
                        .unwrap()
                });
            }
            Platform::Telegram => {
                let mut tg = struct_to_vec(options.tg);
                if !tg.contains(&param("text", message)) {
                    tg.push(param("text", message));
                }
                tokio::task::spawn(async move {
                    api_call(Platform::Telegram, "sendMessage", tg, &config)
                        .await
                        .unwrap()
                });
            }
        }
    }
    /// Send a message with attachments
    /// # Arguments
    /// * `message` - Message text
    /// * `attachments` - Files to send
    /// # Examples
    /// ```
    ///use vtg::client::requests::{File, FileType};
    ///ctx.send_attachment_files(
    ///    "пива бы",
    ///    vec![File {
    ///        filename: "pivo.jpg".to_string(),
    ///        content: tokio::fs::read("C:\\Projects\\RustProjects\\vtg\\examples\\pivo2.jpg")
    ///            .await
    ///            .unwrap(),
    ///        ftype: FileType::Photo,
    ///    }],
    ///).await;
    /// ```
    ///
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
                            param("peer_id", peer_id.to_string()),
                            param("message", &message_str),
                            param("random_id", "0"),
                            param(
                                "attachment",
                                upload_vk_attachments(attachments, &config, peer_id)
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
    /// Send a message with attachments
    /// # Arguments
    /// * `message` - Message text
    /// * `attachments` - Attachments to send
    /// # Examples
    /// ```
    ///use vtg::upload::Attachment;
    ///ctx.send_attachments(
    ///        "attachments test",
    ///        vec![Attachment {
    ///            url:
    ///                "https://sn-gazeta.ru/wp-content/uploads/2023/04/tapeta-piwo-w-kuflu-i-szklance.jpg"
    ///                    .to_string(),
    ///            ftype: FileType::Photo,
    ///        },
    ///        Attachment {
    ///            url:
    ///                "https://w.forfun.com/fetch/a9/a908815bda3f615bfe16bef28c6389db.jpeg"
    ///                    .to_string(),
    ///            ftype: FileType::Photo,
    ///        },
    ///        ],
    ///    )
    ///    .await;
    /// ```
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
                            param("peer_id", peer_id.to_string()),
                            param("message", &message_str),
                            param("random_id", "0"),
                            param(
                                "attachment",
                                upload_vk_attachments(attachments, &config, peer_id)
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
    /// Call any VK or Telegram API method
    ///
    /// # Arguments
    /// * `platform` - Platform to send request to
    /// * `method` - Request method
    /// * `params` - Request params
    pub async fn api_call(
        &self,
        platform: Platform,
        method: &str,
        params: Vec<(Cow<'_, str>, Cow<'_, str>)>,
    ) -> Value {
        api_call(platform, method, params, &self.config)
            .await
            .unwrap()
    }
    /// Set data to context
    /// # Arguments
    /// * `data` - Data to set
    /// # Examples
    /// ```
    /// ctx.set_data("Hello, world!");
    /// ```
    pub fn set_data<T: Any + Send + Sync>(&self, data: T) {
        let mut data_to_edit = self.data.lock().unwrap();
        *data_to_edit = Box::new(data);
    }
    /// Get data from context
    /// # Examples
    /// ```
    /// let data = ctx.get_data::<String>().unwrap();
    /// ```
    pub fn get_data<T: Any + Send + Sync + Clone>(&self) -> Option<T> {
        let data = self.data.lock().unwrap();
        data.downcast_ref::<T>().cloned()
    }
    /// Get event from context
    ///
    /// # Examples
    /// ```
    /// use vtg::structs::vk::VKMessageNew;
    /// use vtg::structs::tg::TGMessage;
    /// if ctx.r#type == EventType::MessageNew {
    ///    match ctx.platform {
    ///       Platform::Telegram => {
    ///            let event = ctx.get_event::<TGMessage>().unwrap();
    ///            println!("{:?}", event);
    ///        }
    ///        Platform::VK => {
    ///            let event = ctx.get_event::<VKMessageNew>().unwrap();
    ///            println!("{:?}", event);
    ///        }
    ///    }
    ///}
    /// ```
    pub fn get_event<T: DeserializeOwned>(&self) -> Option<T> {
        serde_json::from_str(&self.event).ok()
    }
    /// Get attachments from context
    ///
    /// # Examples
    /// ```
    /// use vtg::structs::vk_attachments::VKAttachment;
    /// use vtg::structs::tg_attachments::TGAttachment;
    ///   match ctx.platform {
    ///      Platform::Telegram => {
    ///           let attachment = ctx.get_attachments::<TGAttachment>().unwrap();
    ///           println!("{:?}", event);
    ///       }
    ///       Platform::VK => {
    ///           let attachment = ctx.get_attachments::<VKAttachment>().unwrap();
    ///           println!("{:?}", event);
    ///       }
    ///   }
    /// ```
    pub fn get_attachments<T: DeserializeOwned>(&self) -> Option<Vec<T>> {
        serde_json::from_str(&self.attachments).ok()
    }
}
