#[cfg(feature = "bot")]
use teamtalk::{
    BotApp, Permissions, RequireClientRightsAll, RequireClientRightsAny, Result, Router, UserRights,
};

#[cfg(feature = "bot")]
fn main() -> Result<()> {
    let router = Router::new()
        .use_middleware(RequireClientRightsAny::new(
            UserRights::KICK_USERS | UserRights::BAN_USERS,
        ))
        .use_middleware(RequireClientRightsAll::new(
            Permissions::moderator().rights(),
        ));

    let client = teamtalk::Client::new()?;
    BotApp::new().with_router(router).run_sync(client)
}

#[cfg(not(feature = "bot"))]
fn main() {
    eprintln!("Enable the bot feature: cargo run --example bot_permissions --features bot");
}
