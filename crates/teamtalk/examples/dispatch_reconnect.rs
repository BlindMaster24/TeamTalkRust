#[cfg(feature = "dispatch")]
use std::env;
#[cfg(feature = "dispatch")]
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
#[cfg(feature = "dispatch")]
use teamtalk::dispatch::{ClientConfig, ConnectParamsOwned, DispatchFlow, Dispatcher};
#[cfg(feature = "dispatch")]
use teamtalk::types::ChannelId;
#[cfg(feature = "dispatch")]
use teamtalk::{Client, Event, LoginParams, ReconnectConfig};

#[cfg(feature = "dispatch")]
fn env_or(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

#[cfg(feature = "dispatch")]
fn env_or_i32(name: &str, default: i32) -> i32 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(default)
}

#[cfg(feature = "dispatch")]
fn main() -> teamtalk::Result<()> {
    let host = env_or("TT_HOST", "127.0.0.1");
    let tcp = env_or_i32("TT_TCP", 10333);
    let udp = env_or_i32("TT_UDP", 10333);
    let nickname = env_or("TT_NICK", "DispatchBot");
    let username = env_or("TT_USER", "guest");
    let password = env_or("TT_PASS", "guest");
    let client_name = env_or("TT_CLIENT", "TeamTalkRust");
    let channel_password = env_or("TT_CHAN_PASS", "");
    let root_channel = ChannelId(1);
    let should_join = Arc::new(AtomicBool::new(true));

    let client = Client::new()?;
    client.set_login_params(LoginParams::new(
        &nickname,
        &username,
        &password,
        &client_name,
    ));
    client.set_last_channel(root_channel, Some(&channel_password));
    client.connect_remember(&host, tcp, udp, false)?;

    let config = ClientConfig::new().reconnect_with_events(
        ConnectParamsOwned::new(&host, tcp, udp, false),
        ReconnectConfig::default(),
        vec![Event::MySelfKicked],
    );

    let should_join_on_login = should_join.clone();
    let mut dispatcher = Dispatcher::with_config(client, config)
        .on_connect_success(move |ctx| {
            if let Some(client) = ctx.client() {
                client.login_with_params().ok();
            }
            DispatchFlow::Continue
        })
        .on_event(Event::MySelfKicked, {
            let should_join = should_join.clone();
            move |_| {
                should_join.store(true, Ordering::Relaxed);
                DispatchFlow::Continue
            }
        })
        .on_event(Event::ConnectionLost, {
            let should_join = should_join.clone();
            move |_| {
                should_join.store(true, Ordering::Relaxed);
                DispatchFlow::Continue
            }
        })
        .on_event(Event::MySelfLoggedIn, move |ctx| {
            if let Some(client) = ctx.client()
                && should_join_on_login.swap(false, Ordering::Relaxed)
            {
                client.join_channel(root_channel, &channel_password);
            }
            DispatchFlow::Continue
        });

    dispatcher.run();
    Ok(())
}

#[cfg(not(feature = "dispatch"))]
fn main() {
    eprintln!(
        "Enable the dispatch feature: cargo run --example dispatch_reconnect --features dispatch"
    );
}
