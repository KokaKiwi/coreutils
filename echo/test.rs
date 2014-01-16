#[feature(macro_rules)];

use std::{run, str};

macro_rules! run_command(
    ($name: expr) => {
        {
            let prog = run::process_output($name, []).unwrap();
            str::from_utf8_owned(prog.output)
        }
    };
    ($name: expr, $($arg:expr),+) => {
        {
            let prog = run::process_output($name, [$(~$arg,)+]).unwrap();
            str::from_utf8_owned(prog.output)
        }
    };
)

#[test]
fn test_simple()
{
    let out = run_command!("build/echo", "hello world!");
    assert_eq!(out, ~"hello world!\n");
}

#[test]
fn test_no_newline()
{
    let out = run_command!("build/echo", "-n", "hello world!");
    assert_eq!(out, ~"hello world!");
}

#[test]
fn test_escapes()
{
    let out = run_command!("build/echo", "-e", "escape: \\x02");
    assert_eq!(out, ~"escape: \x02\n");
    let out = run_command!("build/echo", "-E", "escape: \\x02");
    assert_eq!(out, ~"escape: \\x02\n");
}
