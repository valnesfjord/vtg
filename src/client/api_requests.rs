use std::collections::HashMap;

use super::*;

pub async fn api_call(
    platform: Platform,
    method: String,
    params: HashMap<&str, &str>,
    config: &Config,
) -> String {
    let url = match platform {
        Platform::VK => format!("https://api.vk.com/method/{}", method),
        Platform::Telegram => format!(
            "https://api.telegram.org/bot{}/{}",
            config.tg_access_token, method
        ),
    };
    let access_token = match platform {
        Platform::VK => config.vk_access_token.clone(),
        Platform::Telegram => "".to_owned(),
    };
    let response = request(url, access_token, params).await;
    response.unwrap_or("".to_string())
}
