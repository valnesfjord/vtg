use regex_automata::{meta::Regex, util::captures::Captures};
use std::{fs::File, future::Future, pin::Pin};
use tokio::io::AsyncReadExt;
use vtg::{
    client::{
        api_requests::{api_call, ApiResponse},
        requests::file_request,
    },
    structs::{
        context::{EventType, Platform, UnifyedContext},
        keyboard::{Color, KeyboardButton},
        tg::TGMessage,
        vk::VKMessageNew,
    },
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
                text: "Посмотреть баланс".to_string(),
                data: Some("{\"text\": \"balance\"}".to_string()),
                url: None,
            }]],
            true,
            None,
        ),
    );
    ctx
}
pub async fn ping_function(ctx: UnifyedContext) -> UnifyedContext {
    ctx.send("Pong");
    println!("{:?}", ctx);
    let data = ctx.get_data::<i32>().unwrap();
    println!("{:?}", data);

    let server_resp = ctx
        .api_call(
            Platform::VK,
            "photos.getMessagesUploadServer",
            vec![("group_id", "195053810"), ("v", "5.131")],
        )
        .await;
    let val = match server_resp {
        ApiResponse::VkResponse(val) => val,
        _ => panic!("Error while getting upload server ()"),
    };
    let resp_text = file_request(
        &val.get("response")
            .unwrap()
            .as_object()
            .unwrap()
            .get("upload_url")
            .unwrap()
            .as_str()
            .unwrap()
            .replace('\\', ""),
        File::open("C:\\Projects\\RustProjects\\vtg\\examples\\pivo.jpg").unwrap(),
    )
    .await
    .unwrap();
    println!("{:?}", resp_text);
    let response_json: serde_json::Value = serde_json::from_str(&resp_text).unwrap();
    println!("{:?}", response_json);

    let server_resp = ctx
        .api_call(
            Platform::VK,
            "photos.saveMessagesPhoto",
            vec![
                (
                    "photo",
                    response_json.get("photo").unwrap().as_str().unwrap(),
                ),
                (
                    "server",
                    &response_json
                        .get("server")
                        .unwrap()
                        .as_i64()
                        .unwrap()
                        .to_string(),
                ),
                ("hash", response_json.get("hash").unwrap().as_str().unwrap()),
                ("v", "5.131"),
            ],
        )
        .await;
    let val = match server_resp {
        ApiResponse::VkResponse(val) => val,
        _ => panic!("Error while getting upload server ()"),
    };
    println!("{:?}", val);
    ctx.api_call(
        Platform::VK,
        "messages.send",
        vec![
            ("peer_id", &ctx.peer_id.to_string()),
            ("message", "Pivo"),
            ("random_id", "0"),
            ("v", "5.131"),
            (
                "attachment",
                &format!("photo{}_{}", val[0]["owner_id"], val[0]["id"]),
            ),
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
