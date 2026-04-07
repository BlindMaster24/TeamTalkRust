#[cfg(feature = "bot")]
use teamtalk::{
    BotApp, Permissions, RequireClientRightsAll, RequireClientRightsAny, Result, Router,
};

#[cfg(feature = "bot")]
fn main() -> Result<()> {
    let router = Router::new()
        .use_middleware(RequireClientRightsAny::new(
            Permissions::file_manager().rights(),
        ))
        .use_middleware(RequireClientRightsAll::new(
            Permissions::channel_admin().rights(),
        ));

    let client = teamtalk::Client::new()?;
    BotApp::new().with_router(router).run_sync(client)
}

#[cfg(not(feature = "bot"))]
fn main() {
    eprintln!("Enable the bot feature: cargo run --example bot_permissions --features bot");
}
