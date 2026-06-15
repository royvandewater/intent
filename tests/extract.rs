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
