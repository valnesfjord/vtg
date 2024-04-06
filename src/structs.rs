/// Module with the Config for connection to VK and Telegram.
pub mod config;

/// Context struct with the text, from_id, peer_id, id, r#type, platform, data, config, event and attachments.
///
/// Contains methods for sending messages, attachments, files and other.
pub mod context;

/// Structs for working with keyboards.
///
/// Contains structs for creating keyboards for VK and Telegram.
pub mod keyboard;

/// Middleware chain for processing messages.
///
/// Contains struct for middleware chain and functions for adding middleware.
pub mod middleware;

/// Structs for working with Telegram context/events.
///
/// Contains structs for working with Telegram context/events.
pub mod tg;

pub mod struct_to_vec;
/// Structs for working with Telegram API.
///
/// Contains structs for working with Telegram API.
pub mod tg_api;

/// Structs for working with Telegram attachments.
///
/// Contains structs for working with Telegram attachments.
pub mod tg_attachments;

/// Structs for working with uploads.
///
/// Contains structs for working with uploads.
pub mod upload;
/// Structs for working with VK context/events.
///
/// Contains structs for working with VK context/events.
pub mod vk;
/// Structs for working with VK API.
///
/// Contains structs for working with VK API.
pub mod vk_api;

/// Structs for working with VK attachments.
///
/// Contains structs for working with VK attachments.
pub mod vk_attachments;
