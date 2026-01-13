use teamtalk::{Client, Event};
use teamtalk::types::UserId;

fn main() -> teamtalk::Result<()> {
    let client = Client::new()?;
    let _sub = client
        .on_event(Event::TextMessage)
        .filter_user(UserId(1))
        .subscribe(|ctx| {
            let _ = ctx.text();
        });

    loop {
        let _ = client.poll(100);
    }
}
