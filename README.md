# VTG - dual-platform bots library

<p align="center">
  <img src="https://github.com/valnesfjord/vtg/raw/HEAD/vtg.jpeg" width="200">
</p>
VTG is a fully functional library for creating bots for both VK and Telegram. Presents unified context and methods, for comfortable work with dual-platform bots.

## Features

-   Support callback and longpoll updates
-   90% VK and TG API coverage (messages)
-   Unified context for both platforms
-   Unified context methods for both platforms
-   Unified attachments and file uploads for both platforms
-   Unified keyboard for both platforms
-   Easy to use
-   Easy to understand
-   Easy to contribute

## Usage

Example using longpoll client:

```rust
use std::env;
use vtg::{
    client::start_longpoll_client,
    structs::{config::Config, context::UnifyedContext, middleware::MiddlewareChain},
};

async fn catch_new_message(ctx: UnifyedContext) -> UnifyedContext {
    ctx
}

#[tokio::main]
async fn main() {
    let config = Config {
        vk_access_token: env::var("VK_ACCESS_TOKEN").unwrap(),
        vk_group_id: env::var("VK_GROUP_ID").unwrap().parse().unwrap(),
        tg_access_token: env::var("TG_ACCESS_TOKEN").unwrap(), // token starts with "bot", like: bot1234567890:ABCDEFGHIJKL
        vk_api_version: "5.199".to_owned(),
        ..Default::default()
    };

    let mut middleware_chain = MiddlewareChain::new();
    middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));

    start_longpoll_client(middleware_chain, config).await;
}

```

Example using callback server:

```rust
use std::env;

use vtg::{
    server::start_callback_server,
    structs::{
        config::{CallbackSettings, Config},
        context::UnifyedContext,
        middleware::MiddlewareChain,
    },
};

async fn catch_new_message(ctx: UnifyedContext) -> UnifyedContext {
    ctx
}

#[tokio::main]
async fn main() {
    let config = Config {
        vk_access_token: env::var("VK_ACCESS_TOKEN").unwrap(),
        vk_group_id: env::var("VK_GROUP_ID").unwrap().parse().unwrap(),
        tg_access_token: env::var("TG_ACCESS_TOKEN").unwrap(), // token starts with "bot", like: bot1234567890:ABCDEFGHIJKL
        vk_api_version: "5.199".to_owned(),
        callback: Some(CallbackSettings {
            port: 1234,
            callback_url: "https://valnesfjord.com".to_string(),
            secret: "secret".to_string(),
            path: "yourcallbacksecretpathwithoutslashinstartandend".to_string(),
        }),
    };

    let mut middleware_chain = MiddlewareChain::new();
    middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));

    start_callback_server(middleware_chain, config).await;
}
```

## Examples

You can find example bot in the [examples folder](https://github.com/valnesfjord/vtg/tree/master/examples)

## Try bot, that works with vtg

You can try test bot, that works in actual version of vtg: [tg](https://t.me/deformation_bot), [vk](https://vk.me/testdeformation)

## Documentation

You can find the documentation [here](https://docs.rs/vtg)

## It's not finished yet:

-   [ ] Add more tests
-   [ ] Add more examples
-   [ ] Add VK and TG API documentation
-   [ ] Add more features (like more API coverage, etc)

## Contact the maintainer

Telegram: @valnesfjord Discord: valnesfjord VK: https://vk.com/cyournamec

## Contribution

Contributions are always welcome! If you have any ideas, suggestions, or issues, feel free to contribute. You can fork the repository and create a pull request with your changes, or create an issue if you find any bugs or have suggestions for improvements.

We appreciate your help in making this project better!
