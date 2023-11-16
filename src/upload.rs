use std::mem;

use serde_json::from_value;

use crate::{
    client::{
        api_requests::api_call,
        requests::{files_request, File},
    },
    structs::{
        config::Config,
        context::Platform,
        upload::{VKMessagePhotoResponse, VKMessagePhotoUploaded, VKPhotoGetUploadServerResponse},
    },
};

pub async fn upload_vk_message_photos(
    photos: Vec<File>,
    config: &Config,
    peer_id: i64,
) -> Result<String, String> {
    let resp = api_call(
        Platform::VK,
        "photos.getMessagesUploadServer",
        vec![("peer_id", &peer_id.to_string()), ("v", "5.131")],
        config,
    )
    .await?;
    let val: VKPhotoGetUploadServerResponse = from_value(resp).unwrap();
    let mut message_photos: String = "".to_string();
    for photo in photos {
        let server_resp = files_request(&val.response.upload_url, &vec![photo], None)
            .await
            .unwrap();
        let uploaded_photo: VKMessagePhotoUploaded = serde_json::from_str(&server_resp).unwrap();
        let server_resp = api_call(
            Platform::VK,
            "photos.saveMessagesPhoto",
            vec![
                ("photo", &uploaded_photo.photo),
                ("server", &uploaded_photo.server.to_string()),
                ("hash", &uploaded_photo.hash),
                ("v", "5.131"),
            ],
            config,
        )
        .await?;
        let message_photo: VKMessagePhotoResponse = from_value(server_resp).unwrap();
        message_photos.push_str(&format!(
            "photo{}_{},",
            message_photo.response[0].owner_id, message_photo.response[0].id
        ))
    }
    Ok(message_photos)
}

pub async fn send_tg_photo(photos: Vec<File>, config: &Config, peer_id: i64, message: &str) {
    files_request(
        &format!(
            "https://api.telegram.org/{}/sendPhoto",
            config.tg_access_token,
        ),
        &photos,
        Some(vec![
            ("caption", message),
            ("chat_id", &peer_id.to_string()),
        ]),
    )
    .await
    .unwrap();
}
