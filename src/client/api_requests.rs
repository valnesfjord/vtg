use std::collections::HashMap;

use super::*;
pub enum ApiResponse {
    VkResponse(serde_json::Value),
    TelegramResponse(serde_json::Value),
    Error(String),
}

pub async fn api_call(
    platform: Platform,
    method: String,
    params: HashMap<&str, &str>,
    config: &Config,
) -> Result<ApiResponse, String> {
    let url = match platform {
        Platform::VK => format!("https://api.vk.com/method/{}", method),
        Platform::Telegram => format!(
            "https://api.telegram.org/{}/{}",
            config.tg_access_token, method
        ),
    };
    let access_token = match platform {
        Platform::VK => config.vk_access_token.clone(),
        Platform::Telegram => "".to_owned(),
    };
    let response = request(url, access_token, params).await;
    match response {
        Ok(response_text) => {
            let response_json: serde_json::Value =
                serde_json::from_str(&response_text).map_err(|e| e.to_string())?;
            match platform {
                Platform::VK => {
                    if let Some(error) = response_json.get("error") {
                        let error_msg = error["error_msg"].as_str().unwrap_or("Unknown error");
                        Err(error_msg.to_string())
                    } else {
                        Ok(ApiResponse::VkResponse(response_json))
                    }
                }
                Platform::Telegram => {
                    if let Some(ok) = response_json.get("ok") {
                        if ok.as_bool().unwrap_or(false) {
                            Ok(ApiResponse::TelegramResponse(response_json))
                        } else {
                            let error_msg = response_json["description"]
                                .as_str()
                                .unwrap_or("Unknown error");
                            Err(error_msg.to_string())
                        }
                    } else {
                        Err("Invalid response from Telegram API".to_string())
                    }
                }
            }
        }
        Err(_) => Err("Error while sending request".to_string()),
    }
}
