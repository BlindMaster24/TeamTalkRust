#![cfg(feature = "mock")]

use std::sync::Arc;
use std::time::Duration;

use teamtalk::client::Client;
use teamtalk::client::backend::MockBackend;
use teamtalk::client::connection::{
    ConnectParamsOwned, ReconnectConfig, ReconnectPhaseTimeouts, ReconnectWorkflowConfig,
};
use teamtalk::client::ffi;
use teamtalk::client::users::LoginParams;
use teamtalk::client::users::SendTextOptions;
use teamtalk::events::ConnectionState;
use teamtalk::types::{
    Channel, ChannelId, MessageTarget, TT_STRLEN, UserId, UserPresence, UserStatus,
};

fn test_channel(id: i32, name: &str) -> Channel {
    let mut channel = Channel::builder(name).build();
    channel.id = ChannelId(id);
    channel
}

fn raw_user(id: i32, username: &str, local_subscriptions: u32) -> ffi::User {
    let mut user = unsafe { std::mem::zeroed::<ffi::User>() };
    user.nUserID = id;
    user.uLocalSubscriptions = local_subscriptions;
    let encoded: Vec<_> = username.encode_utf16().chain(std::iter::once(0)).collect();
    let len = encoded.len().min(user.szUsername.len());
    user.szUsername[..len].copy_from_slice(&encoded[..len]);
    user
}

#[test]
fn login_with_params_requires_connected_state() {
    let backend = Arc::new(MockBackend::new());
    backend.set_login_result(42);
    let client = Client::with_backend(backend.clone()).expect("client");
    client.set_login_params(teamtalk::client::users::LoginParams::new(
        "nick", "user", "pass", "client",
    ));

    let cmd_id = client.login_with_params().expect("login");

    assert_eq!(cmd_id, 0);
    assert_eq!(client.connection_state(), ConnectionState::Idle);
    assert_eq!(backend.last_login(), None);
}

#[test]
fn login_with_params_requires_login_params() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");

    let err = client.login_with_params().expect_err("missing params");

    assert!(matches!(err, teamtalk::events::Error::MissingLoginParams));
}

#[test]
fn login_from_env_uses_env() {
    let backend = Arc::new(MockBackend::new());
    backend.set_login_result(7);
    let client = Client::with_backend(backend.clone()).expect("client");

    let original_nick = std::env::var("TT_NICK").ok();
    let original_user = std::env::var("TT_USER").ok();
    let original_pass = std::env::var("TT_PASS").ok();
    let original_client = std::env::var("TT_CLIENT").ok();

    unsafe {
        std::env::set_var("TT_NICK", "nick-env");
        std::env::set_var("TT_USER", "user-env");
        std::env::set_var("TT_PASS", "pass-env");
        std::env::set_var("TT_CLIENT", "client-env");
    }

    let cmd_id = client.login_from_env();

    assert_eq!(cmd_id, 0);
    assert_eq!(backend.last_login(), None);

    match original_nick {
        Some(value) => unsafe { std::env::set_var("TT_NICK", value) },
        None => unsafe { std::env::remove_var("TT_NICK") },
    }
    match original_user {
        Some(value) => unsafe { std::env::set_var("TT_USER", value) },
        None => unsafe { std::env::remove_var("TT_USER") },
    }
    match original_pass {
        Some(value) => unsafe { std::env::set_var("TT_PASS", value) },
        None => unsafe { std::env::remove_var("TT_PASS") },
    }
    match original_client {
        Some(value) => unsafe { std::env::set_var("TT_CLIENT", value) },
        None => unsafe { std::env::remove_var("TT_CLIENT") },
    }
}

#[test]
fn login_returns_zero_when_not_connected() {
    let backend = Arc::new(MockBackend::new());
    backend.set_login_result(9);
    let client = Client::with_backend(backend).expect("client");
    let cmd_id = client.login("nick", "user", "pass", "client");
    assert_eq!(cmd_id, 0);
    assert_eq!(client.connection_state(), ConnectionState::Idle);
}

#[test]
fn join_channel_sets_state_when_successful() {
    let backend = Arc::new(MockBackend::new());
    backend.set_join_result(11);
    backend.set_channel(test_channel(1, "main"));

    let client = Client::with_backend(backend).expect("client");
    let cmd_id = client.join_channel(ChannelId(1), "");

    assert_eq!(cmd_id, 0);
    assert_eq!(client.connection_state(), ConnectionState::Idle);
}

#[test]
fn join_channel_does_not_change_state_on_failure() {
    let backend = Arc::new(MockBackend::new());
    backend.set_join_result(0);
    let client = Client::with_backend(backend).expect("client");

    let cmd_id = client.join_channel(ChannelId(1), "");

    assert_eq!(cmd_id, 0);
    assert_eq!(client.connection_state(), ConnectionState::Idle);
}

#[test]
fn send_text_short_message_uses_single_packet() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend.clone()).expect("client");

    let cmd_id = client.send_to_user(UserId(99), "hello");

    assert_eq!(cmd_id, 0);
    let messages = backend.text_messages();
    assert_eq!(messages.len(), 0);
}

#[test]
fn send_text_long_message_splits_and_sets_more_flag() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend.clone()).expect("client");
    let text = "a".repeat(TT_STRLEN + 25);

    let cmd_id = client.send_to_channel(ChannelId(7), &text);

    assert_eq!(cmd_id, 0);
    let messages = backend.text_messages();
    assert!(messages.is_empty());
}

#[test]
fn send_text_boundary_exact_limit_is_single_packet() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend.clone()).expect("client");
    let text = "b".repeat(TT_STRLEN - 1);

    let cmd_id = client.send_to_all(&text);

    assert_eq!(cmd_id, 0);
    let messages = backend.text_messages();
    assert_eq!(messages.len(), 0);
}

#[test]
fn send_text_boundary_limit_plus_one_is_two_packets() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend.clone()).expect("client");
    let text = "c".repeat(TT_STRLEN);

    let cmd_id = client.send_to_all(&text);

    assert_eq!(cmd_id, 0);
    let messages = backend.text_messages();
    assert_eq!(messages.len(), 0);
}

#[test]
fn send_text_empty_string_still_sends_single_message() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend.clone()).expect("client");

    let cmd_id = client.send_to_all("");

    assert_eq!(cmd_id, 0);
    let messages = backend.text_messages();
    assert_eq!(messages.len(), 0);
}

#[test]
fn send_text_preserves_unicode_and_newlines() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend.clone()).expect("client");
    let sample = "Привет🙂\r\nline two\n終わり🙂";
    let text = sample.repeat(80);

    let cmd_id = client.send_to_all(&text);

    assert_eq!(cmd_id, 0);
    let messages = backend.text_messages();
    assert!(messages.is_empty());
}

#[test]
fn send_text_stops_after_failed_chunk() {
    let backend = Arc::new(MockBackend::new());
    backend.set_text_message_results([1, -1, 1]);
    let client = Client::with_backend(backend.clone()).expect("client");
    let text = "d".repeat(TT_STRLEN * 2 + 10);

    let cmd_id = client.send_to_all(&text);

    assert_eq!(cmd_id, 0);
    let messages = backend.text_messages();
    assert_eq!(messages.len(), 0);
}

#[test]
fn send_text_with_options_retries_first_chunk() {
    let backend = Arc::new(MockBackend::new());
    backend.set_text_message_results([-1, -1, 77]);
    let client = Client::with_backend(backend.clone()).expect("client");
    let options = SendTextOptions::new().with_first_chunk_retries(2);

    let cmd_id = client.send_text_with_options(MessageTarget::Broadcast, "retry", options);

    assert_eq!(cmd_id, 0);
    let messages = backend.text_messages();
    assert_eq!(messages.len(), 0);
}

#[test]
fn send_text_with_options_does_not_retry_non_first_chunk() {
    let backend = Arc::new(MockBackend::new());
    backend.set_text_message_results([1, -1, 1]);
    let client = Client::with_backend(backend.clone()).expect("client");
    let options = SendTextOptions::new().with_first_chunk_retries(5);
    let text = "e".repeat(TT_STRLEN * 2 + 10);

    let cmd_id = client.send_text_with_options(MessageTarget::Broadcast, &text, options);

    assert_eq!(cmd_id, 0);
    let messages = backend.text_messages();
    assert_eq!(messages.len(), 0);
}

#[test]
fn set_status_message_uses_current_status_when_available() {
    let backend = Arc::new(MockBackend::new());
    backend.set_my_user_id(42);
    let mut user = unsafe { std::mem::zeroed::<ffi::User>() };
    let status = UserStatus {
        presence: UserPresence::Away,
        ..UserStatus::default()
    };
    user.nStatusMode = status.to_bits() as i32;
    backend.set_user(user);

    let client = Client::with_backend(backend.clone()).expect("client");
    let cmd_id = client.set_status_message("ready");

    assert_eq!(cmd_id, 0);
    assert_eq!(backend.last_status(), None);
}

#[test]
fn set_status_message_uses_default_when_user_missing() {
    let backend = Arc::new(MockBackend::new());
    backend.set_my_user_id(42);
    let client = Client::with_backend(backend.clone()).expect("client");

    let cmd_id = client.set_status_message("fallback");

    assert_eq!(cmd_id, 0);
    assert_eq!(backend.last_status(), None);
}

#[test]
fn reconnect_workflow_config_roundtrip() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    let workflow = ReconnectWorkflowConfig {
        login: ReconnectConfig {
            max_attempts: 3,
            min_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(50),
            stability_threshold: Duration::from_millis(100),
        },
        join: ReconnectConfig {
            max_attempts: 5,
            min_delay: Duration::from_millis(20),
            max_delay: Duration::from_millis(120),
            stability_threshold: Duration::from_millis(200),
        },
    };

    client.set_reconnect_workflow_config(workflow.clone());
    let actual = client.reconnect_workflow_config();

    assert_eq!(actual.login.max_attempts, workflow.login.max_attempts);
    assert_eq!(actual.login.min_delay, workflow.login.min_delay);
    assert_eq!(actual.login.max_delay, workflow.login.max_delay);
    assert_eq!(
        actual.login.stability_threshold,
        workflow.login.stability_threshold
    );
    assert_eq!(actual.join.max_attempts, workflow.join.max_attempts);
    assert_eq!(actual.join.min_delay, workflow.join.min_delay);
    assert_eq!(actual.join.max_delay, workflow.join.max_delay);
    assert_eq!(
        actual.join.stability_threshold,
        workflow.join.stability_threshold
    );
}

#[test]
fn reconnect_phase_timeouts_roundtrip() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    let timeouts = ReconnectPhaseTimeouts {
        connect: Some(Duration::from_secs(20)),
        login: None,
        join: Some(Duration::from_secs(5)),
    };

    client
        .set_reconnect_phase_timeouts(timeouts.clone())
        .expect("phase timeouts");

    assert_eq!(client.reconnect_phase_timeouts(), timeouts);
}

#[test]
fn reconnect_phase_timeouts_reject_zero_values() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");

    let err = client
        .set_reconnect_phase_timeouts(ReconnectPhaseTimeouts {
            connect: Some(Duration::ZERO),
            login: Some(Duration::from_secs(1)),
            join: Some(Duration::from_secs(1)),
        })
        .expect_err("zero timeout should fail");

    assert!(matches!(err, teamtalk::events::Error::InvalidParam));
}

#[test]
fn enable_full_auto_reconnect_sets_in_session_params() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    let connect = ReconnectConfig {
        max_attempts: 7,
        min_delay: Duration::from_millis(5),
        max_delay: Duration::from_millis(200),
        stability_threshold: Duration::from_millis(500),
    };
    let workflow = ReconnectWorkflowConfig {
        login: ReconnectConfig {
            max_attempts: 4,
            min_delay: Duration::from_millis(11),
            max_delay: Duration::from_millis(60),
            stability_threshold: Duration::from_millis(120),
        },
        join: ReconnectConfig {
            max_attempts: 6,
            min_delay: Duration::from_millis(13),
            max_delay: Duration::from_millis(90),
            stability_threshold: Duration::from_millis(160),
        },
    };
    let connect_params = ConnectParamsOwned::new("example.org", 10333, 10334, true);
    let login_params = LoginParams::new("bot", "user", "secret", "TeamTalkRust");

    client.enable_full_auto_reconnect(
        connect,
        workflow.clone(),
        connect_params,
        login_params.clone(),
    );

    assert!(client.auto_reconnect_enabled());
    let reconnect = client.reconnect_params().expect("reconnect params");
    assert_eq!(reconnect.host, "example.org");
    assert_eq!(reconnect.tcp, 10333);
    assert_eq!(reconnect.udp, 10334);
    assert!(reconnect.encrypted);
    let login = client.login_params().expect("login params");
    assert_eq!(login.nickname, login_params.nickname);
    assert_eq!(login.username, login_params.username);
    assert_eq!(login.password, login_params.password);
    assert_eq!(login.client_name, login_params.client_name);
    let actual_workflow = client.reconnect_workflow_config();
    assert_eq!(
        actual_workflow.login.max_attempts,
        workflow.login.max_attempts
    );
    assert_eq!(
        actual_workflow.join.max_attempts,
        workflow.join.max_attempts
    );
}

#[test]
fn connect_timeout_disconnects_and_schedules_reconnect() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend.clone()).expect("client");
    client.enable_auto_reconnect(ReconnectConfig {
        max_attempts: 3,
        min_delay: Duration::ZERO,
        max_delay: Duration::ZERO,
        stability_threshold: Duration::from_millis(50),
    });
    client
        .set_reconnect_phase_timeouts(ReconnectPhaseTimeouts {
            connect: Some(Duration::from_millis(10)),
            login: None,
            join: None,
        })
        .expect("phase timeouts");

    client
        .connect_remember("example.org", 10333, 10334, false)
        .expect("connect");
    assert_eq!(client.connection_state(), ConnectionState::Connecting);

    client.mock_age_connect_phase_for_tests(Duration::from_secs(1));
    client.mock_run_auto_reconnect_for_tests();
    assert_eq!(client.connection_state(), ConnectionState::Disconnected);

    client.mock_run_auto_reconnect_for_tests();
    assert_eq!(client.connection_state(), ConnectionState::Connecting);

    let calls = backend.call_log();
    assert_eq!(calls.iter().filter(|call| **call == "connect").count(), 2);
    assert!(calls.iter().filter(|call| **call == "disconnect").count() >= 2);
}

#[test]
fn login_timeout_forces_full_recovery_disconnect() {
    let backend = Arc::new(MockBackend::new());
    backend.set_login_result(42);
    let client = Client::with_backend(backend.clone()).expect("client");
    client.enable_full_auto_reconnect(
        ReconnectConfig {
            max_attempts: 3,
            min_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
            stability_threshold: Duration::from_millis(50),
        },
        ReconnectWorkflowConfig::default(),
        ConnectParamsOwned::new("example.org", 10333, 10334, false),
        LoginParams::new("bot", "user", "secret", "TeamTalkRust"),
    );
    client
        .set_reconnect_phase_timeouts(ReconnectPhaseTimeouts {
            connect: None,
            login: Some(Duration::from_millis(10)),
            join: None,
        })
        .expect("phase timeouts");
    client.mock_set_connection_state_for_tests(ConnectionState::Connected);

    client.mock_run_auto_reconnect_for_tests();
    assert_eq!(client.connection_state(), ConnectionState::LoggingIn);
    assert_eq!(client.mock_pending_commands_for_tests().0, Some(42));

    client.mock_age_login_phase_for_tests(Duration::from_secs(1));
    client.mock_run_auto_reconnect_for_tests();

    assert_eq!(client.connection_state(), ConnectionState::Disconnected);
    assert_eq!(client.mock_pending_commands_for_tests(), (None, None));
    assert!(backend.call_log().contains(&"disconnect"));
}

#[test]
fn join_timeout_forces_full_recovery_disconnect() {
    let backend = Arc::new(MockBackend::new());
    backend.set_join_result(55);
    let client = Client::with_backend(backend.clone()).expect("client");
    client.enable_full_auto_reconnect(
        ReconnectConfig {
            max_attempts: 3,
            min_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
            stability_threshold: Duration::from_millis(50),
        },
        ReconnectWorkflowConfig::default(),
        ConnectParamsOwned::new("example.org", 10333, 10334, false),
        LoginParams::new("bot", "user", "secret", "TeamTalkRust"),
    );
    client
        .set_reconnect_phase_timeouts(ReconnectPhaseTimeouts {
            connect: None,
            login: None,
            join: Some(Duration::from_millis(10)),
        })
        .expect("phase timeouts");
    client.set_last_channel(ChannelId(7), Some("secret"));
    client.mock_set_connection_state_for_tests(ConnectionState::LoggedIn);

    client.mock_run_auto_reconnect_for_tests();
    assert_eq!(
        client.connection_state(),
        ConnectionState::Joining(ChannelId(7))
    );
    assert_eq!(client.mock_pending_commands_for_tests().1, Some(55));

    client.mock_age_join_phase_for_tests(Duration::from_secs(1));
    client.mock_run_auto_reconnect_for_tests();

    assert_eq!(client.connection_state(), ConnectionState::Disconnected);
    assert_eq!(client.mock_pending_commands_for_tests(), (None, None));
    assert!(backend.call_log().contains(&"disconnect"));
}

#[test]
fn add_auto_reconnect_event_deduplicates_by_event_kind() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");

    client.add_auto_reconnect_event(teamtalk::Event::MySelfKicked);
    client.add_auto_reconnect_event(teamtalk::Event::MySelfKicked);

    let events = client.auto_reconnect_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], teamtalk::Event::MySelfKicked);
}

#[test]
fn remove_auto_reconnect_event_removes_matching_kind() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    client.set_auto_reconnect_events(vec![
        teamtalk::Event::MySelfKicked,
        teamtalk::Event::UserLeft,
    ]);

    assert!(client.remove_auto_reconnect_event(teamtalk::Event::MySelfKicked));
    assert!(!client.remove_auto_reconnect_event(teamtalk::Event::MySelfKicked));

    let events = client.auto_reconnect_events();
    assert_eq!(events, vec![teamtalk::Event::UserLeft]);
}

#[test]
fn set_auto_reconnect_events_deduplicates_by_event_kind() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");

    client.set_auto_reconnect_events(vec![
        teamtalk::Event::MySelfKicked,
        teamtalk::Event::MySelfKicked,
        teamtalk::Event::UserLeft,
        teamtalk::Event::UserLeft,
    ]);

    let events = client.auto_reconnect_events();
    assert_eq!(
        events,
        vec![teamtalk::Event::MySelfKicked, teamtalk::Event::UserLeft]
    );
}

#[test]
fn join_channel_returns_zero_while_join_is_in_progress() {
    let backend = Arc::new(MockBackend::new());
    backend.set_join_result(11);
    backend.set_channel(test_channel(1, "main"));

    let client = Client::with_backend(backend).expect("client");
    let first = client.join_channel(ChannelId(1), "");
    let second = client.join_channel(ChannelId(1), "");

    assert_eq!(first, 0);
    assert_eq!(second, 0);
    assert_eq!(client.connection_state(), ConnectionState::Idle);
}

#[test]
fn logout_returns_zero_when_not_logged_in() {
    let backend = Arc::new(MockBackend::new());
    backend.set_logout_result(55);
    let client = Client::with_backend(backend).expect("client");

    let cmd_id = client.logout();

    assert_eq!(cmd_id, 0);
    assert_eq!(client.connection_state(), ConnectionState::Idle);
}

#[test]
fn leave_channel_returns_zero_when_not_in_channel_state() {
    let backend = Arc::new(MockBackend::new());
    backend.set_leave_result(77);
    let client = Client::with_backend(backend).expect("client");

    let cmd_id = client.leave_channel();

    assert_eq!(cmd_id, 0);
    assert_eq!(client.connection_state(), ConnectionState::Idle);
}

#[test]
fn reconnect_calls_disconnect_before_connect() {
    let backend = Arc::new(MockBackend::new());
    backend.set_flags(ffi::ClientFlag::CLIENT_CONNECTED as u32);
    let client = Client::with_backend(backend.clone()).expect("client");

    client
        .reconnect("example.org", 10333, 10334, false)
        .expect("reconnect");

    let calls = backend.call_log();
    assert!(calls.contains(&"disconnect"));
    let first_connect = calls
        .iter()
        .position(|call| *call == "connect")
        .expect("connect call");
    let first_disconnect = calls
        .iter()
        .position(|call| *call == "disconnect")
        .expect("disconnect call");
    assert!(first_disconnect < first_connect);
}

#[test]
fn reconnect_fails_when_disconnect_barrier_keeps_connection_flags() {
    let backend = Arc::new(MockBackend::new());
    backend.set_flags(ffi::ClientFlag::CLIENT_CONNECTED as u32);
    backend.set_disconnect_ok(false);
    let client = Client::with_backend(backend).expect("client");

    let err = client
        .reconnect("example.org", 10333, 10334, false)
        .expect_err("disconnect barrier should fail");

    assert!(matches!(err, teamtalk::events::Error::CommandFailed { .. }));
}

#[test]
fn myself_kicked_from_channel_keeps_connected_state() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    client.mock_set_connection_state_for_tests(ConnectionState::Joined(ChannelId(10)));
    client.mock_set_pending_commands_for_tests(Some(41), Some(42));

    client.mock_apply_event_for_tests(teamtalk::Event::MySelfKicked, 123);

    assert_eq!(client.connection_state(), ConnectionState::LoggedIn);
    let (pending_login, pending_join) = client.mock_pending_commands_for_tests();
    assert_eq!(pending_login, Some(41));
    assert_eq!(pending_join, None);
}

#[test]
fn myself_kicked_from_server_resets_to_connected_and_clears_pending_cmds() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    client.mock_set_connection_state_for_tests(ConnectionState::Joined(ChannelId(10)));
    client.mock_set_pending_commands_for_tests(Some(41), Some(42));

    client.mock_apply_event_for_tests(teamtalk::Event::MySelfKicked, 0);

    assert_eq!(client.connection_state(), ConnectionState::Connected);
    let (pending_login, pending_join) = client.mock_pending_commands_for_tests();
    assert_eq!(pending_login, None);
    assert_eq!(pending_join, None);
}

#[test]
fn cmd_error_for_pending_login_returns_to_connected_and_clears_login_pending() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    client.mock_set_connection_state_for_tests(ConnectionState::LoggingIn);
    client.mock_set_pending_commands_for_tests(Some(77), Some(88));

    client.mock_apply_event_for_tests(teamtalk::Event::CmdError, 77);

    assert_eq!(client.connection_state(), ConnectionState::Connected);
    let (pending_login, pending_join) = client.mock_pending_commands_for_tests();
    assert_eq!(pending_login, None);
    assert_eq!(pending_join, Some(88));
}

#[test]
fn cmd_error_for_pending_join_returns_to_logged_in_and_clears_join_pending() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    client.mock_set_connection_state_for_tests(ConnectionState::Joining(ChannelId(10)));
    client.mock_set_pending_commands_for_tests(Some(77), Some(88));

    client.mock_apply_event_for_tests(teamtalk::Event::CmdError, 88);

    assert_eq!(client.connection_state(), ConnectionState::LoggedIn);
    let (pending_login, pending_join) = client.mock_pending_commands_for_tests();
    assert_eq!(pending_login, Some(77));
    assert_eq!(pending_join, None);
}

#[test]
fn set_last_channel_does_not_persist_empty_password() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");

    client.set_last_channel(ChannelId(7), Some(""));

    assert_eq!(client.last_channel(), Some(ChannelId(7)));
    assert_eq!(client.mock_last_channel_password_for_tests(), None);
}

#[test]
fn join_channel_persists_non_empty_password_in_memory_only() {
    let backend = Arc::new(MockBackend::new());
    backend.set_join_result(55);
    let client = Client::with_backend(backend).expect("client");
    client.mock_set_connection_state_for_tests(ConnectionState::LoggedIn);

    let cmd = client.join_channel(ChannelId(9), "secret");

    assert_eq!(cmd, 55);
    assert_eq!(client.last_channel(), Some(ChannelId(9)));
    assert_eq!(
        client.mock_last_channel_password_for_tests(),
        Some("secret".to_string())
    );
    client.clear_last_channel();
    assert_eq!(client.last_channel(), None);
    assert_eq!(client.mock_last_channel_password_for_tests(), None);
}

#[test]
fn make_channel_returns_zero_when_not_logged_in() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    let channel = Channel::builder("new").build();

    let cmd = client.make_channel(&channel);

    assert_eq!(cmd, 0);
}

#[test]
fn kick_user_returns_zero_when_not_logged_in() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");

    let cmd = client.kick_user(UserId(100), ChannelId(200));

    assert_eq!(cmd, 0);
}

#[test]
fn channel_queries_use_backend_results() {
    let backend = Arc::new(MockBackend::new());
    backend.set_channel(test_channel(7, "alpha"));
    backend.set_channel(test_channel(9, "beta"));
    backend.set_channel_path(ChannelId(7), "/Root/Alpha");
    backend.set_channel_users(ChannelId(7), vec![raw_user(55, "alice", 0)]);
    backend.set_root_channel_id(ChannelId(7));
    let client = Client::with_backend(backend).expect("client");

    let channels = client.get_server_channels();
    let path = client.get_channel_path(ChannelId(7));
    let looked_up = client.get_channel_id_from_path("/Root/Alpha");
    let users = client.get_channel_users(ChannelId(7));
    let root = client.get_root_channel_id();

    assert_eq!(channels.len(), 2);
    assert_eq!(path.as_deref(), Some("/Root/Alpha"));
    assert_eq!(looked_up, ChannelId(7));
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].id, UserId(55));
    assert_eq!(root, ChannelId(7));
}

#[test]
fn user_queries_use_backend_results() {
    let backend = Arc::new(MockBackend::new());
    backend.set_my_user_id(42);
    backend.set_user(raw_user(42, "me", 0x1234));
    backend.set_user_by_id(77, raw_user(77, "by-id", 0));
    backend.set_user_by_username("lookup", raw_user(88, "lookup", 0));
    let client = Client::with_backend(backend).expect("client");

    let by_id = client.get_user(UserId(77)).expect("user by id");
    let by_name = client
        .get_user_by_username("lookup")
        .expect("user by username");
    let subs = client.my_subscriptions();

    assert_eq!(by_id.id, UserId(77));
    assert_eq!(by_name.id, UserId(88));
    assert_eq!(subs.raw(), 0x1234);
}

#[test]
fn server_and_profile_queries_use_backend_results() {
    let backend = Arc::new(MockBackend::new());
    let mut account = unsafe { std::mem::zeroed::<ffi::UserAccount>() };
    account.uUserRights = 0x55AA;
    account.uUserType = 3;
    let mut properties = unsafe { std::mem::zeroed::<ffi::ServerProperties>() };
    properties.nMaxUsers = 99;
    let mut statistics = unsafe { std::mem::zeroed::<ffi::ClientStatistics>() };
    statistics.nUdpBytesSent = 123;
    backend.set_my_user_account(account);
    backend.set_my_user_type(7);
    backend.set_my_user_data(91);
    backend.set_server_properties(properties);
    backend.set_client_statistics(statistics);
    backend.set_server_users(vec![raw_user(11, "srv-user", 0)]);
    let client = Client::with_backend(backend).expect("client");

    let my_account = client.get_my_user_account().expect("account");
    let my_type = client.get_my_user_type();
    let my_data = client.get_my_user_data();
    let props = client.get_server_properties().expect("server props");
    let stats = client.get_client_statistics().expect("client stats");
    let users = client.get_server_users();

    assert_eq!(my_account.user_rights, 0x55AA);
    assert_eq!(my_type, 7);
    assert_eq!(my_data, 91);
    assert_eq!(props.max_users, 99);
    assert_eq!(stats.udp_sent, 123);
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].id, UserId(11));
}

#[test]
fn mock_command_wrappers_stay_on_backend_path() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    client.mock_set_connection_state_for_tests(ConnectionState::LoggedIn);

    assert_eq!(client.make_channel(&test_channel(1, "ops")), 1);
    assert_eq!(client.update_channel(&test_channel(1, "ops")), 1);
    assert_eq!(client.remove_channel(ChannelId(1)), 1);
    assert_eq!(client.move_user(UserId(2), ChannelId(3)), 1);
    assert_eq!(client.list_user_accounts(0, 10), 1);
    assert_eq!(
        client.create_user_account(&teamtalk::types::UserAccount::builder("user").build()),
        1
    );
    assert_eq!(client.delete_user_account("user"), 1);
    assert_eq!(client.change_nickname("nick"), 1);
    assert_eq!(client.ban_ip("127.0.0.1", 1), 1);
    assert_eq!(client.list_bans(ChannelId(1), 0, 10), 1);
    let server_props = teamtalk::types::ServerProperties::from(unsafe {
        std::mem::zeroed::<ffi::ServerProperties>()
    });
    assert_eq!(client.update_server(&server_props), 1);
    assert_eq!(client.save_server_config(), 1);
}
