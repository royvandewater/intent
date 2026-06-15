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

#[when("I extract the intent")]
fn extract_the_intent(world: &mut IntentWorld) {
    world.output = intent::extract(&world.source);
}

#[then("the output is empty")]
fn the_output_is_empty(world: &mut IntentWorld) {
    assert_eq!(world.output, "");
}

#[tokio::main]
async fn main() {
    IntentWorld::run("tests/features/extract.feature").await;
}
