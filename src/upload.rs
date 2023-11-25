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
        upload::{
            VKGetUploadServerResponse, VKMessageDocumentResponse, VKMessageDocumentUploaded,
            VKMessagePhotoResponse, VKMessagePhotoUploaded,
        },
    },
};

pub async fn download_files(attachments: Vec<Attachment>) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    for attachment in attachments {
        let file = get_file(&attachment.url).await.unwrap();
        files.push(file);
    }
    files
}
struct VKUploadServers {
    photo: String,
    audio: String,
    doc: String,
}
pub async fn upload_vk_attachments(
    attachments: Vec<File>,
    config: &Config,
    peer_id: i64,
) -> Result<String, String> {
    let mut message_attachments: String = "".to_string();
    let mut upload_servers = VKUploadServers {
        photo: "".to_string(),
        audio: "".to_string(),
        doc: "".to_string(),
    };
    for attachment in attachments {
        let server: String = if attachment.ftype == FileType::Photo {
            if upload_servers.photo.is_empty() {
                let resp = api_call(
                    Platform::VK,
                    "photos.getMessagesUploadServer",
                    vec![("peer_id", &peer_id.to_string()), ("v", "5.131")],
                    config,
                )
                .await?;
                let val: VKGetUploadServerResponse = from_value(resp).unwrap();
                upload_servers.photo = val.response.upload_url;
                upload_servers.photo.clone()
            } else {
                upload_servers.photo.clone()
            }
        } else if attachment.ftype == FileType::Audio || attachment.ftype == FileType::Voice {
            if upload_servers.audio.is_empty() {
                let resp = api_call(
                    Platform::VK,
                    "docs.getMessagesUploadServer",
                    vec![
                        ("peer_id", &peer_id.to_string()),
                        ("type", "audio_message"),
                        ("v", "5.131"),
                    ],
                    config,
                )
                .await?;
                let val: VKGetUploadServerResponse = from_value(resp).unwrap();
                upload_servers.doc = val.response.upload_url;
                upload_servers.doc.clone()
            } else {
                upload_servers.audio.clone()
            }
        } else if upload_servers.doc.is_empty() {
            let resp = api_call(
                Platform::VK,
                "docs.getMessagesUploadServer",
                vec![
                    ("peer_id", &peer_id.to_string()),
                    ("type", "doc"),
                    ("v", "5.131"),
                ],
                config,
            )
            .await?;
            let val: VKGetUploadServerResponse = from_value(resp).unwrap();
            upload_servers.doc = val.response.upload_url;
            upload_servers.doc.clone()
        } else {
            upload_servers.doc.clone()
        };

        if attachment.ftype == FileType::Photo {
            let server_resp = files_request(&server, &[attachment], None, Platform::VK)
                .await
                .unwrap();
            let uploaded_photo: VKMessagePhotoUploaded =
                serde_json::from_str(&server_resp).unwrap();
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
            message_attachments.push_str(&format!(
                "photo{}_{},",
                message_photo.response[0].owner_id, message_photo.response[0].id
            ))
        } else {
            let ftype = attachment.ftype.clone();
            let server_resp = files_request(&server, &[attachment], None, Platform::VK)
                .await
                .unwrap();
            let uploaded_doc: VKMessageDocumentUploaded =
                serde_json::from_str(&server_resp).unwrap();
            let server_resp = api_call(
                Platform::VK,
                "docs.save",
                vec![("file", &uploaded_doc.file), ("v", "5.131")],
                config,
            )
            .await?;
            if ftype == FileType::Audio || ftype == FileType::Voice {
                let message_audio: VKMessageDocumentResponse = from_value(server_resp).unwrap();
                let audio_message = message_audio.response.audio_message.unwrap();
                message_attachments.push_str(&format!(
                    "audio_message{}_{},",
                    audio_message.owner_id, audio_message.id
                ))
            } else {
                let message_doc: VKMessageDocumentResponse = from_value(server_resp).unwrap();
                let doc = message_doc.response.doc.unwrap();
                message_attachments.push_str(&format!("doc{}_{},", doc.owner_id, doc.id))
            }
        }
    }
    Ok(message_attachments)
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
                attachments[0].ftype.to_string().replace('_', ""),
                config.tg_access_token,
            ),
            &attachments,
            Some(vec![
                ("caption", message),
                ("chat_id", &peer_id.to_string()),
            ]),
            Platform::Telegram,
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
            Platform::Telegram,
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
