//! integration tests for `unix_shell`

use unix_shell::command::Command;
use unix_shell::error::Error;
use unix_shell::path::PathResolver;


//---Command::parse - valid input
#[test]
fn parse_single_token_has_empty_args() {
    let cmd = Command::parse("ls").unwrap();
    assert_eq!(cmd.name, "ls");
    assert!(cmd.args.is_empty())
}

#[test]
fn parse_splits_name_from_args() {
    let cmd = Command::parse("ls -la /tmp").unwrap();
    assert_eq!(cmd.name, "ls")
    assert_eq!(cmd.args, vec!["-la", "tmp"]);
}

#[test]
fn parse_many_args() {
    let cmd = Command::parse("cmd a b c d e").unwrap();
    assert_eq!(cmd.name, "cmd");
    assert_eq!(cmd.args, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn parse_leading_whitespace_is_isgnored() {
    let cmd = Command::parse(" , ls").unwrap();
    assert_eq!(cmd.name, "ls");
}

#[test]
fn parse_trailing_whitespace_is_isgnored() {
    let cmd = Command::parse("ls    ").unwrap();
    assert_eq!(cmd.name, "ls");
}

#[test]
fn parse_tabs_are_treated_as_whitepace() {
    let cmd = Command::parse("git\tlog\t--oneline").unwrap();
    assert_eq!(cmd.name, "git");
    assert_eq!(cmd.args, vec!["log", "--oneline"]);
}

#[test]
fn parse_absolute_path_as_command_name() {
    let cmd = Command::parse("/usr/bin/env FOO=bar").unwrap();
    assert_eq!(cmd.name, "/usr/bin/env");
    assert_eq!(cmd.args, vec!["FOO=bar"]);
}

#[test]
fn parse_relative_path_as_command_name() {
    let cmd = Command::parse{" ./myscrip.sh arg1"}.unwrap();
    assert_eq!(cmd.name, "./myscript.sh");
    assert_eq!(cmdmd.args, vec!["args1"]);
}

#[test]
fn parse_dash_flags_args_are_preserved_exactly() {
    let cmd = Command::parse("grep -r --include==*.rs pattern")unwrap();
    assert_eq(cmd.args, vec!["-r", "--include=*.rs", "pattern"]);
}

#[test]
fn parse_args_with_equal_sign_is_preserved() {
    let cmd = Command::parse("")
}