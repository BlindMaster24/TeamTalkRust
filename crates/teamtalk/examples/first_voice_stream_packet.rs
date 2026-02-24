use teamtalk::Event;

fn main() -> teamtalk::Result<()> {
    let client = teamtalk::Client::new()?;

    loop {
        if let Some((event, message)) = client.poll(100) {
            match event {
                Event::UserJoined => {
                    if let Some(user) = message.user() {
                        let _ = client.subscribe(
                            user.id,
                            teamtalk::types::Subscriptions::from_raw(
                                teamtalk::types::Subscriptions::VOICE,
                            ),
                        );
                        let _ = client.enable_audio_block_event(
                            user.id,
                            teamtalk::types::Subscriptions::VOICE,
                            true,
                        );
                    }
                }
                Event::UserFirstVoiceStreamPacket => {
                    let stream_id = message.source();
                    let user_id = message.user().map(|u| u.id.0).unwrap_or_default();
                    println!("first voice stream packet: user_id={user_id}, stream_id={stream_id}");
                }
                Event::AudioBlock => {
                    if let Some(user) = message.user() {
                        let user_id = user.id;
                        if let Some(ptr) = client.acquire_user_audio_block(
                            teamtalk::types::Subscriptions::VOICE,
                            user_id,
                        ) {
                            let block = unsafe { &*ptr };
                            let sample_index = block.uSampleIndex;
                            let samples = block.nSamples;
                            let channels = block.nChannels;
                            println!(
                                "audio block: user_id={}, sample_index={sample_index}, samples={samples}, channels={channels}",
                                user_id.0
                            );
                            unsafe {
                                let _ = client.release_user_audio_block(ptr);
                            }
                        }
                    }
                }
                Event::ConnectionLost | Event::ConnectFailed => break,
                _ => {}
            }
        }
    }

    Ok(())
}
