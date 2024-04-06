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
///    tg_access_token: "TG_ACCESS_TOKEN".to_string(),
///    ..Default::default()
/// };
/// ```
/// ```
/// use vtg::structs::config::Config;
/// use vtg::structs::config::CallbackSettings;
/// let config = Config {
///    vk_access_token: "VK_ACCESS_TOKEN".to_string(),
///    vk_group_id: 123456789,
///    tg_access_token: "TG_ACCESS_TOKEN".to_string(),
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
