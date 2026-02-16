#![cfg(feature = "mock")]

use std::sync::Arc;
use std::time::Duration;

use teamtalk::client::Client;
use teamtalk::client::backend::MockBackend;
use teamtalk::client::connection::{ConnectParamsOwned, ReconnectConfig, ReconnectWorkflowConfig};
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
