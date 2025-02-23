use vtg::{
    client::requests::{File, FileType},
    structs::context::{EAttachment, UnifyedContext},
    upload::Attachment,
};

pub async fn test_attachments(ctx: UnifyedContext) {
    ctx.send("test attachments (check console)");

    if let Some(attachments) = ctx.attachments {
        match attachments {
            EAttachment::VK(vk_attachments) => {
                println!("VK attachments: {:?}", vk_attachments);
            }
            EAttachment::Telegram(tg_attachment) => {
                println!("TG attachment: {:?}", tg_attachment);
            }
        }
    }
}

pub async fn send_files(ctx: UnifyedContext) {
    ctx.send_attachment_files(
        "пива бы",
        vec![
            File {
                filename: "pivo.jpg".to_string(),
                content: tokio::fs::read("./examples/commands/pivo.jpg")
                    .await
                    .unwrap(),
                ftype: FileType::Photo,
            },
            File {
                filename: "pivo1.jpg".to_string(),
                content: tokio::fs::read("./examples/commands/pivo1.jpg")
                    .await
                    .unwrap(),
                ftype: FileType::Photo,
            },
            File {
                filename: "pivo2.jpg".to_string(),
                content: tokio::fs::read("./examples/commands/pivo2.jpg")
                    .await
                    .unwrap(),
                ftype: FileType::Photo,
            },
        ],
    )
    .await;
}

pub async fn send_attachments(ctx: UnifyedContext) {
    ctx.send_attachments(
            "attachments test",
            vec![Attachment {
                url:
                    "https://sn-gazeta.ru/wp-content/uploads/2023/04/tapeta-piwo-w-kuflu-i-szklance.jpg"
                        .to_string(),
                ftype: FileType::Photo,
            },
            Attachment {
                url:
                    "https://pivoug.ru/d/54702081_2.jpg"
                        .to_string(),
                ftype: FileType::Photo,
            },
            ],
        )
        .await;
}
