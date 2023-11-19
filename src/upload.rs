use log::debug;
use serde_json::from_value;

use crate::{
    client::{
        api_requests::api_call,
        requests::{files_request, get_file, File, FileType},
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
        let server_resp = files_request(&val.response.upload_url, &[photo], None)
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

pub async fn download_files(attachments: Vec<Attachment>) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    for attachment in attachments {
        let file = get_file(&attachment.url).await.unwrap();
        files.push(file);
    }
    files
}

pub async fn send_tg_attachment_files(
    attachments: Vec<File>,
    config: &Config,
    peer_id: i64,
    message: &str,
) {
    if attachments.len() == 1 {
        files_request(
            &format!(
                "https://api.telegram.org/{}/send{}",
                attachments[0].ftype.to_string(),
                config.tg_access_token,
            ),
            &attachments,
            Some(vec![
                ("caption", message),
                ("chat_id", &peer_id.to_string()),
            ]),
        )
        .await
        .unwrap();
    } else {
        let mut media: Vec<String> = Vec::new();
        for (index, f) in attachments.iter().enumerate() {
            let mut name: String = f.ftype.to_string();
            if index != 0 {
                name = name + &index.to_string();
            }

            media.push(format!(
                "{{\"type\":\"{}\",\"media\":\"attach://{}\",\"caption\":\"{}\"}}",
                f.ftype.to_string().to_lowercase(),
                name.to_lowercase(),
                message
            ));
        }
        debug!("MEDIA: {}", media.join(","));
        files_request(
            &format!(
                "https://api.telegram.org/{}/sendMediaGroup",
                config.tg_access_token,
            ),
            &attachments,
            Some(vec![
                ("media", &format!("[{}]", media.join(","))),
                ("chat_id", &peer_id.to_string()),
            ]),
        )
        .await
        .unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct Attachment {
    pub url: String,
    pub ftype: FileType,
}

pub async fn send_tg_attachments(
    attachments: Vec<Attachment>,
    config: &Config,
    peer_id: i64,
    message: &str,
) {
    if attachments.len() == 1 {
        let ftype = attachments[0].ftype.to_string();
        api_call(
            Platform::Telegram,
            &format!("send{}", ftype),
            vec![
                ("caption", message),
                ("chat_id", &peer_id.to_string()),
                (&ftype.to_lowercase(), &attachments[0].url),
            ],
            config,
        )
        .await
        .unwrap();
    } else {
        let mut media: Vec<String> = Vec::new();
        for f in attachments.iter() {
            media.push(format!(
                "{{\"type\":\"{}\",\"media\":\"{}\",\"caption\":\"{}\"}}",
                f.ftype.to_string().to_lowercase(),
                f.url,
                message
            ));
        }
        debug!("MEDIA: {}", media.join(","));
        api_call(
            Platform::Telegram,
            "sendMediaGroup",
            vec![
                ("media", &format!("[{}]", media.join(","))),
                ("chat_id", &peer_id.to_string()),
            ],
            config,
        )
        .await
        .unwrap();
    }
}
