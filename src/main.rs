use borrowr::adapters::storage::item_repository::SqliteItemRepository;
use borrowr::adapters::storage::loan_repository::SqliteLoanRepository;
use borrowr::adapters::storage::person_repository::SqlitePersonRepository;
use rusqlite::Connection;

fn main() {
    let path = borrowr::config::db_path(
        std::env::var("BORROWR_DB").ok(),
        std::env::var("HOME").expect("HOME not set"),
    );
    borrowr::config::ensure_data_dir(&path).expect("failed to create data directory");

    let item_conn = Connection::open(&path).expect("failed to open database");
    let person_conn = Connection::open(&path).expect("failed to open database");
    let loan_conn = Connection::open(&path).expect("failed to open database");

    item_conn
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS items (id INTEGER PRIMARY KEY, description TEXT NOT NULL);",
        )
        .expect("failed to initialize schema");
    person_conn
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS people (id INTEGER PRIMARY KEY, name TEXT NOT NULL);",
        )
        .expect("failed to initialize schema");
    loan_conn
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS loans (
            id          INTEGER PRIMARY KEY,
            person_id   INTEGER NOT NULL REFERENCES people(id),
            item_id     INTEGER NOT NULL REFERENCES items(id),
            direction   TEXT NOT NULL CHECK(direction IN ('lend', 'borrow')),
            loan_date   TEXT NOT NULL,
            return_date TEXT
        );",
        )
        .expect("failed to initialize schema");

    let item_repo = SqliteItemRepository::new(item_conn);
    let person_repo = SqlitePersonRepository::new(person_conn);
    let loan_repo = SqliteLoanRepository::new(loan_conn);
    if let Err(e) = borrowr::adapters::cli::run(&borrowr::adapters::cli::Ctx::new(
        &item_repo,
        &person_repo,
        &loan_repo,
    )) {
        eprintln!("{}", e.message);
        std::process::exit(e.code);
    }
}
