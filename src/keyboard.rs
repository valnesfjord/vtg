use serde_json::Value;

use crate::structs::keyboard::{
    Keyboard, KeyboardButton, KeyboardButtonAction, TGKeyboardButton, VKKeyboard, VKKeyboardButton,
};

impl Keyboard {
    pub fn new(buttons: Vec<Vec<KeyboardButton>>, inline: bool, one_time: Option<bool>) -> Self {
        Self {
            inline,
            one_time,
            vk_buttons: VKKeyboard {
                one_time: one_time.unwrap_or(false),
                inline,
                buttons: buttons
                    .iter()
                    .map(|a| {
                        a.iter()
                            .map(|b| VKKeyboardButton {
                                action: KeyboardButtonAction {
                                    r#type: "text".to_string(),
                                    payload: Some(
                                        serde_json::from_str::<Value>(
                                            b.data.clone().unwrap().as_str(),
                                        )
                                        .unwrap(),
                                    ),
                                    label: Some(b.text.clone()),
                                },
                                color: Some(b.color.clone().as_string()),
                            })
                            .collect()
                    })
                    .collect(),
            },
            tg_buttons: buttons
                .iter()
                .map(|a| {
                    a.iter()
                        .map(|b| TGKeyboardButton {
                            text: b.text.clone(),
                            callback_data: b.data.clone(),
                            url: b.url.clone(),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}
