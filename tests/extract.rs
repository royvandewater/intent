use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
struct IntentWorld {
    source: String,
    output: String,
}

#[given("an empty source")]
fn an_empty_source(world: &mut IntentWorld) {
    world.source = String::new();
}

#[given("the source:")]
fn the_source(world: &mut IntentWorld, step: &cucumber::gherkin::Step) {
    world.source = docstring(step);
}

#[when("I extract the intent")]
fn extract_the_intent(world: &mut IntentWorld) {
    world.output = intent::extract(&world.source);
}

#[when("I extract the colored intent")]
fn extract_the_colored_intent(world: &mut IntentWorld) {
    world.output = intent::extract_colored(&world.source);
}

#[then(expr = "{string} is shown as a describe block")]
fn shown_as_describe_block(world: &mut IntentWorld, title: String) {
    let expected = format!("\x1b[1m{title}\x1b[0m");
    assert!(
        world.output.contains(&expected),
        "expected {expected:?} in {:?}",
        world.output
    );
}

#[then(expr = "{string} is shown as a passing test")]
fn shown_as_passing_test(world: &mut IntentWorld, title: String) {
    let expected = format!("\x1b[32m{title}\x1b[0m");
    assert!(
        world.output.contains(&expected),
        "expected {expected:?} in {:?}",
        world.output
    );
}

#[then("the output is empty")]
fn the_output_is_empty(world: &mut IntentWorld) {
    assert_eq!(world.output, "");
}

#[then("the output is:")]
fn the_output_is(world: &mut IntentWorld, step: &cucumber::gherkin::Step) {
    assert_eq!(world.output, docstring(step));
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
    IntentWorld::run("tests/features/extract.feature").await;
}
