#![cfg(feature = "bot")]
//! Additional coverage for `Command` parsing, `CommandPattern` matching, and
//! `Args` helpers beyond the baseline in `bot_primitives.rs`.

use teamtalk::{Args, CommandPattern, parse_command};

#[test]
fn parse_command_accepts_multiple_prefix_variants() {
    let slash = parse_command("/ping", &['/', '!']).expect("slash prefix");
    assert_eq!(slash.prefix, '/');
    assert_eq!(slash.name, "ping");

    let bang = parse_command("!ping", &['/', '!']).expect("bang prefix");
    assert_eq!(bang.prefix, '!');
    assert_eq!(bang.name, "ping");
}

#[test]
fn parse_command_rejects_empty_input_and_prefix_only() {
    assert!(parse_command("", &['/']).is_none());
    // A bare prefix without a name is not a command.
    assert!(parse_command("/", &['/']).is_none());
}

#[test]
fn parse_command_preserves_argument_order_and_count() {
    let cmd = parse_command("/kick alice spammer bye", &['/']).unwrap();
    assert_eq!(cmd.name, "kick");
    assert_eq!(cmd.args, vec!["alice", "spammer", "bye"]);
    assert_eq!(cmd.arg(0), Some("alice"));
    assert_eq!(cmd.arg(99), None);
}

#[test]
fn parse_command_collapses_extra_whitespace_between_tokens() {
    let cmd = parse_command("/say   hello     world", &['/']).unwrap();
    assert_eq!(cmd.name, "say");
    assert_eq!(cmd.args, vec!["hello", "world"]);
}

#[test]
fn command_pattern_parse_rejects_empty_input() {
    assert!(CommandPattern::parse("").is_err());
    assert!(CommandPattern::parse("   ").is_err());
}

#[test]
fn command_pattern_parse_rejects_required_after_optional() {
    assert!(CommandPattern::parse("ban [reason] <user>").is_err());
}

#[test]
fn command_pattern_parse_rejects_variadic_not_last() {
    assert!(CommandPattern::parse("say <words...> <extra>").is_err());
}

#[test]
fn command_pattern_parse_rejects_command_token_after_args() {
    assert!(CommandPattern::parse("ban <user> more-command").is_err());
}

#[test]
fn command_pattern_min_and_max_args_for_fixed_arity() {
    let pat = CommandPattern::parse("kick <user> [reason]").unwrap();
    assert_eq!(pat.min_args(), 1);
    assert_eq!(pat.max_args(), Some(2));
}

#[test]
fn command_pattern_max_args_none_for_variadic_tail() {
    let pat = CommandPattern::parse("say <message...>").unwrap();
    assert_eq!(pat.min_args(), 1);
    assert_eq!(pat.max_args(), None);
}

#[test]
fn command_pattern_accepts_validates_arg_counts() {
    let pat = CommandPattern::parse("move <user> [channel]").unwrap();
    assert!(!pat.accepts(&[]));
    assert!(pat.accepts(&["alice".into()]));
    assert!(pat.accepts(&["alice".into(), "lobby".into()]));
    assert!(!pat.accepts(&["alice".into(), "lobby".into(), "extra".into()]));
}

#[test]
fn command_pattern_accepts_unbounded_for_variadic_tail() {
    let pat = CommandPattern::parse("echo <msg...>").unwrap();
    for n in 1..10 {
        let args: Vec<String> = (0..n).map(|i| i.to_string()).collect();
        assert!(pat.accepts(&args), "accepts must be true for {n} args");
    }
}

#[test]
fn command_pattern_args_descriptors_expose_required_and_variadic_flags() {
    let pat = CommandPattern::parse("kick <user> [reason...]").unwrap();
    let args = pat.args();
    assert_eq!(args.len(), 2);
    assert!(args[0].required());
    assert!(!args[0].variadic());
    assert!(!args[1].required());
    assert!(args[1].variadic());
}

#[test]
fn command_pattern_command_parts_support_multi_word_command_names() {
    let pat = CommandPattern::parse("admin ban <user>").unwrap();
    assert_eq!(pat.command_parts(), &["admin", "ban"]);
    assert_eq!(pat.command(), "admin ban");
    assert_eq!(pat.usage(), "admin ban <user>");
}

#[test]
fn command_pattern_usage_strings_include_bracketing_conventions() {
    let pat = CommandPattern::parse("kick <user> [reason]").unwrap();
    assert_eq!(pat.usage(), "kick <user> [reason]");
    assert_eq!(pat.usage_with_prefix('!'), "!kick <user> [reason]");
}

#[test]
fn args_raw_returns_original_slice_values() {
    let raw = vec!["alpha".to_owned(), "beta".to_owned()];
    let args = Args::new(&raw);
    assert_eq!(args.raw(0), Some("alpha"));
    assert_eq!(args.raw(1), Some("beta"));
    assert_eq!(args.raw(2), None);
}

#[test]
fn args_rest_returns_none_past_end() {
    let raw = vec!["a".to_owned()];
    let args = Args::new(&raw);
    assert_eq!(args.rest(5), None);
    assert_eq!(args.rest(0).as_deref(), Some("a"));
}

#[test]
fn args_get_reports_parse_errors_and_missing_values() {
    let raw = vec!["fifty".to_owned()];
    let args = Args::new(&raw);
    // Present but not numeric → Err.
    assert!(args.get::<u32>(0).is_err());
    // Missing index → Ok(None).
    assert!(matches!(args.get::<u32>(1), Ok(None)));
}

#[test]
fn args_all_returns_underlying_slice() {
    let raw = vec!["x".to_owned(), "y".to_owned()];
    let args = Args::new(&raw);
    assert_eq!(args.all(), raw.as_slice());
}
