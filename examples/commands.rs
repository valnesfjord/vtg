use regex_automata::{meta::Regex, util::captures::Captures};
use std::{future::Future, pin::Pin};
use vtg::{
    client::requests::FileType,
    structs::{
        context::{EventType, Platform, SendOptions, UnifyedContext},
        keyboard::{Color, KeyboardButton},
        tg::TGMessage,
        tg_api::{self, TGSendMessageOptions},
        vk::VKMessageNew,
        vk_api::{self, VKMessageSendOptions},
    },
    upload::Attachment,
};

type CommandFunction = Pin<Box<dyn Future<Output = UnifyedContext> + Send + 'static>>;
pub struct Command {
    pub regex: Regex,
    pub function: fn(UnifyedContext, Captures) -> CommandFunction,
}
pub fn get_potential_matches(text: String, caps: Captures) -> Vec<String> {
    caps.iter()
        .map(|a| text.as_str().get(a.unwrap().range()).unwrap().to_string())
        .collect()
}

pub async fn hello_function(ctx: UnifyedContext, caps: Captures) -> UnifyedContext {
    println!("{:?}", get_potential_matches(ctx.clone().text, caps));
    ctx.send_with_keyboard(
        "Hello",
        vtg::structs::keyboard::Keyboard::new(
            vec![vec![KeyboardButton {
                color: Color::Positive,
                text: "Посмотреть".to_string(),
                data: Some("{\"text\": \"test\"}".to_string()),
                url: None,
            }]],
            true,
            None,
        ),
    );
    ctx.send_with_options(
        "пивко @valnesfjord @cyournamec",
        SendOptions {
            vk: VKMessageSendOptions {
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

    if ctx.platform == Platform::VK {
        vk_api::Messages::send(
            VKMessageSendOptions {
                peer_id: Some(ctx.peer_id),
                message: Some("test".to_string()),
                random_id: Some(0),
                ..Default::default()
            },
            ctx.config.clone(),
        )
        .await
        .unwrap();
    } else {
        tg_api::Api::send_message(
            TGSendMessageOptions {
                chat_id: Some(ctx.peer_id),
                text: Some("test".to_string()),
                ..Default::default()
            },
            ctx.config.clone(),
        );
    }
    ctx
}
pub async fn ping_function(ctx: UnifyedContext) -> UnifyedContext {
    ctx.send("Pong");
    println!("{:?}", ctx);
    let data = ctx.get_data::<i32>().unwrap();
    println!("{:?}", data);

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
    ctx
}

pub async fn bye_function(ctx: UnifyedContext) -> UnifyedContext {
    ctx.send("Goodbye");
    if ctx.r#type == EventType::MessageNew {
        match ctx.platform {
            Platform::Telegram => {
                let event = ctx.get_event::<TGMessage>().unwrap();
                println!("{:?}", event);
            }
            Platform::VK => {
                let event = ctx.get_event::<VKMessageNew>().unwrap();
                println!("{:?}", event);
            }
        }
    }
    ctx
}

pub fn command_vec() -> Vec<Command> {
    vec![
        Command {
            regex: Regex::new(r"(?:hello|hi)\s(.*)").unwrap(),
            function: |ctx, caps| Box::pin(hello_function(ctx, caps)),
        },
        Command {
            regex: Regex::new(r"!ping").unwrap(),
            function: |ctx, _| Box::pin(ping_function(ctx)),
        },
        Command {
            regex: Regex::new(r"bye|goodbye").unwrap(),
            function: |ctx, _| Box::pin(bye_function(ctx)),
        },
    ]
}
#[allow(dead_code)]
fn main() {}
