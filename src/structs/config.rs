#[derive(Debug, Clone, Default)]
pub struct CallbackSettings {
    pub port: u16,
    pub callback_url: String,
    pub secret: String,
    pub path: String,
}
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub vk_access_token: String,
    pub vk_group_id: i64,
    pub vk_api_version: String,
    pub tg_access_token: String,
    pub callback: Option<CallbackSettings>,
}