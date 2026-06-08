use cucumber::{World, given, then, when};
use expectrl::session::OsSession;
use expectrl::Expect;
use std::env;
use std::process::Command;

fn expect_or_panic(session: &mut OsSession, pattern: &str) {
    if let Err(e) = session.expect(pattern) {
        let mut buf = [0u8; 4096];
        let n = session.try_read(&mut buf).unwrap_or(0);
        panic!(
            "expected {:?}, but got:\n{}\nerror: {}",
            pattern,
            String::from_utf8_lossy(&buf[..n]),
            e
        );
    }
}

#[derive(Debug, World)]
struct AppWorld {
    session: Option<OsSession>,
}

impl Default for AppWorld {
    fn default() -> Self {
        Self { session: None }
    }
}

#[given("the app is running")]
async fn app_is_running(world: &mut AppWorld) {
    let exe = env::var("CARGO_BIN_EXE_borrowr").expect("CARGO_BIN_EXE_borrowr not set");
    let db_path = std::env::temp_dir().join(format!("borrowr_test_{}.db", std::process::id()));
    let mut cmd = Command::new(&exe);
    cmd.env("BORROWR_DB", db_path);
    let session = OsSession::spawn(cmd).expect("failed to spawn borrowr");
    world.session = Some(session);
}

#[then("the main menu is displayed")]
async fn main_menu_is_displayed(world: &mut AppWorld) {
    expect_or_panic(world.session.as_mut().unwrap(), "What would you like to do?");
}

#[when("the user selects Quit")]
async fn user_selects_quit(world: &mut AppWorld) {
    world
        .session
        .as_mut()
        .unwrap()
        .send_line("Quit")
        .unwrap();
}

#[then("the app exits successfully")]
async fn app_exits_successfully(world: &mut AppWorld) {
    world
        .session
        .as_mut()
        .unwrap()
        .expect(expectrl::Eof)
        .unwrap();
}

#[tokio::main]
async fn main() {
    AppWorld::run("features/").await;
}
