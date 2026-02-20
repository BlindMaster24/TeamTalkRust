#[cfg(feature = "bot")]
use teamtalk::{BotApp, BotConfig, HandlerResult, Result, Router, UnknownCommandPolicy};

#[cfg(feature = "bot")]
fn main() -> Result<()> {
    let client = teamtalk::Client::new()?;
    let router = Router::new()
        .with_unknown_command_policy(UnknownCommandPolicy::Reply(
            "Unknown command. Try /help".to_owned(),
        ))
        .on_command("help", |ctx| {
            let _ = ctx.reply_private("Commands: /help /echo <text>");
            Ok(HandlerResult::Continue)
        })
        .on_command("echo", |ctx| {
            let msg = ctx
                .args()
                .and_then(|args| args.rest(0))
                .unwrap_or_else(|| "empty".to_owned());
            let _ = ctx.reply_private(&msg);
            Ok(HandlerResult::Continue)
        });

    BotApp::new()
        .with_router(router)
        .with_config(BotConfig::new().poll_timeout_ms(100))
        .run_sync(client)
}

#[cfg(not(feature = "bot"))]
fn main() {
    eprintln!("Enable bot feature: cargo run --example bot_app --features bot");
}
