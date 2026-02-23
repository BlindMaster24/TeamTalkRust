#[cfg(all(feature = "bot", feature = "bot-macros"))]
use teamtalk::{
    BotApp, HandlerResult, Result, Router, teamtalk_command, teamtalk_event, teamtalk_middleware,
};

#[cfg(all(feature = "bot", feature = "bot-macros"))]
#[teamtalk_command("help")]
fn help_handler(ctx: &mut teamtalk::Context<'_>) -> teamtalk::Result<HandlerResult> {
    let _ = ctx.reply_private("Commands: /help /ping");
    Ok(HandlerResult::Continue)
}

#[cfg(all(feature = "bot", feature = "bot-macros"))]
#[teamtalk_command("ping")]
fn ping_handler(ctx: &mut teamtalk::Context<'_>) -> teamtalk::Result<HandlerResult> {
    let _ = ctx.reply_private("pong");
    Ok(HandlerResult::Continue)
}

#[cfg(all(feature = "bot", feature = "bot-macros"))]
#[teamtalk_event(teamtalk::Event::ConnectionLost)]
fn stop_on_disconnect(_ctx: &mut teamtalk::Context<'_>) -> teamtalk::Result<HandlerResult> {
    Ok(HandlerResult::Stop)
}

#[cfg(all(feature = "bot", feature = "bot-macros"))]
#[teamtalk_middleware]
fn command_only(ctx: &mut teamtalk::Context<'_>) -> teamtalk::Result<HandlerResult> {
    Ok(if ctx.command.is_some() {
        HandlerResult::Continue
    } else {
        HandlerResult::Stop
    })
}

#[cfg(all(feature = "bot", feature = "bot-macros"))]
fn main() -> Result<()> {
    let client = teamtalk::Client::new()?;
    let router = register_command_only(register_stop_on_disconnect(register_ping_handler(
        register_help_handler(Router::new().with_auto_help()),
    )));
    BotApp::new().with_router(router).run_sync(client)
}

#[cfg(not(all(feature = "bot", feature = "bot-macros")))]
fn main() {
    eprintln!("Enable features: cargo run --example bot_macros --features bot,bot-macros");
}
