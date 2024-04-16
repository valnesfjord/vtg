/// CallbackSettings struct with the port, callback_url, secret and path.
///
///Note: callback_url don't need to have slash in the end, path must be without slash in start and end
///
/// # Examples
///
/// ```
///use vtg::structs::config::CallbackSettings;
///let callback_settings = CallbackSettings {
///  port: 1234,
///  callback_url: "https://valnesfjord.com".to_string(),
///  secret: "secret".to_string(),
///  path: "yourcallbacksecretpathwithoutslashinstartandend".to_string(),
///};
///```
#[derive(Debug, Clone, Default)]
pub struct CallbackSettings {
    pub port: u16,
    pub callback_url: String,
    pub secret: String,
    pub path: String,
}

/// Config struct with the VK and TG access tokens, VK group ID and VK API version.
///
///Note: If you use callback settings, callback_url don't need to have slash in the end, path must be without slash in start and end
///
/// # Examples
///
/// ```
/// use vtg::structs::config::Config;
///
/// let config = Config {
///    vk_access_token: "VK_ACCESS_TOKEN".to_string(),
///    vk_group_id: 123456789,
///    vk_api_version: "5.199".to_string(),
///    tg_access_token: "TG_ACCESS_TOKEN".to_string(), // token starts with "bot", like: bot1234567890:ABCDEFGHIJKL
///    ..Default::default()
/// };
/// ```
/// ```
/// use vtg::structs::config::Config;
/// use vtg::structs::config::CallbackSettings;
/// let config = Config {
///    vk_access_token: "VK_ACCESS_TOKEN".to_string(),
///    vk_group_id: 123456789,
///    tg_access_token: "TG_ACCESS_TOKEN".to_string(), // token starts with "bot", like: bot1234567890:ABCDEFGHIJKL
///    vk_api_version: "5.199".to_string(),
///    callback: Some(CallbackSettings {
///        port: 1234,
///        callback_url: "https://valnesfjord.com".to_string(),
///        secret: "secret".to_string(),
///        path: "yourcallbacksecretpathwithoutslashinstartandend".to_string(),
///    }),
/// };
///```
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub vk_access_token: String,
    pub vk_group_id: i64,
    pub vk_api_version: String,
    pub tg_access_token: String,
    pub callback: Option<CallbackSettings>,
}

impl Config {
    pub fn check(mut self) -> Self {
        if self.tg_access_token.is_empty() || self.vk_access_token.is_empty() {
            panic!("Telegram or VK access token is empty");
        }
        if !self.tg_access_token.starts_with("bot") {
            panic!("Telegram access token must starts with 'bot'");
        }
        if self.vk_group_id == 0 || self.vk_group_id.is_negative() {
            panic!("VK group ID is empty or invalid");
        }
        if self.vk_api_version.is_empty() {
            self.vk_api_version = "5.199".to_string();
        }
        if self.callback.is_some() {
            let callback = self.callback.as_ref().unwrap();
            if callback.port == 0 {
                panic!("Callback port is empty or invalid");
            }
            if callback.callback_url.is_empty() {
                panic!("Callback URL is empty");
            }
            if callback.secret.is_empty() {
                panic!("Callback secret is empty");
            }
            if callback.path.is_empty() {
                panic!("Callback path is empty");
            }
            if callback.path.starts_with('/') || callback.path.ends_with('/') {
                panic!("Callback path must not start or end with slash");
            }
        }
        self
    }
}
