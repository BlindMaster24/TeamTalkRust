use std::collections::HashMap;
use std::time::{Duration, Instant};

use teamtalk::Event;
use teamtalk::types::{Subscriptions, UserId};

#[derive(Debug, Default)]
struct VoiceState {
    active: bool,
    last_sample_index: Option<u32>,
    last_block_at: Option<Instant>,
}

fn main() -> teamtalk::Result<()> {
    let client = teamtalk::Client::new()?;
    let mut states: HashMap<UserId, VoiceState> = HashMap::new();
    let silence_timeout = Duration::from_millis(800);

    loop {
        if let Some((event, message)) = client.poll(100) {
            match event {
                Event::UserJoined => {
                    if let Some(user) = message.user() {
                        let _ = client.subscribe(
                            user.id,
                            Subscriptions::from_raw(
                                Subscriptions::VOICE | Subscriptions::MEDIAFILE,
                            ),
                        );
                        let _ =
                            client.enable_audio_block_event(user.id, Subscriptions::VOICE, true);
                        states.entry(user.id).or_default();
                    }
                }
                Event::UserLeft => {
                    if let Some(user) = message.user() {
                        let _ =
                            client.enable_audio_block_event(user.id, Subscriptions::VOICE, false);
                        states.remove(&user.id);
                    }
                }
                Event::UserFirstVoiceStreamPacket => {
                    if let Some(user) = message.user() {
                        let state = states.entry(user.id).or_default();
                        if !state.active {
                            state.active = true;
                            println!(
                                "segment start (first packet): user_id={}, stream_id={}",
                                user.id.0,
                                message.source()
                            );
                        }
                    }
                }
                Event::AudioBlock => {
                    if let Some(user) = message.user()
                        && let Some(ptr) =
                            client.acquire_user_audio_block(Subscriptions::VOICE, user.id)
                    {
                        let block = unsafe { &*ptr };
                        let sample_index = block.uSampleIndex;
                        let now = Instant::now();

                        let state = states.entry(user.id).or_default();
                        if !state.active {
                            state.active = true;
                            println!(
                                "segment start (first block): user_id={}, sample_index={sample_index}",
                                user.id.0
                            );
                        } else if let Some(last) = state.last_sample_index
                            && sample_index == 0
                            && last > 0
                        {
                            println!(
                                "segment restart: user_id={}, sample_index={sample_index}",
                                user.id.0
                            );
                        }

                        state.last_sample_index = Some(sample_index);
                        state.last_block_at = Some(now);

                        unsafe {
                            let _ = client.release_user_audio_block(ptr);
                        }
                    }
                }
                Event::ConnectionLost | Event::ConnectFailed => break,
                _ => {}
            }
        }

        let now = Instant::now();
        for (user_id, state) in &mut states {
            if state.active
                && let Some(last) = state.last_block_at
                && now.duration_since(last) >= silence_timeout
            {
                state.active = false;
                state.last_sample_index = None;
                println!("segment end (silence timeout): user_id={}", user_id.0);
            }
        }
    }

    Ok(())
}
