#[cfg(feature = "bot")]
use teamtalk::{Bot, BotConfig, HandlerResult, Result, Router};

#[cfg(feature = "bot")]
fn main() -> Result<()> {
    let client = teamtalk::Client::new()?;

    let router = Router::new().on_command("start", |ctx| {
        ctx.dialog_start("onboarding", "ask_name");
        if let Some(text) = ctx.message.text() {
            let _ = text.send_private(ctx.client, "??????! ??? ???? ??????");
        }
        Ok(HandlerResult::Continue)
    });

    let mut bot = Bot::builder(client)
        .with_router(router)
        .with_config(BotConfig::new().poll_timeout_ms(100))
        .build();

    bot.run()
}

#[cfg(not(feature = "bot"))]
fn main() {
    eprintln!("Enable bot feature: cargo run --example bot_dialog --features bot");
}
