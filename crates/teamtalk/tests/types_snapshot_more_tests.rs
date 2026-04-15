use insta::assert_debug_snapshot;
use teamtalk::types::{AudioCodec, Channel, ChannelType, OpusCodec};

#[test]
fn channel_builder_snapshot() {
    let mut channel = Channel::builder("room")
        .topic("topic")
        .channel_type(ChannelType::from_raw(
            ChannelType::HIDDEN | ChannelType::PERMANENT,
        ))
        .max_users(10)
        .build();
    channel.audio_codec = AudioCodec::Opus(OpusCodec::new(
        48_000, 2, 2049, 10, true, false, 64_000, true, false, 20, 10,
    ));
    assert_debug_snapshot!(channel);
}
