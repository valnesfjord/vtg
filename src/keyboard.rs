use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Clone)]
pub struct Keyboard {
    pub inline: bool,
    pub one_time: Option<bool>,
    pub vk_buttons: VKKeyboard,
    pub tg_buttons: Vec<Vec<TGKeyboardButton>>,
}
#[derive(Serialize, Clone)]
pub struct VKKeyboard {
    pub one_time: bool,
    pub inline: bool,
    pub buttons: Vec<Vec<VKKeyboardButton>>,
}
#[derive(Serialize, Clone)]
pub struct KeyboardButton {
    pub text: String,
    pub color: Color,
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Clone)]
pub enum Color {
    Negative,
    Positive,
    Secondary,
    Primary,
}
impl Color {
    fn as_string(&self) -> String {
        match self {
            Color::Negative => "negative".to_string(),
            Color::Positive => "positive".to_string(),
            Color::Secondary => "secondary".to_string(),
            Color::Primary => "primary".to_string(),
        }
    }
}
impl Keyboard {
    pub fn new(buttons: Vec<Vec<KeyboardButton>>, inline: bool, one_time: Option<bool>) -> Self {
        if !inline && one_time.is_some() {
            return Self {
                inline,
                one_time,
                vk_buttons: VKKeyboard {
                    #[allow(clippy::unnecessary_unwrap)]
                    one_time: one_time.unwrap(),
                    inline: false,
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
                                callback_data: None,
                                url: None,
                            })
                            .collect()
                    })
                    .collect(),
            };
        };
        Self {
            inline,
            one_time,
            vk_buttons: VKKeyboard {
                one_time: false,
                inline: true,
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

#[derive(Serialize, Clone)]
pub struct TGKeyboardButton {
    pub text: String,
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Serialize, Clone)]
pub struct VKKeyboardButton {
    pub action: KeyboardButtonAction,
    pub color: Option<String>,
}
#[derive(Serialize, Clone)]
pub struct KeyboardButtonAction {
    pub r#type: String,
    pub payload: Option<Value>,
    pub label: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<TGKeyboardButton>>,
    pub one_time_keyboard: bool,
}

#[derive(Serialize, Clone)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<TGKeyboardButton>>,
}
