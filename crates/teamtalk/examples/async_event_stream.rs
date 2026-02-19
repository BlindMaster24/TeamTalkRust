#[cfg(feature = "async")]
use futures::StreamExt;
#[cfg(feature = "async")]
use teamtalk::{Client, Event};

#[cfg(feature = "async")]
fn main() -> teamtalk::Result<()> {
    // Convert the polling client into an async stream of events.
    let client = Client::new()?;
    let mut stream = client.into_async();

    futures::executor::block_on(async {
        // High-level helper: wait for a specific condition without manual loops.
        let _ = stream
            .wait_for_predicate(|event, _| {
                matches!(event, Event::ConnectionLost | Event::ConnectFailed)
            })
            .await;

        // Stream-style iteration still works when you need full control.
        if let Some((event, _msg)) = stream.next().await
            && matches!(event, Event::ConnectionLost | Event::ConnectFailed)
        {
            // no-op
        }
    });

    // Graceful shutdown keeps lifecycle explicit in apps.
    stream.shutdown();
    let _client = stream.into_client();

    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    // This example requires the "async" feature.
    eprintln!("Enable the async feature: cargo run --example async_event_stream --features async");
}
