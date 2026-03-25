use crate::commands::loans::{BorrowItemCommand, LendItemCommand, ReturnItemCommand};
use crate::domain::error::DomainError;
use crate::domain::loan::{Direction, LoanId};
use crate::ports::loan_repository::LoanRepository;
use crate::queries::loans::LoanView;
use chrono::NaiveDate;
use rusqlite::Connection;

pub struct SqliteLoanRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteLoanRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

impl<'a> LoanRepository for SqliteLoanRepository<'a> {
    fn lend(&self, command: &LendItemCommand) -> Result<LoanId, DomainError> {
        self.conn.execute(
            "INSERT INTO loans (person_id, item_id, direction, loan_date) VALUES (?1, ?2, 'lend', ?3)",
            rusqlite::params![
                command.person_id.value(),
                command.item_id.value(),
                command.loan_date.to_string(),
            ],
        ).map_err(|_| DomainError::LoanSaveFailed)?;
        Ok(LoanId::new(self.conn.last_insert_rowid().to_string()))
    }

    fn borrow(&self, command: &BorrowItemCommand) -> Result<LoanId, DomainError> {
        self.conn.execute(
            "INSERT INTO loans (person_id, item_id, direction, loan_date) VALUES (?1, ?2, 'borrow', ?3)",
            rusqlite::params![
                command.person_id.value(),
                command.item_id.value(),
                command.loan_date.to_string(),
            ],
        ).map_err(|_| DomainError::LoanSaveFailed)?;
        Ok(LoanId::new(self.conn.last_insert_rowid().to_string()))
    }

    fn return_item(&self, command: &ReturnItemCommand) -> Result<(), DomainError> {
        self.conn
            .execute(
                "UPDATE loans SET return_date = ?1 WHERE id = ?2",
                rusqlite::params![command.return_date.to_string(), command.loan_id.value(),],
            )
            .map_err(|_| DomainError::LoanUpdateFailed)?;
        Ok(())
    }

    fn find_active(&self) -> Result<Vec<LoanView>, DomainError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT l.id, i.description, p.name, l.direction, l.loan_date \
                 FROM loans l \
                 JOIN items i ON i.id = l.item_id \
                 JOIN people p ON p.id = l.person_id \
                 WHERE l.return_date IS NULL",
            )
            .map_err(|_| DomainError::LoanLoadFailed)?;
        stmt.query_map([], read_view_row)
            .map_err(|_| DomainError::LoanLoadFailed)?
            .map(|r| {
                r.map_err(|_| DomainError::LoanLoadFailed)
                    .and_then(to_loan_view)
            })
            .collect()
    }
}

type ViewRow = (i64, String, String, String, String);

fn read_view_row(row: &rusqlite::Row) -> rusqlite::Result<ViewRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
    ))
}

fn to_loan_view(
    (id, item_description, person_name, direction, loan_date): ViewRow,
) -> Result<LoanView, DomainError> {
    let direction = match direction.as_str() {
        "lend" => Direction::Lend,
        "borrow" => Direction::Borrow,
        _ => return Err(DomainError::InvalidData),
    };
    let loan_date =
        NaiveDate::parse_from_str(&loan_date, "%Y-%m-%d").map_err(|_| DomainError::InvalidData)?;

    Ok(LoanView {
        loan_id: LoanId::new(id.to_string()),
        item_description,
        person_name,
        direction,
        loan_date,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::loans::{BorrowItemCommand, LendItemCommand, ReturnItemCommand};
    use crate::domain::loan::{Direction, LoanId};
    use crate::ports::loan_repository::LoanRepository;
    use chrono::NaiveDate;
    use rusqlite::Connection;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE people (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
            CREATE TABLE items  (id INTEGER PRIMARY KEY, description TEXT NOT NULL);
            CREATE TABLE loans  (
                id          INTEGER PRIMARY KEY,
                person_id   INTEGER NOT NULL REFERENCES people(id),
                item_id     INTEGER NOT NULL REFERENCES items(id),
                direction   TEXT NOT NULL CHECK(direction IN ('lend', 'borrow')),
                loan_date   TEXT NOT NULL,
                return_date TEXT
            );
        ",
        )
        .unwrap();
        conn.execute("INSERT INTO people (name) VALUES (?1)", ["Alice"])
            .unwrap();
        conn.execute("INSERT INTO items (description) VALUES (?1)", ["Drill"])
            .unwrap();
        conn
    }

    fn setup_conn_no_check() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE people (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
            CREATE TABLE items  (id INTEGER PRIMARY KEY, description TEXT NOT NULL);
            CREATE TABLE loans  (
                id          INTEGER PRIMARY KEY,
                person_id   INTEGER NOT NULL,
                item_id     INTEGER NOT NULL,
                direction   TEXT NOT NULL,
                loan_date   TEXT NOT NULL,
                return_date TEXT
            );
        ",
        )
        .unwrap();
        conn.execute("INSERT INTO people (name) VALUES (?1)", ["Alice"])
            .unwrap();
        conn.execute("INSERT INTO items (description) VALUES (?1)", ["Drill"])
            .unwrap();
        conn
    }

    #[test]
    fn find_active_returns_lend_loan_view_with_description_and_name() {
        let conn = setup_conn();
        let repo = SqliteLoanRepository::new(&conn);
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        repo.lend(&LendItemCommand {
            person_id: crate::domain::person::PersonId::new("1"),
            item_id: crate::domain::item::ItemId::new("1"),
            loan_date,
        })
        .unwrap();

        let active = repo.find_active().unwrap();
        assert_eq!(active.len(), 1);
        let view = &active[0];
        assert!(matches!(view.direction, Direction::Lend));
        assert_eq!(view.item_description, "Drill");
        assert_eq!(view.person_name, "Alice");
        assert_eq!(view.loan_date, loan_date);
    }

    #[test]
    fn find_active_returns_borrow_loan_view_with_description_and_name() {
        let conn = setup_conn();
        let repo = SqliteLoanRepository::new(&conn);
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        repo.borrow(&BorrowItemCommand {
            person_id: crate::domain::person::PersonId::new("1"),
            item_id: crate::domain::item::ItemId::new("1"),
            loan_date,
        })
        .unwrap();

        let active = repo.find_active().unwrap();
        assert_eq!(active.len(), 1);
        let view = &active[0];
        assert!(matches!(view.direction, Direction::Borrow));
        assert_eq!(view.item_description, "Drill");
        assert_eq!(view.person_name, "Alice");
        assert_eq!(view.loan_date, loan_date);
    }

    #[test]
    fn lend_returns_loan_save_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqliteLoanRepository::new(&conn);
        assert!(matches!(
            repo.lend(&LendItemCommand {
                person_id: crate::domain::person::PersonId::new("1"),
                item_id: crate::domain::item::ItemId::new("1"),
                loan_date: NaiveDate::from_ymd_opt(2026, 3, 23).unwrap(),
            }),
            Err(DomainError::LoanSaveFailed)
        ));
    }

    #[test]
    fn borrow_returns_loan_save_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqliteLoanRepository::new(&conn);
        assert!(matches!(
            repo.borrow(&BorrowItemCommand {
                person_id: crate::domain::person::PersonId::new("1"),
                item_id: crate::domain::item::ItemId::new("1"),
                loan_date: NaiveDate::from_ymd_opt(2026, 3, 23).unwrap(),
            }),
            Err(DomainError::LoanSaveFailed)
        ));
    }

    #[test]
    fn return_item_returns_loan_update_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqliteLoanRepository::new(&conn);
        assert!(matches!(
            repo.return_item(&ReturnItemCommand {
                loan_id: LoanId::new("1"),
                return_date: NaiveDate::from_ymd_opt(2026, 3, 23).unwrap(),
            }),
            Err(DomainError::LoanUpdateFailed)
        ));
    }

    #[test]
    fn find_active_returns_loan_load_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqliteLoanRepository::new(&conn);
        assert!(matches!(
            repo.find_active(),
            Err(DomainError::LoanLoadFailed)
        ));
    }

    #[test]
    fn lend_inserts_loan_into_db() {
        let conn = setup_conn();
        let repo = SqliteLoanRepository::new(&conn);
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 23).unwrap();

        let id = repo
            .lend(&LendItemCommand {
                person_id: crate::domain::person::PersonId::new("1"),
                item_id: crate::domain::item::ItemId::new("1"),
                loan_date,
            })
            .unwrap();

        let (direction, stored_date, return_date): (String, String, Option<String>) = conn
            .query_row(
                "SELECT direction, loan_date, return_date FROM loans WHERE id = ?1",
                [id.value()],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();
        assert_eq!(direction, "lend");
        assert_eq!(stored_date, "2026-03-23");
        assert!(return_date.is_none());
    }

    #[test]
    fn borrow_inserts_loan_with_borrow_direction() {
        let conn = setup_conn();
        let repo = SqliteLoanRepository::new(&conn);
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 23).unwrap();

        let id = repo
            .borrow(&BorrowItemCommand {
                person_id: crate::domain::person::PersonId::new("1"),
                item_id: crate::domain::item::ItemId::new("1"),
                loan_date,
            })
            .unwrap();

        let direction: String = conn
            .query_row(
                "SELECT direction FROM loans WHERE id = ?1",
                [id.value()],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(direction, "borrow");
    }

    #[test]
    fn return_item_sets_return_date() {
        let conn = setup_conn();
        let repo = SqliteLoanRepository::new(&conn);
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 23).unwrap();
        let return_date = NaiveDate::from_ymd_opt(2026, 3, 30).unwrap();

        let id = repo
            .lend(&LendItemCommand {
                person_id: crate::domain::person::PersonId::new("1"),
                item_id: crate::domain::item::ItemId::new("1"),
                loan_date,
            })
            .unwrap();
        repo.return_item(&ReturnItemCommand {
            loan_id: LoanId::new(id.value()),
            return_date,
        })
        .unwrap();

        let stored_return_date: String = conn
            .query_row(
                "SELECT return_date FROM loans WHERE id = ?1",
                [id.value()],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(stored_return_date, "2026-03-30");
    }

    #[test]
    fn find_active_excludes_returned_loans() {
        let conn = setup_conn();
        let repo = SqliteLoanRepository::new(&conn);
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 23).unwrap();
        let return_date = NaiveDate::from_ymd_opt(2026, 3, 30).unwrap();

        let id = repo
            .lend(&LendItemCommand {
                person_id: crate::domain::person::PersonId::new("1"),
                item_id: crate::domain::item::ItemId::new("1"),
                loan_date,
            })
            .unwrap();
        repo.return_item(&ReturnItemCommand {
            loan_id: LoanId::new(id.value()),
            return_date,
        })
        .unwrap();

        assert!(repo.find_active().unwrap().is_empty());
    }

    #[test]
    fn find_active_returns_invalid_data_on_unknown_direction() {
        let conn = setup_conn_no_check();
        conn.execute(
            "INSERT INTO loans (person_id, item_id, direction, loan_date) VALUES (1, 1, 'invalid', '2026-03-23')",
            [],
        ).unwrap();
        let repo = SqliteLoanRepository::new(&conn);
        assert!(matches!(repo.find_active(), Err(DomainError::InvalidData)));
    }

    #[test]
    fn find_active_returns_invalid_data_on_malformed_loan_date() {
        let conn = setup_conn_no_check();
        conn.execute(
            "INSERT INTO loans (person_id, item_id, direction, loan_date) VALUES (1, 1, 'lend', 'not-a-date')",
            [],
        ).unwrap();
        let repo = SqliteLoanRepository::new(&conn);
        assert!(matches!(repo.find_active(), Err(DomainError::InvalidData)));
    }
}
