//! Main integration test entry point that re-exports the command test modules.

mod command;

// libra-main/tests/command/stats_test.rs

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

use serial_test::serial;

#[test]
#[serial]
fn test_stats_counts_extensions_in_workdir() {
    let dir = tempdir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();

    std::fs::write("hello.txt", "hello").unwrap();
    std::fs::write("world.rs", "fn main(){}").unwrap();
    std::fs::write("no_ext", "data").unwrap();
    std::fs::create_dir(".libra").unwrap();
    std::fs::write(".libra/ignore", "ignored").unwrap();
    std::fs::create_dir("target").unwrap();
    std::fs::write("target/build.log", "ignored").unwrap();

    let mut cmd = Command::cargo_bin("libra").unwrap();
    cmd.arg("stats")
       .arg("run")
       .assert()
       .success()
       .stdout(predicates::str::contains("txt"))   // 实际只有一个 hello.txt
       .stdout(predicates::str::contains("rs"))
       .stdout(predicates::str::contains("no_extension"));
    // 注意：不要断言 json，因为这里没有创建 json 文件
}

#[test]
#[serial]
fn test_stats_json_output() {
    let dir = tempdir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    std::fs::write("a.json", "{}").unwrap();
    std::fs::write("b.txt", "text").unwrap();

    let mut cmd = Command::cargo_bin("libra").unwrap();
    cmd.arg("stats")
       .arg("run")
       .arg("--json-output")
       .assert()
       .success()
       .stdout(predicates::str::contains(r#""extensions": {"#))
       .stdout(predicates::str::contains(r#""total_files": 2"#));  // 只有两个文件
}