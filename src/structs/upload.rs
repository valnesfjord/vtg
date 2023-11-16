use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKPhotoGetUploadServerResponse {
    pub response: VKPhotoGetUploadServer,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKPhotoGetUploadServer {
    pub upload_url: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKMessagePhotoUploaded {
    pub hash: String,
    pub photo: String,
    pub server: i64,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKMessagePhotoResponse {
    pub response: Vec<VKMessagePhoto>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VKMessagePhoto {
    pub id: i64,
    pub owner_id: i64,
    pub access_key: String,
}
