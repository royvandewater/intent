use cucumber::{World, given, then};

#[derive(Debug, Default, World)]
struct FilterWorld {
    path: String,
    filters: Vec<String>,
}

#[given(expr = "the changed path {string}")]
fn the_changed_path(world: &mut FilterWorld, path: String) {
    world.path = path;
}

#[given("no filename filters")]
fn no_filename_filters(world: &mut FilterWorld) {
    world.filters.clear();
}

#[given(expr = "the filename filter {string}")]
fn the_filename_filter(world: &mut FilterWorld, filter: String) {
    world.filters.push(filter);
}

#[then("the path is shown")]
fn the_path_is_shown(world: &mut FilterWorld) {
    assert!(
        intent::path_matches(&world.path, &world.filters),
        "expected {:?} to match {:?}",
        world.path,
        world.filters
    );
}

#[then("the path is hidden")]
fn the_path_is_hidden(world: &mut FilterWorld) {
    assert!(
        !intent::path_matches(&world.path, &world.filters),
        "expected {:?} not to match {:?}",
        world.path,
        world.filters
    );
}

#[tokio::main]
async fn main() {
    FilterWorld::run("tests/features/filter.feature").await;
}
