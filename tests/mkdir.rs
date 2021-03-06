#[macro_use]
mod common;

use common::util::*;

static UTIL_NAME: &'static str = "mkdir";

static TEST_DIR1: &'static str = "mkdir_test1";
static TEST_DIR2: &'static str = "mkdir_test2";
static TEST_DIR3: &'static str = "mkdir_test3";
static TEST_DIR4: &'static str = "mkdir_test4/mkdir_test4_1";
static TEST_DIR5: &'static str = "mkdir_test5/mkdir_test5_1";


#[test]
fn test_mkdir_mkdir() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    let exit_success = ucmd.arg(TEST_DIR1).run().success;
    assert_eq!(exit_success, true);
}

#[test]
fn test_mkdir_dup_dir() {
    let ts = TestSet::new(UTIL_NAME);
    let exit_success = ts.util_cmd().arg(TEST_DIR2).run().success;
    let exit_success2 = ts.util_cmd().arg(TEST_DIR2).run().success;
    assert!(exit_success);
    assert!(!exit_success2);
}

#[test]
fn test_mkdir_mode() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    let exit_success = ucmd.arg("-m")
                           .arg("755")
                           .arg(TEST_DIR3)
                           .run()
                           .success;
    assert!(exit_success);
}

#[test]
fn test_mkdir_parent() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    let exit_success = ucmd.arg("-p").arg(TEST_DIR4).run().success;
    assert!(exit_success);
}

#[test]
fn test_mkdir_no_parent() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    let exit_success = ucmd.arg(TEST_DIR5).run().success;
    assert!(!exit_success);
}
