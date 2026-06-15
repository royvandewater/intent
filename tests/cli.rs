use std::process::Command;

use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
struct CliWorld {
    source: String,
    success: bool,
    stdout: String,
}

#[given("a test file containing:")]
fn a_test_file_containing(world: &mut CliWorld, step: &cucumber::gherkin::Step) {
    world.source = docstring(step);
}

#[when("I run intent on that file")]
fn run_intent_on_that_file(world: &mut CliWorld) {
    let path = std::env::temp_dir().join(format!("intent_cli_{}.test.ts", std::process::id()));
    std::fs::write(&path, &world.source).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_intent"))
        .arg(&path)
        .output()
        .unwrap();

    std::fs::remove_file(&path).unwrap();
    world.success = output.status.success();
    world.stdout = String::from_utf8_lossy(&output.stdout).into_owned();
}

#[then("it exits successfully")]
fn it_exits_successfully(world: &mut CliWorld) {
    assert!(world.success);
}

#[then("it prints:")]
fn it_prints(world: &mut CliWorld, step: &cucumber::gherkin::Step) {
    assert_eq!(world.stdout.trim_end(), docstring(step));
}

/// The `gherkin` crate wraps doc string content in surrounding newlines;
/// trim them so feature files read naturally.
fn docstring(step: &cucumber::gherkin::Step) -> String {
    step.docstring()
        .map(|text| text.trim().to_string())
        .unwrap_or_default()
}

#[tokio::main]
async fn main() {
    CliWorld::run("tests/features/cli.feature").await;
}
