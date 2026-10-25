use std::process::Command;

#[test]
fn fails_when_arguments_are_missing() {
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .env_remove("SHOW_LINE_NUMBERS")
        .output()
        .expect("应能启动 minigrep");
    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("错误输出应为 UTF-8");
    assert!(stderr.contains("Problem parsing arguments"));
    assert!(stderr.contains("missing query"));
}

#[test]
fn prints_matching_lines() {
    let file_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("poem.txt");
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .arg("nobody")
        .arg(file_path)
        .env_remove("SHOW_LINE_NUMBERS")
        .env_remove("IGNORE_CASE")
        .output()
        .expect("应能启动 minigrep");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).expect("输出应为 UTF-8");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(
        lines,
        vec!["I'm nobody! Who are you?", "Are you nobody, too?"]
    );
}

#[test]
fn ignores_case_when_environment_variable_is_set() {
    let file_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("poem.txt");
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .arg("NOBODY")
        .arg(file_path)
        .env("IGNORE_CASE", "1")
        .env_remove("SHOW_LINE_NUMBERS")
        .output()
        .expect("应能启动 minigrep");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).expect("输出应为 UTF-8");
    assert!(stdout.contains("I'm nobody! Who are you?"));
    assert!(stdout.contains("Are you nobody, too?"));
    assert!(!stdout.contains("How dreary to be somebody"));
}

#[test]
fn respects_case_when_environment_variable_is_absent() {
    let file_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("poem.txt");
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .arg("NOBODY")
        .arg(file_path)
        .env_remove("IGNORE_CASE")
        .env_remove("SHOW_LINE_NUMBERS")
        .output()
        .expect("应能启动 minigrep");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).expect("输出应为 UTF-8");
    assert!(stdout.is_empty());
}

#[test]
fn fails_when_file_does_not_exist() {
    let file_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/nonexistent-file.txt");
    assert!(!file_path.exists(), "测试要求这个路径不存在");
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .arg("nobody")
        .arg(file_path)
        .env_remove("IGNORE_CASE")
        .env_remove("SHOW_LINE_NUMBERS")
        .output()
        .expect("应能启动 minigrep");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).expect("输出应为 UTF-8");
    assert!(stderr.contains("Application error:"));
}

#[test]
fn prints_line_numbers_when_enabled() {
    let file_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("poem.txt");
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .arg("nobody")
        .arg(file_path)
        .env("SHOW_LINE_NUMBERS", "1")
        .env_remove("IGNORE_CASE")
        .output()
        .expect("应能启动 minigrep");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).expect("输出应为 UTF-8");

    assert_eq!(
        stdout.lines().collect::<Vec<_>>(),
        vec!["1:I'm nobody! Who are you?", "3:Are you nobody, too?"]
    );
}

#[test]
fn prints_line_numbers_when_ignoring_case() {
    let file_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("poem.txt");
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .arg("NOBODY")
        .arg(file_path)
        .env("SHOW_LINE_NUMBERS", "1")
        .env("IGNORE_CASE", "1")
        .output()
        .expect("应能启动 minigrep");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).expect("输出应为 UTF-8");

    assert_eq!(
        stdout.lines().collect::<Vec<_>>(),
        vec!["1:I'm nobody! Who are you?", "3:Are you nobody, too?"]
    );
}
