use vtg::{
    client::requests::{File, FileType},
    structs::{
        context::{Platform, UnifyedContext},
        tg_attachments::TGAttachment,
        vk_attachments::VKAttachment,
    },
    upload::Attachment,
};

pub async fn test_attachments(ctx: UnifyedContext) {
    ctx.send("test attachments (check console)");

    if ctx.platform == Platform::VK {
        let attachments = ctx.get_attachments::<VKAttachment>().unwrap_or_default();
        println!("{:?}", attachments);
        return;
    }

    let attachments = ctx.get_attachments::<TGAttachment>().unwrap_or_default();
    println!("{:?}", attachments);
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
                    "https://w.forfun.com/fetch/a9/a908815bda3f615bfe16bef28c6389db.jpeg"
                        .to_string(),
                ftype: FileType::Photo,
            },
            ],
        )
        .await;
}
