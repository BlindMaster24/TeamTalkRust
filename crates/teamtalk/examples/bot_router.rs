#[cfg(feature = "bot")]
use teamtalk::{Bot, BotConfig, Event, HandlerResult, Result, Router};

#[cfg(feature = "bot")]
fn main() -> Result<()> {
    let client = teamtalk::Client::new()?;
    let router = Router::new()
        .on_command("help", |ctx| {
            if let Some(text) = ctx.message.text() {
                let _ = text.send_private(ctx.client, "Available commands: /help /ping");
            }
            Ok(HandlerResult::Continue)
        })
        .on_command("ping", |ctx| {
            if let Some(text) = ctx.message.text() {
                let _ = text.send_private(ctx.client, "pong");
            }
            Ok(HandlerResult::Continue)
        })
        .on_event(Event::ConnectionLost, |_ctx| Ok(HandlerResult::Stop));

    let mut bot = Bot::builder(client)
        .with_router(router)
        .with_config(BotConfig::new().poll_timeout_ms(100))
        .build();

    bot.run()
}

#[cfg(not(feature = "bot"))]
fn main() {
    eprintln!("Enable bot feature: cargo run --example bot_router --features bot");
}
