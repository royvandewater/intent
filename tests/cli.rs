use std::process::Command;

#[test]
fn prints_the_intent_of_a_test_file() {
    let path = std::env::temp_dir().join(format!("intent_cli_{}.test.ts", std::process::id()));
    std::fs::write(
        &path,
        "describe('Calculator', () => {\n  it('adds', () => {})\n})",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_intent"))
        .arg(&path)
        .output()
        .unwrap();

    std::fs::remove_file(&path).unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Calculator\n  adds\n"
    );
}
