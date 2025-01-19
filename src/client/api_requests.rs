use std::borrow::Cow;

use log::debug;
use serde_json::Value;

use crate::structs::{context::Platform, struct_to_vec::param};

use super::*;
/// Send request to VK or Telegram API
/// # Arguments
/// * `platform` - Platform to send request to
/// * `method` - Method to call
/// * `params` - Parameters to send
/// * `config` - Config to use
///
/// # Returns
/// * `Result<Value, String>` - Response from API
///
/// # Examples
/// ```no_run
/// use vtg::client::api_requests::api_call;
/// use vtg::structs::{context::Platform, struct_to_vec::param};
///
/// let response = api_call(Platform::VK, "messages.send", vec![param("peer_id", "1"), param("message", "Hello, world!")], &config).await;
/// match response {
///   Ok(response) => {
///      println!("Response: {}", response);
///   }
///   Err(e) => {
///      println!("Error: {}", e);
///   }
///}
/// ```
pub async fn api_call(
    platform: Platform,
    method: &str,
    mut params: Vec<(Cow<'_, str>, Cow<'_, str>)>,
    config: &Config,
) -> Result<Value, String> {
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
    if platform == Platform::VK {
        params.push(param("v", &config.vk_api_version));
    }
    let response = request(&url, &access_token, params).await;
    match response {
        Ok(response_text) => {
            debug!("API call response text: {}", response_text);
            let response_json: serde_json::Value =
                serde_json::from_str(&response_text).map_err(|e| e.to_string())?;
            match platform {
                Platform::VK => {
                    if let Some(error) = response_json.get("error") {
                        let error_msg = error["error_msg"].as_str().unwrap_or("Unknown error");
                        Err(error_msg.to_string())
                    } else {
                        Ok(response_json)
                    }
                }
                Platform::Telegram => {
                    if let Some(ok) = response_json.get("ok") {
                        if ok.as_bool().unwrap_or(false) {
                            Ok(response_json)
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
