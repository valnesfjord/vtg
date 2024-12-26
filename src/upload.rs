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
        struct_to_vec::param,
        upload::{
            VKGetUploadServerResponse, VKMessageDocumentResponse, VKMessageDocumentUploaded,
            VKMessagePhotoResponse, VKMessagePhotoUploaded,
        },
    },
};

/// Download files from URLs
/// # Arguments
/// * `attachments` - Vector of attachments to download
///
/// # Returns
/// * `Vec<File>` - Vector of downloaded files
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

/// Upload files to VK
///
/// # Arguments
/// * `attachments` - Vector of files to upload
/// * `config` - Config to use
/// * `peer_id` - Peer ID to send attachments to
///
/// # Returns
/// * `Result<String, String>` - String of uploaded attachments
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
        let server: &str = match attachment.ftype {
            FileType::Photo => {
                if upload_servers.photo.is_empty() {
                    let val: VKGetUploadServerResponse = from_value(
                        api_call(
                            Platform::VK,
                            "photos.getMessagesUploadServer",
                            vec![param("peer_id", peer_id.to_string())],
                            config,
                        )
                        .await?,
                    )
                    .unwrap();
                    upload_servers.photo = val.response.upload_url;
                }
                &upload_servers.photo
            }
            FileType::Audio | FileType::Voice => {
                if upload_servers.audio.is_empty() {
                    let val: VKGetUploadServerResponse = from_value(
                        api_call(
                            Platform::VK,
                            "docs.getMessagesUploadServer",
                            vec![
                                param("peer_id", peer_id.to_string()),
                                param("type", "audio_message"),
                            ],
                            config,
                        )
                        .await?,
                    )
                    .unwrap();
                    upload_servers.audio = val.response.upload_url;
                }
                &upload_servers.audio
            }
            _ => {
                if upload_servers.doc.is_empty() {
                    let val: VKGetUploadServerResponse = from_value(
                        api_call(
                            Platform::VK,
                            "docs.getMessagesUploadServer",
                            vec![param("peer_id", peer_id.to_string()), param("type", "doc")],
                            config,
                        )
                        .await?,
                    )
                    .unwrap();
                    upload_servers.doc = val.response.upload_url;
                }
                &upload_servers.doc
            }
        };

        match attachment.ftype {
            FileType::Photo => {
                let uploaded_photo: VKMessagePhotoUploaded = serde_json::from_str(
                    &files_request(server, &[attachment], None, Platform::VK)
                        .await
                        .unwrap(),
                )
                .unwrap();
                let message_photo: VKMessagePhotoResponse = from_value(
                    api_call(
                        Platform::VK,
                        "photos.saveMessagesPhoto",
                        vec![
                            param("photo", uploaded_photo.photo),
                            param("server", uploaded_photo.server.to_string()),
                            param("hash", uploaded_photo.hash),
                        ],
                        config,
                    )
                    .await?,
                )
                .unwrap();
                message_attachments.push_str(&format!(
                    "photo{}_{},",
                    message_photo.response[0].owner_id, message_photo.response[0].id
                ))
            }
            _ => {
                let ftype = attachment.ftype.clone();
                let server_resp = files_request(server, &[attachment], None, Platform::VK)
                    .await
                    .unwrap();
                let uploaded_doc: VKMessageDocumentUploaded =
                    serde_json::from_str(&server_resp).unwrap();
                let server_resp = api_call(
                    Platform::VK,
                    "docs.save",
                    vec![param("file", uploaded_doc.file)],
                    config,
                )
                .await?;
                match ftype {
                    FileType::Audio | FileType::Voice => {
                        let message_audio: VKMessageDocumentResponse =
                            from_value(server_resp).unwrap();
                        let audio_message = message_audio.response.audio_message.unwrap();
                        message_attachments.push_str(&format!(
                            "audio_message{}_{},",
                            audio_message.owner_id, audio_message.id
                        ))
                    }
                    _ => {
                        let message_doc: VKMessageDocumentResponse =
                            from_value(server_resp).unwrap();
                        let doc = message_doc.response.doc.unwrap();
                        message_attachments.push_str(&format!("doc{}_{},", doc.owner_id, doc.id))
                    }
                }
            }
        }
    }
    Ok(message_attachments)
}

/// Send files to TG
///
/// # Arguments
/// * `attachments` - Vector of files to send
/// * `config` - Config to use
/// * `peer_id` - Chat ID to send attachments to
/// * `message` - Message to send with attachments
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
                config.tg_access_token,
                attachments[0].ftype.to_string().replace('_', ""),
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

/// Attachment struct
/// # Fields
/// * `url` - URL of the attachment
/// * `ftype` - FileType of the attachment
#[derive(Debug, Clone)]
pub struct Attachment {
    pub url: String,
    pub ftype: FileType,
}

/// Send attachments to TG
/// # Arguments
/// * `attachments` - Vector of attachments to send
/// * `config` - Config to use
/// * `peer_id` - Chat ID to send attachments to
/// * `message` - Message to send with attachments
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
            &format!("send{}", ftype.replace('_', "")),
            vec![
                param("caption", message),
                param("chat_id", peer_id.to_string()),
                param(ftype.to_lowercase(), &attachments[0].url),
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
                param("media", format!("[{}]", media.join(","))),
                param("chat_id", peer_id.to_string()),
            ],
            config,
        )
        .await
        .unwrap();
    }
}
