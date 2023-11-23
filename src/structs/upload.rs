use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKGetUploadServerResponse {
    pub response: VKGetUploadServer,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKGetUploadServer {
    pub upload_url: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKMessagePhotoUploaded {
    pub hash: String,
    pub photo: String,
    pub server: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKMessageDocumentUploaded {
    pub file: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKMessagePhotoResponse {
    pub response: Vec<VKMessagePhoto>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKMessageDocumentResponse {
    pub response: VKMessageDocument,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKMessageDocument {
    pub audio_message: Option<VKAudioMessage>,
    pub doc: Option<VKInMessageDocument>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKInMessageDocument {
    pub id: i64,
    pub owner_id: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKAudioMessage {
    pub id: i64,
    pub owner_id: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKMessagePhoto {
    pub id: i64,
    pub owner_id: i64,
    pub access_key: String,
}
