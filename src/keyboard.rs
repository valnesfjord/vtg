use crate::structs::{
    context::Platform,
    keyboard::{
        Keyboard, KeyboardButton, KeyboardButtonAction, TGKeyboardButton, VKKeyboard,
        VKKeyboardButton,
    },
};

impl Keyboard {
    pub fn new(
        buttons: Vec<Vec<KeyboardButton>>,
        inline: bool,
        one_time: bool,
        platform: &Platform,
    ) -> Self {
        let mut keyboard: Self = Self {
            inline,
            one_time,
            vk_buttons: None,
            tg_buttons: None,
        };
        match platform {
            Platform::VK => {
                keyboard.vk_buttons = Some(VKKeyboard {
                    one_time,
                    inline,
                    buttons: buttons
                        .iter()
                        .map(|a| {
                            a.iter()
                                .map(|b| match b {
                                    KeyboardButton::Text { label, color, data } => {
                                        VKKeyboardButton {
                                            action: KeyboardButtonAction {
                                                r#type: "text".to_string(),
                                                label: Some(label.clone()),
                                                payload: data.clone(),
                                                ..Default::default()
                                            },
                                            color: Some(color.as_string()),
                                        }
                                    }
                                    KeyboardButton::Url {
                                        link,
                                        label,
                                        data,
                                        color,
                                    } => VKKeyboardButton {
                                        action: KeyboardButtonAction {
                                            r#type: "open_link".to_string(),
                                            link: Some(link.clone()),
                                            label: Some(label.clone()),
                                            payload: data.clone(),
                                            ..Default::default()
                                        },
                                        color: Some(color.as_string()),
                                    },
                                    KeyboardButton::Location { label, data, color } => {
                                        VKKeyboardButton {
                                            action: KeyboardButtonAction {
                                                r#type: "location".to_string(),
                                                label: Some(label.clone()),
                                                payload: data.clone(),
                                                ..Default::default()
                                            },
                                            color: Some(color.as_string()),
                                        }
                                    }
                                    KeyboardButton::VKPay {
                                        label,
                                        hash,
                                        data,
                                        color,
                                    } => VKKeyboardButton {
                                        action: KeyboardButtonAction {
                                            r#type: "vkpay".to_string(),
                                            hash: Some(hash.clone()),
                                            label: Some(label.clone()),
                                            payload: data.clone(),
                                            ..Default::default()
                                        },
                                        color: Some(color.as_string()),
                                    },
                                    KeyboardButton::VKApp {
                                        label,
                                        app_id,
                                        owner_id,
                                        hash,
                                        data,
                                        color,
                                    } => VKKeyboardButton {
                                        action: KeyboardButtonAction {
                                            r#type: "open_app".to_string(),
                                            app_id: Some(*app_id),
                                            owner_id: Some(*owner_id),
                                            hash: Some(hash.clone()),
                                            label: Some(label.clone()),
                                            payload: data.clone(),
                                            ..Default::default()
                                        },
                                        color: Some(color.as_string()),
                                    },
                                    KeyboardButton::Callback { label, data, color } => {
                                        VKKeyboardButton {
                                            action: KeyboardButtonAction {
                                                r#type: "callback".to_string(),
                                                label: Some(label.clone()),
                                                payload: data.clone(),
                                                ..Default::default()
                                            },
                                            color: Some(color.as_string()),
                                        }
                                    }
                                    KeyboardButton::TGInline {
                                        text,
                                        callback_data,
                                        ..
                                    } => VKKeyboardButton {
                                        action: KeyboardButtonAction {
                                            r#type: "text".to_string(),
                                            label: Some(text.clone()),
                                            payload: callback_data.clone(),
                                            ..Default::default()
                                        },
                                        color: Some("primary".to_string()),
                                    },
                                    KeyboardButton::TGKeyboardButton { text, .. } => {
                                        VKKeyboardButton {
                                            action: KeyboardButtonAction {
                                                r#type: "text".to_string(),
                                                label: Some(text.clone()),
                                                payload: None,
                                                ..Default::default()
                                            },
                                            color: Some("primary".to_string()),
                                        }
                                    }
                                    _ => VKKeyboardButton {
                                        action: KeyboardButtonAction {
                                            r#type: "text".to_string(),
                                            label: Some("".to_string()),
                                            payload: None,
                                            ..Default::default()
                                        },
                                        color: Some("primary".to_string()),
                                    },
                                })
                                .collect()
                        })
                        .collect(),
                })
            }
            Platform::Telegram => {
                keyboard.tg_buttons = Some(
                    buttons
                        .iter()
                        .map(|a| {
                            a.iter()
                                .map(|b| match b {
                                    KeyboardButton::Text { label, data, .. } => TGKeyboardButton {
                                        text: label.clone(),
                                        callback_data: Some(
                                            serde_json::to_string(&data.clone().unwrap_or(
                                                serde_json::Value::String("".to_string()),
                                            ))
                                            .unwrap(),
                                        ),
                                        ..Default::default()
                                    },
                                    KeyboardButton::Url {
                                        link, label, data, ..
                                    } => TGKeyboardButton {
                                        text: label.clone(),
                                        url: Some(link.clone()),
                                        callback_data: Some(
                                            serde_json::to_string(&data.clone().unwrap_or(
                                                serde_json::Value::String("".to_string()),
                                            ))
                                            .unwrap(),
                                        ),
                                        ..Default::default()
                                    },
                                    KeyboardButton::Location { label, data, .. } => {
                                        TGKeyboardButton {
                                            text: label.clone(),
                                            callback_data: Some(
                                                serde_json::to_string(&data.clone().unwrap_or(
                                                    serde_json::Value::String("".to_string()),
                                                ))
                                                .unwrap(),
                                            ),
                                            ..Default::default()
                                        }
                                    }
                                    KeyboardButton::VKPay { label, data, .. } => TGKeyboardButton {
                                        text: label.clone(),
                                        callback_data: Some(
                                            serde_json::to_string(&data.clone().unwrap_or(
                                                serde_json::Value::String("".to_string()),
                                            ))
                                            .unwrap(),
                                        ),
                                        ..Default::default()
                                    },
                                    KeyboardButton::VKApp { label, data, .. } => TGKeyboardButton {
                                        text: label.clone(),
                                        callback_data: Some(
                                            serde_json::to_string(&data.clone().unwrap_or(
                                                serde_json::Value::String("".to_string()),
                                            ))
                                            .unwrap(),
                                        ),
                                        ..Default::default()
                                    },
                                    KeyboardButton::Callback { label, data, .. } => {
                                        TGKeyboardButton {
                                            text: label.clone(),
                                            callback_data: Some(
                                                serde_json::to_string(&data.clone().unwrap_or(
                                                    serde_json::Value::String("".to_string()),
                                                ))
                                                .unwrap(),
                                            ),
                                            ..Default::default()
                                        }
                                    }
                                    KeyboardButton::TGInline {
                                        text,
                                        url,
                                        callback_data,
                                        web_app,
                                        login,
                                        switch_inline_query,
                                        switch_inline_query_choosen_chat,
                                        switch_inline_query_current_chat,
                                        callback_game,
                                        pay,
                                    } => TGKeyboardButton {
                                        text: text.clone(),
                                        url: url.clone(),
                                        callback_data: Some(
                                            serde_json::to_string(
                                                &callback_data.clone().unwrap_or(
                                                    serde_json::Value::String("".to_string()),
                                                ),
                                            )
                                            .unwrap(),
                                        ),
                                        web_app: web_app.clone(),
                                        login: login.clone(),
                                        switch_inline_query: switch_inline_query.clone(),
                                        switch_inline_query_current_chat:
                                            switch_inline_query_current_chat.clone(),
                                        switch_inline_query_choosen_chat:
                                            switch_inline_query_choosen_chat.clone(),
                                        callback_game: callback_game.clone(),
                                        pay: *pay,
                                        ..Default::default()
                                    },
                                    KeyboardButton::TGKeyboardButton {
                                        text,
                                        request_chat,
                                        request_contact,
                                        request_location,
                                        request_poll,
                                        request_users,
                                        web_app,
                                    } => TGKeyboardButton {
                                        text: text.clone(),
                                        request_chat: request_chat.clone(),
                                        request_contact: *request_contact,
                                        request_location: *request_location,
                                        request_poll: request_poll.clone(),
                                        request_users: request_users.clone(),
                                        web_app: web_app.clone(),
                                        ..Default::default()
                                    },
                                    KeyboardButton::TGKeyboardRemove {
                                        remove_keyboard,
                                        selective,
                                    } => TGKeyboardButton {
                                        remove_keyboard: Some(*remove_keyboard),
                                        selective: *selective,
                                        ..Default::default()
                                    },
                                    KeyboardButton::TGForceReply {
                                        force_reply,
                                        selective,
                                        input_field_placeholder,
                                    } => TGKeyboardButton {
                                        force_reply: Some(*force_reply),
                                        selective: *selective,
                                        input_field_placeholder: input_field_placeholder.clone(),
                                        ..Default::default()
                                    },
                                })
                                .collect()
                        })
                        .collect(),
                )
            }
        }
        keyboard
    }
}
