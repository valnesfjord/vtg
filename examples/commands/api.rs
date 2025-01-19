use serde_json::to_value;
use vtg::structs::{
    context::{Platform, SendOptions, UnifyedContext},
    keyboard::{Color, Keyboard, KeyboardButton},
    tg_api::{self, TGSendMessageOptions},
    vk_api::{self, VKMessagesSendOptions},
};

use super::KeyboardData;

pub async fn send_with_builder(ctx: UnifyedContext) {
    ctx.message("пива бы.")
        .keyboard(Keyboard::new(
            vec![vec![KeyboardButton::Text {
                color: Color::Positive,
                label: "Retry".to_string(),
                data: Some(
                    to_value(KeyboardData {
                        text: "testbuilder".to_string(),
                    })
                    .unwrap(),
                ),
            }]],
            true,
            false,
            &ctx.platform,
        ))
        .vk_options(vk_api::VKMessagesSendOptions {
            disable_mentions: Some(true),
            ..Default::default()
        })
        .tg_options(tg_api::TGSendMessageOptions {
            disable_notification: Some(true),
            ..Default::default()
        })
        .send()
        .await;
}

pub async fn send_with_options(ctx: UnifyedContext) {
    ctx.send_with_options(
        "@valnesfjord @cyournamec",
        SendOptions {
            vk: VKMessagesSendOptions {
                disable_mentions: Some(true),
                peer_id: Some(ctx.peer_id),
                ..Default::default()
            },
            tg: TGSendMessageOptions {
                disable_notification: Some(true),
                chat_id: Some(ctx.peer_id),
                ..Default::default()
            },
        },
    );
}

pub async fn send_with_api_request(ctx: UnifyedContext) {
    if ctx.platform == Platform::VK {
        let start_time = std::time::Instant::now();
        vk_api::Messages::send(
            VKMessagesSendOptions {
                peer_id: Some(ctx.peer_id),
                message: Some("testing api requests".to_string()),
                random_id: Some(0),
                ..Default::default()
            },
            ctx.config.clone(),
        )
        .await
        .unwrap();
        ctx.send(&format!("VK API request time: {:?}", start_time.elapsed()));
        return;
    }
    let start_time = std::time::Instant::now();
    tg_api::Api::send_message(
        TGSendMessageOptions {
            chat_id: Some(ctx.peer_id),
            text: Some("testing api requests".to_string()),
            ..Default::default()
        },
        ctx.config.clone(),
    )
    .await
    .unwrap();
    ctx.send(&format!("TG API request time: {:?}", start_time.elapsed()));
}
