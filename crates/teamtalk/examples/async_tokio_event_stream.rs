#[cfg(feature = "async-tokio")]
use futures::StreamExt;
#[cfg(feature = "async-tokio")]
use std::env;
#[cfg(feature = "async-tokio")]
use std::time::Duration;
#[cfg(feature = "async-tokio")]
use teamtalk::{AsyncConfig, Client, Event};

#[cfg(feature = "async-tokio")]
fn env_or(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

#[cfg(feature = "async-tokio")]
fn env_or_i32(name: &str, default: i32) -> i32 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(default)
}

#[cfg(feature = "async-tokio")]
fn main() -> teamtalk::Result<()> {
    let host = env_or("TT_HOST", "127.0.0.1");
    let tcp = env_or_i32("TT_TCP", 10333);
    let udp = env_or_i32("TT_UDP", 10333);

    let client = Client::new()?;
    client.connect(&host, tcp, udp, false)?;

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("tokio runtime");

    rt.block_on(async {
        let mut stream = client.into_async_with_config(AsyncConfig::default());

        // Timeout helper for bounded waits in bots/services.
        let _ = stream
            .wait_for_event_timeout(Event::ConnectSuccess, Duration::from_secs(5))
            .await;

        // Typed payload helper: waits for next event that carries data.
        let _ = stream.wait_for_data_timeout(Duration::from_secs(5)).await;

        // Stream-style API is still available.
        let _ = stream.next().await;

        stream.shutdown();
        let _ = stream.into_client();
    });

    Ok(())
}

#[cfg(not(feature = "async-tokio"))]
fn main() {
    eprintln!(
        "Enable async-tokio: cargo run --example async_tokio_event_stream --features async,async-tokio"
    );
}
