use serde_json::to_value;
use vtg::structs::{
    context::UnifyedContext,
    keyboard::{Color, Keyboard, KeyboardButton},
};

use super::KeyboardData;

pub async fn keyboard_function(ctx: UnifyedContext) {
    ctx.send_with_keyboard(
        "Keyboard test",
        Keyboard::new(
            vec![vec![KeyboardButton::Text {
                color: Color::Positive,
                label: "Retry".to_string(),
                data: Some(
                    to_value(KeyboardData {
                        text: "testkeyboard".to_string(),
                    })
                    .unwrap(),
                ),
            }]],
            true,
            false,
            &ctx.platform,
        ),
    );
}
