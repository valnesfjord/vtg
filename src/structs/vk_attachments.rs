use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use super::vk::VKMessage;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKAttachment {
    pub r#type: String,
    pub photo: Option<PhotoAttachment>,
    pub video: Option<VideoAttachment>,
    pub audio: Option<AudioAttachment>,
    pub doc: Option<DocAttachment>,
    pub link: Option<LinkAttachment>,
    pub sticker: Option<StickerAttachment>,
    pub wall: Option<WallAttachment>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PhotoAttachment {
    pub id: i64,
    pub album_id: i64,
    pub owner_id: i64,
    pub user_id: i64,
    pub text: String,
    pub sizes: Vec<VKPhotoSizes>,
    pub date: i64,
    pub access_key: Option<String>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKPhotoSizes {
    pub r#type: String,
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VideoAttachment {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
    pub description: String,
    pub duration: i64,
    pub image: Vec<VKVideoImage>,
    pub first_frame: Vec<VKVideoFirstFrame>,
    pub date: i64,
    pub adding_date: i64,
    pub views: i64,
    pub comments: i64,
    pub player: String,
    pub platform: Option<String>,
    pub can_edit: Option<i8>,
    pub can_add: Option<i8>,
    pub is_private: Option<i8>,
    pub access_key: Option<String>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKVideoImage {
    pub with_padding: Option<i8>,
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKVideoFirstFrame {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AudioAttachment {
    pub id: i64,
    pub owner_id: i64,
    pub artist: String,
    pub title: String,
    pub duration: i64,
    pub url: String,
    pub lyrics_id: Option<i64>,
    pub album_id: Option<i64>,
    pub genre_id: Option<i64>,
    pub date: i64,
    pub access_key: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DocAttachment {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
    pub size: i64,
    pub ext: String,
    pub url: String,
    pub date: i64,
    pub r#type: i64,
    pub access_key: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LinkAttachment {
    pub url: String,
    pub title: String,
    pub caption: String,
    pub description: String,
    pub photo: Option<PhotoAttachment>,
    pub is_favorite: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct StickerAttachment {
    pub product_id: i64,
    pub sticker_id: i64,
    pub images: Vec<VKStickerImage>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKStickerImage {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WallAttachment {
    pub id: i64,
    pub from_id: i64,
    pub to_id: i64,
    pub date: i64,
    pub post_type: String,
    pub text: String,
    pub attachments: Vec<VKAttachment>,
    pub comments: Comments,
    pub likes: Likes,
    pub reposts: Reposts,
    pub views: Views,
    pub is_favorite: bool,
    pub short_text_rate: Option<f64>,
    pub copy_history: Option<Vec<WallAttachment>>,
    pub can_edit: Option<i8>,
    pub created_by: Option<i64>,
    pub can_delete: Option<i8>,
    pub can_pin: Option<i8>,
    pub is_pinned: Option<i8>,
    pub marked_as_ads: Option<i8>,
    pub postponed_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Comments {
    pub count: i64,
    pub can_post: i8,
    pub groups_can_post: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Likes {
    pub count: i64,
    pub user_likes: i8,
    pub can_like: i8,
    pub can_publish: i8,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Reposts {
    pub count: i64,
    pub user_reposted: i8,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Views {
    pub count: i64,
}

pub fn unify_attachments(
    message: Option<VKMessage>,
) -> Arc<Mutex<Vec<Box<dyn Any + Send + Sync>>>> {
    if message.is_none() {
        return Arc::new(Mutex::new(Vec::new()));
    }
    let message = message.unwrap();
    let mut attachments: Vec<Box<dyn Any + Send + Sync>> = Vec::new();
    for attachment in message.attachments.unwrap_or_default() {
        attachments.push(Box::new(attachment));
    }
    Arc::new(Mutex::new(attachments))
}
