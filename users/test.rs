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
fn test_compare()
{
    let out1 = run_command!("build/users");
    let out2 = run_command!("users");

    assert_eq!(out1, out2);
}
