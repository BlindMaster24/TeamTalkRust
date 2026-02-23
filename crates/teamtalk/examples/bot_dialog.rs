#[cfg(feature = "bot")]
use teamtalk::{Bot, BotConfig, DialogFlow, HandlerResult, Result, Router};

#[cfg(feature = "bot")]
fn main() -> Result<()> {
    let client = teamtalk::Client::new()?;
    let onboarding = DialogFlow::new("onboarding", "ask_name")
        .step("ask_email")
        .step("done");
    let onboarding_start = onboarding.clone();
    let onboarding_name = onboarding.clone();

    let router = Router::new()
        .on_command("start", move |ctx| {
            ctx.dialog_start_checked(&onboarding_start)?;
            let _ = ctx.reply_private("Welcome! What is your name?");
            Ok(HandlerResult::Continue)
        })
        .on_dialog_step("onboarding", "ask_name", move |ctx| {
            let Some(name) = ctx.text() else {
                return Ok(HandlerResult::Continue);
            };
            ctx.user_state_set("name", name);
            ctx.dialog_advance_checked(&onboarding_name, "ask_email")?;
            let _ = ctx.reply_private("Thanks. Now send your e-mail:");
            Ok(HandlerResult::Continue)
        })
        .on_dialog_step("onboarding", "ask_email", |ctx| {
            let Some(email) = ctx.text() else {
                return Ok(HandlerResult::Continue);
            };
            ctx.user_state_set("email", email);
            let _ = ctx.reply_private("Onboarding complete.");
            let _ = ctx.dialog_stop();
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
