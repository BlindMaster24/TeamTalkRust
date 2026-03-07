#[cfg(feature = "bot")]
use std::time::Duration;
#[cfg(feature = "bot")]
use teamtalk::{Bot, BotConfig, DialogFlow, DialogState, HandlerResult, Result, Router};

#[cfg(feature = "bot")]
fn main() -> Result<()> {
    let client = teamtalk::Client::new()?;
    let onboarding = DialogFlow::new("onboarding", "ask_name")
        .step("ask_email")
        .step("done");
    let onboarding_start = onboarding.clone();
    let onboarding_name = onboarding.clone();
    let onboarding_email = onboarding.clone();

    let router = Router::new()
        .on_command("start", move |ctx| {
            let state = DialogState::new(onboarding_start.name(), onboarding_start.start_step())
                .with_timeout(Duration::from_secs(300))
                .with_metadata([("locale", "en"), ("mode", "guided")]);
            ctx.dialog_start_state(state);
            let _ = ctx.reply_private("Welcome! What is your name?");
            Ok(HandlerResult::Continue)
        })
        .on_dialog_step("onboarding", "ask_name", move |ctx| {
            let Some(name) = ctx.text() else {
                return Ok(HandlerResult::Continue);
            };
            ctx.user_state_set("name", &name);
            let _ = ctx.dialog_set_metadata("name", name);
            let _ = ctx.dialog_state_set("started", "true");
            ctx.dialog_advance_next(&onboarding_name)?;
            let _ = ctx.reply_private("Thanks. Now send your e-mail:");
            Ok(HandlerResult::Continue)
        })
        .on_dialog_step("onboarding", "ask_email", move |ctx| {
            let Some(email) = ctx.text() else {
                return Ok(HandlerResult::Continue);
            };
            ctx.user_state_set("email", email);
            let locale = ctx
                .dialog_metadata("locale")
                .unwrap_or_else(|| "en".to_owned());
            let _ = ctx.dialog_advance_next(&onboarding_email)?;
            let _ = ctx.dialog_step();
            let started = ctx
                .dialog_state_get("started")
                .unwrap_or_else(|| "false".to_owned());
            let _ = ctx.reply_private("Onboarding complete.");
            let _ = ctx.reply_private(&format!("Stored locale: {locale}; started: {started}"));
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
