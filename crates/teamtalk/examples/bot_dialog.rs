#[cfg(feature = "bot")]
use std::time::Duration;
#[cfg(feature = "bot")]
use teamtalk::{
    Bot, BotConfig, DialogFlow, DialogState, DialogTimeoutPolicy, HandlerResult, Result, Router,
};

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
                .with_timeout_policy(DialogTimeoutPolicy::Pause)
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
            ctx.user_state_set("email", &email);
            ctx.user_state_set_typed("completed_steps", 2_u32);
            let locale = ctx
                .dialog_metadata("locale")
                .unwrap_or_else(|| "en".to_owned());
            let _ = ctx.dialog_advance_next(&onboarding_email)?;
            let current_step = ctx.dialog_step().unwrap_or_else(|| "unknown".to_owned());
            let started = ctx.dialog_state_parse::<bool>("started")?.unwrap_or(false);
            let completed_steps = ctx.user_state_parse::<u32>("completed_steps")?.unwrap_or(0);
            let _ = ctx.reply_private("Onboarding complete.");
            let _ = ctx.reply_private(&format!(
                "Stored locale: {locale}; started: {started}; step: {current_step}; completed: {completed_steps}"
            ));
            let _ = ctx.dialog_finish();
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
