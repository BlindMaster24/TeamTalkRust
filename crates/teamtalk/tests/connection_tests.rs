use rstest::rstest;
use std::time::Duration;

use teamtalk::client::connection::{
    ConnectParamsOwned, ReconnectConfig, ReconnectHandler, ReconnectWorkflowConfig,
};

#[test]
fn reconnect_handler_resets_after_stable_connection() {
    let config = ReconnectConfig {
        max_attempts: 10,
        min_delay: Duration::from_millis(0),
        max_delay: Duration::from_millis(0),
        stability_threshold: Duration::from_millis(0),
    };
    let mut handler = ReconnectHandler::new(config);
    handler.record_attempt();
    handler.record_attempt();
    assert_eq!(handler.attempts(), 2);
    handler.mark_connected();
    handler.mark_disconnected();
    assert_eq!(handler.attempts(), 0);
    assert_eq!(handler.current_delay(), Duration::from_millis(0));
}

#[rstest]
#[case(1, 0, true)]
#[case(1, 1, false)]
#[case(2, 1, true)]
#[case(2, 2, false)]
fn reconnect_handler_respects_max_attempts(
    #[case] max_attempts: u32,
    #[case] recorded_attempts: u32,
    #[case] can_attempt: bool,
) {
    let config = ReconnectConfig {
        max_attempts,
        min_delay: Duration::from_millis(0),
        max_delay: Duration::from_millis(0),
        stability_threshold: Duration::from_millis(0),
    };
    let mut handler = ReconnectHandler::new(config);
    for _ in 0..recorded_attempts {
        handler.record_attempt();
    }
    assert_eq!(handler.can_attempt(), can_attempt);
}

#[test]
fn connect_params_from_env_parses_values() {
    unsafe {
        std::env::set_var("TT_HOST", "example.com");
        std::env::set_var("TT_TCP", "10443");
        std::env::set_var("TT_UDP", "10555");
        std::env::set_var("TT_ENCRYPTED", "true");
    }

    let params = ConnectParamsOwned::from_env();
    assert_eq!(params.host, "example.com");
    assert_eq!(params.tcp, 10443);
    assert_eq!(params.udp, 10555);
    assert!(params.encrypted);

    unsafe {
        std::env::remove_var("TT_HOST");
        std::env::remove_var("TT_TCP");
        std::env::remove_var("TT_UDP");
        std::env::remove_var("TT_ENCRYPTED");
    }
}

#[test]
fn reconnect_workflow_config_defaults_follow_reconnect_defaults() {
    let defaults = ReconnectConfig::default();
    let workflow = ReconnectWorkflowConfig::default();

    assert_eq!(workflow.login.max_attempts, defaults.max_attempts);
    assert_eq!(workflow.login.min_delay, defaults.min_delay);
    assert_eq!(workflow.login.max_delay, defaults.max_delay);
    assert_eq!(
        workflow.login.stability_threshold,
        defaults.stability_threshold
    );
    assert_eq!(workflow.join.max_attempts, defaults.max_attempts);
    assert_eq!(workflow.join.min_delay, defaults.min_delay);
    assert_eq!(workflow.join.max_delay, defaults.max_delay);
    assert_eq!(
        workflow.join.stability_threshold,
        defaults.stability_threshold
    );
}
