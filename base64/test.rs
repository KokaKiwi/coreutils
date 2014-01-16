use std::{run, str};

#[test]
fn test_encode() {
    let prog = run::process_output("build/base64", [~"base64/fixtures/data1.txt"]).unwrap();
    let out = str::from_utf8_owned(prog.output);

    assert_eq!(out, ~"SGVsbG8gd29ybGQhClRoaXMgaXMgYSB0ZXN0Lgo=\n");
}

// The Rust base64 implementation doesn't handle properly line wrapping. :(
/*
#[test]
fn test_encode_wrap()
{
    let prog = run::process_output("build/base64", [~"-w", ~"10", ~"base64/fixtures/data1.txt"]).unwrap();
    let out = str::from_utf8_owned(prog.output);

    assert_eq!(out, ~"SGVsbG8gd2\n9ybGQhClRo\naXMgaXMgYS\nB0ZXN0Lgo=\n");
}
*/

#[test]
fn test_decode() {
    let prog = run::process_output("build/base64", [~"-d", ~"base64/fixtures/data2.txt"]).unwrap();
    let out = str::from_utf8_owned(prog.output);

    assert_eq!(out, ~"418: I'm a teapot.\n");
}
