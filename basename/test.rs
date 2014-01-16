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
fn test_simple() {
    let out = run_command!("build/basename", "basename/basename.rs");
    assert_eq!(out, ~"basename.rs\n");
}
