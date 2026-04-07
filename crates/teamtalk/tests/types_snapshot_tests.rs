use insta::assert_debug_snapshot;
use teamtalk::types::UserAccountBuilder;

#[test]
fn user_account_builder_snapshot() {
    let account = UserAccountBuilder::new("alice")
        .password("secret")
        .user_type(2)
        .rights(7)
        .note("ops")
        .init_channel("/Root/Ops")
        .user_data(42)
        .add_auto_operator_channel(teamtalk::types::ChannelId(5))
        .audio_codec_bps_limit(96000)
        .abuse_prevention(teamtalk::types::AbusePrevention {
            commands_limit: 3,
            commands_interval_ms: 500,
        })
        .build();
    assert_debug_snapshot!(account);
}
