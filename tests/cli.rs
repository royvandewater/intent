use std::path::PathBuf;
use std::process::Command;

use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
struct CliWorld {
    source: String,
    repo: PathBuf,
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

#[given(expr = "a git repository containing {string}:")]
fn a_git_repository_containing(world: &mut CliWorld, path: String, step: &cucumber::gherkin::Step) {
    world.repo = std::env::temp_dir().join(format!("intent_cli_repo_{}", std::process::id()));
    std::fs::remove_dir_all(&world.repo).ok();

    let file = world.repo.join(&path);
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, docstring(step)).unwrap();

    let status = Command::new("git")
        .args(["init"])
        .current_dir(&world.repo)
        .output()
        .unwrap()
        .status;
    assert!(status.success());
}

#[when(expr = "I run intent on {string} from a subdirectory of that repository")]
fn run_intent_from_subdirectory(world: &mut CliWorld, path: String) {
    let subdirectory = world.repo.join("nested/deeper");
    std::fs::create_dir_all(&subdirectory).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_intent"))
        .arg(&path)
        .current_dir(&subdirectory)
        .output()
        .unwrap();

    std::fs::remove_dir_all(&world.repo).unwrap();
    world.success = output.status.success();
    world.stdout = String::from_utf8_lossy(&output.stdout).into_owned();
}

#[given(expr = "a git repository whose main branch has {string}:")]
fn a_git_repository_whose_main_has(
    world: &mut CliWorld,
    path: String,
    step: &cucumber::gherkin::Step,
) {
    world.repo = std::env::temp_dir().join(format!("intent_cli_diff_repo_{}", std::process::id()));
    std::fs::remove_dir_all(&world.repo).ok();
    std::fs::create_dir_all(&world.repo).unwrap();

    git(&world.repo, &["init", "-b", "main"]);
    git(&world.repo, &["config", "user.email", "test@example.com"]);
    git(&world.repo, &["config", "user.name", "Test"]);
    commit(&world.repo, &path, &docstring(step));
}

#[given(expr = "this branch changed {string} to:")]
fn this_branch_changed(world: &mut CliWorld, path: String, step: &cucumber::gherkin::Step) {
    git(&world.repo, &["switch", "-c", "feature"]);
    commit(&world.repo, &path, &docstring(step));
}

#[given(expr = "main then changed {string} to:")]
fn main_then_changed(world: &mut CliWorld, path: String, step: &cucumber::gherkin::Step) {
    git(&world.repo, &["switch", "main"]);
    commit(&world.repo, &path, &docstring(step));
    git(&world.repo, &["switch", "feature"]);
}

#[when(expr = "I run intent with {string} in that repository")]
fn run_intent_with_in_repository(world: &mut CliWorld, arg: String) {
    let output = Command::new(env!("CARGO_BIN_EXE_intent"))
        .arg(&arg)
        .env("NO_COLOR", "1")
        .current_dir(&world.repo)
        .output()
        .unwrap();

    std::fs::remove_dir_all(&world.repo).unwrap();
    world.success = output.status.success();
    world.stdout = String::from_utf8_lossy(&output.stdout).into_owned();
}

fn git(repo: &PathBuf, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn commit(repo: &PathBuf, path: &str, contents: &str) {
    std::fs::write(repo.join(path), contents).unwrap();
    git(repo, &["add", path]);
    git(repo, &["commit", "-m", "update"]);
}

#[when(expr = "I run intent with {string}")]
fn run_intent_with(world: &mut CliWorld, arg: String) {
    let output = Command::new(env!("CARGO_BIN_EXE_intent"))
        .arg(&arg)
        .output()
        .unwrap();

    world.success = output.status.success();
    world.stdout = String::from_utf8_lossy(&output.stdout).into_owned();
}

#[then("it exits successfully")]
fn it_exits_successfully(world: &mut CliWorld) {
    assert!(world.success);
}

#[then("the output describes the usage of intent")]
fn the_output_describes_usage(world: &mut CliWorld) {
    assert!(
        world.stdout.contains("Usage:")
            && world.stdout.contains("--diff")
            && world.stdout.contains("--help"),
        "expected usage information, got {:?}",
        world.stdout
    );
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
