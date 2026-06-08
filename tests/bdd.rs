use cucumber::{World, given, then};

#[derive(Debug, Default, World)]
struct AppWorld;

#[given("the app exists")]
async fn the_app_exists(_world: &mut AppWorld) {}

#[then("it works")]
async fn it_works(_world: &mut AppWorld) {}

#[tokio::main]
async fn main() {
    AppWorld::run("features/").await;
}
