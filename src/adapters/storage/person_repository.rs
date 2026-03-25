use crate::commands::people::{AddPersonCommand, UpdatePersonCommand};
use crate::domain::error::DomainError;
use crate::domain::person::{Person, PersonId};
use crate::ports::person_repository::PersonRepository;
use rusqlite::Connection;

pub struct SqlitePersonRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SqlitePersonRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

impl<'a> PersonRepository for SqlitePersonRepository<'a> {
    fn add(&self, command: &AddPersonCommand) -> Result<PersonId, DomainError> {
        self.conn
            .execute("INSERT INTO people (name) VALUES (?1)", [&command.name])
            .map_err(|_| DomainError::PersonSaveFailed)?;
        Ok(PersonId::new(self.conn.last_insert_rowid().to_string()))
    }

    fn update(&self, command: &UpdatePersonCommand) -> Result<(), DomainError> {
        self.conn
            .execute(
                "UPDATE people SET name = ?1 WHERE id = ?2",
                [&command.name, command.id.value()],
            )
            .map_err(|_| DomainError::PersonUpdateFailed)?;
        Ok(())
    }

    fn find_by_id(&self, id: &PersonId) -> Result<Person, DomainError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM people WHERE id = ?1")
            .map_err(|_| DomainError::PersonLoadFailed)?;
        let mut rows = stmt
            .query_map([id.value()], map_row)
            .map_err(|_| DomainError::PersonLoadFailed)?;
        match rows.next() {
            Some(row) => row.map_err(|_| DomainError::PersonLoadFailed),
            None => Err(DomainError::PersonNotFound),
        }
    }

    fn find_all(&self) -> Result<Vec<Person>, DomainError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM people")
            .map_err(|_| DomainError::PersonLoadFailed)?;
        stmt.query_map([], map_row)
            .map_err(|_| DomainError::PersonLoadFailed)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| DomainError::PersonLoadFailed)
    }

    fn remove(&self, id: &PersonId) -> Result<(), DomainError> {
        self.conn
            .execute("DELETE FROM people WHERE id = ?1", [id.value()])
            .map_err(|_| DomainError::PersonDeleteFailed)?;
        Ok(())
    }
}

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<Person> {
    let id: i64 = row.get(0)?;
    let name: String = row.get(1)?;
    Ok(Person {
        id: PersonId::new(id.to_string()),
        name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::people::{AddPersonCommand, UpdatePersonCommand};
    use crate::ports::person_repository::PersonRepository;
    use rusqlite::Connection;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("CREATE TABLE people (id INTEGER PRIMARY KEY, name TEXT NOT NULL);")
            .unwrap();
        conn
    }

    #[test]
    fn add_returns_person_save_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqlitePersonRepository::new(&conn);
        assert!(matches!(
            repo.add(&AddPersonCommand {
                name: "Alice".to_string()
            }),
            Err(DomainError::PersonSaveFailed)
        ));
    }

    #[test]
    fn update_returns_person_update_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqlitePersonRepository::new(&conn);
        assert!(matches!(
            repo.update(&UpdatePersonCommand {
                id: PersonId::new("1"),
                name: "Alicia".to_string()
            }),
            Err(DomainError::PersonUpdateFailed)
        ));
    }

    #[test]
    fn find_by_id_returns_person_load_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqlitePersonRepository::new(&conn);
        assert!(matches!(
            repo.find_by_id(&PersonId::new("1")),
            Err(DomainError::PersonLoadFailed)
        ));
    }

    #[test]
    fn find_all_returns_person_load_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqlitePersonRepository::new(&conn);
        assert!(matches!(
            repo.find_all(),
            Err(DomainError::PersonLoadFailed)
        ));
    }

    #[test]
    fn remove_returns_person_delete_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqlitePersonRepository::new(&conn);
        assert!(matches!(
            repo.remove(&PersonId::new("1")),
            Err(DomainError::PersonDeleteFailed)
        ));
    }

    #[test]
    fn add_inserts_person_into_db() {
        let conn = setup_conn();
        let repo = SqlitePersonRepository::new(&conn);
        let id = repo
            .add(&AddPersonCommand {
                name: "Alice".to_string(),
            })
            .unwrap();
        let name: String = conn
            .query_row(
                "SELECT name FROM people WHERE id = ?1",
                [id.value()],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(name, "Alice");
    }

    #[test]
    fn find_by_id_returns_matching_person() {
        let conn = setup_conn();
        conn.execute("INSERT INTO people (name) VALUES (?1)", ["Alice"])
            .unwrap();
        conn.execute("INSERT INTO people (name) VALUES (?1)", ["Bob"])
            .unwrap();
        let repo = SqlitePersonRepository::new(&conn);

        let person = repo.find_by_id(&PersonId::new("2")).unwrap();
        assert_eq!(person.id.value(), "2");
        assert_eq!(person.name, "Bob");
    }

    #[test]
    fn find_by_id_returns_person_not_found_when_missing() {
        let conn = setup_conn();
        let repo = SqlitePersonRepository::new(&conn);
        assert!(matches!(
            repo.find_by_id(&PersonId::new("99")),
            Err(DomainError::PersonNotFound)
        ));
    }

    #[test]
    fn find_all_returns_all_people() {
        let conn = setup_conn();
        conn.execute("INSERT INTO people (name) VALUES (?1)", ["Alice"])
            .unwrap();
        conn.execute("INSERT INTO people (name) VALUES (?1)", ["Bob"])
            .unwrap();
        let repo = SqlitePersonRepository::new(&conn);

        let people = repo.find_all().unwrap();
        let pairs: Vec<(&str, &str)> = people
            .iter()
            .map(|p| (p.id.value(), p.name.as_str()))
            .collect();
        assert_eq!(pairs.len(), 2);
        assert!(pairs.contains(&("1", "Alice")));
        assert!(pairs.contains(&("2", "Bob")));
    }

    #[test]
    fn update_changes_name() {
        let conn = setup_conn();
        conn.execute("INSERT INTO people (name) VALUES (?1)", ["Alice"])
            .unwrap();
        let repo = SqlitePersonRepository::new(&conn);

        repo.update(&UpdatePersonCommand {
            id: PersonId::new("1"),
            name: "Alicia".to_string(),
        })
        .unwrap();
        let name: String = conn
            .query_row("SELECT name FROM people WHERE id = 1", [], |row| row.get(0))
            .unwrap();
        assert_eq!(name, "Alicia");
    }

    #[test]
    fn remove_deletes_person_from_db() {
        let conn = setup_conn();
        conn.execute("INSERT INTO people (name) VALUES (?1)", ["Alice"])
            .unwrap();
        let repo = SqlitePersonRepository::new(&conn);

        repo.remove(&PersonId::new("1")).unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM people WHERE id = 1", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(count, 0);
    }
}
