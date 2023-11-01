use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Attachment {
    pub r#type: String,
    pub photo: Option<PhotoAttachment>,
    pub video: Option<VideoAttachment>,
    pub audio: Option<AudioAttachment>,
    pub doc: Option<DocAttachment>,
    pub link: Option<LinkAttachment>,
    pub sticker: Option<StickerAttachment>,
    pub gift: Option<GiftAttachment>,
}

#[derive(Deserialize, Serialize)]
pub struct PhotoAttachment {
    pub id: i64,
    pub album_id: i64,
    pub owner_id: i64,
    pub user_id: i64,
    pub text: String,
    pub sizes: Vec<VKPhotoSizes>,
}
#[derive(Deserialize, Serialize)]
pub struct VKPhotoSizes {
    pub r#type: String,
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize, Serialize)]
pub struct VideoAttachment {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
    pub description: String,
    pub duration: i64,
    pub image: Vec<VKVideoImage>,
    pub first_frame: Vec<VKVideoFirstFrame>,
}
#[derive(Deserialize, Serialize)]
pub struct VKVideoImage {
    pub with_padding: Option<i8>,
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize, Serialize)]
pub struct VKVideoFirstFrame {
    pub url: String,
    pub width: i32,
    pub height: i32,
}
