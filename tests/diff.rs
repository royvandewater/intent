use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
struct DiffWorld {
    main_source: String,
    branch_source: String,
    diff: String,
}

#[given("the source on main:")]
fn the_source_on_main(world: &mut DiffWorld, step: &cucumber::gherkin::Step) {
    world.main_source = docstring(step);
}

#[given("the source on this branch:")]
fn the_source_on_this_branch(world: &mut DiffWorld, step: &cucumber::gherkin::Step) {
    world.branch_source = docstring(step);
}

#[when("I diff the intent")]
fn diff_the_intent(world: &mut DiffWorld) {
    world.diff = intent::diff_intent(&world.main_source, &world.branch_source, true);
}

#[then(expr = "{string} is marked as added")]
fn marked_as_added(world: &mut DiffWorld, title: String) {
    assert!(
        marked_line(&world.diff, "\x1b[32m", '+', &title),
        "expected {title:?} added (green +) in {:?}",
        world.diff
    );
}

#[then(expr = "{string} is marked as removed")]
fn marked_as_removed(world: &mut DiffWorld, title: String) {
    assert!(
        marked_line(&world.diff, "\x1b[31m", '-', &title),
        "expected {title:?} removed (red -) in {:?}",
        world.diff
    );
}

#[then(expr = "{string} is shown as unchanged")]
fn shown_as_unchanged(world: &mut DiffWorld, title: String) {
    let found = world
        .diff
        .lines()
        .any(|line| !line.contains('\x1b') && line.trim() == title);
    assert!(
        found,
        "expected {title:?} as plain context in {:?}",
        world.diff
    );
}

#[then("the diff is empty")]
fn the_diff_is_empty(world: &mut DiffWorld) {
    assert_eq!(world.diff, "");
}

/// True when some line is colored, carries the given sign, and contains the title.
fn marked_line(diff: &str, color: &str, sign: char, title: &str) -> bool {
    diff.lines()
        .any(|line| line.starts_with(color) && line.contains(sign) && line.contains(title))
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
    DiffWorld::run("tests/features/diff.feature").await;
}
