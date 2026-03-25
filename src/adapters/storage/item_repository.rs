use crate::commands::items::{AddItemCommand, UpdateItemCommand};
use crate::domain::error::DomainError;
use crate::domain::item::{Item, ItemId};
use crate::ports::item_repository::ItemRepository;
use rusqlite::Connection;

pub struct SqliteItemRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteItemRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

impl<'a> ItemRepository for SqliteItemRepository<'a> {
    fn add(&self, command: &AddItemCommand) -> Result<ItemId, DomainError> {
        self.conn
            .execute(
                "INSERT INTO items (description) VALUES (?1)",
                [&command.description],
            )
            .map_err(|_| DomainError::ItemSaveFailed)?;
        Ok(ItemId::new(self.conn.last_insert_rowid().to_string()))
    }

    fn update(&self, command: &UpdateItemCommand) -> Result<(), DomainError> {
        self.conn
            .execute(
                "UPDATE items SET description = ?1 WHERE id = ?2",
                [&command.description, command.id.value()],
            )
            .map_err(|_| DomainError::ItemUpdateFailed)?;
        Ok(())
    }

    fn find_by_id(&self, id: &ItemId) -> Result<Item, DomainError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, description FROM items WHERE id = ?1")
            .map_err(|_| DomainError::ItemLoadFailed)?;
        let mut rows = stmt
            .query_map([id.value()], map_row)
            .map_err(|_| DomainError::ItemLoadFailed)?;
        match rows.next() {
            Some(row) => row.map_err(|_| DomainError::ItemLoadFailed),
            None => Err(DomainError::ItemNotFound),
        }
    }

    fn find_all(&self) -> Result<Vec<Item>, DomainError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, description FROM items")
            .map_err(|_| DomainError::ItemLoadFailed)?;
        stmt.query_map([], map_row)
            .map_err(|_| DomainError::ItemLoadFailed)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| DomainError::ItemLoadFailed)
    }

    fn remove(&self, id: &ItemId) -> Result<(), DomainError> {
        self.conn
            .execute("DELETE FROM items WHERE id = ?1", [id.value()])
            .map_err(|_| DomainError::ItemDeleteFailed)?;
        Ok(())
    }
}

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<Item> {
    let id: i64 = row.get(0)?;
    let description: String = row.get(1)?;
    Ok(Item {
        id: ItemId::new(id.to_string()),
        description,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::items::{AddItemCommand, UpdateItemCommand};
    use crate::ports::item_repository::ItemRepository;
    use rusqlite::Connection;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE items (id INTEGER PRIMARY KEY, description TEXT NOT NULL);",
        )
        .unwrap();
        conn
    }

    #[test]
    fn add_returns_item_save_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqliteItemRepository::new(&conn);
        assert!(matches!(
            repo.add(&AddItemCommand {
                description: "Hammer".to_string()
            }),
            Err(DomainError::ItemSaveFailed)
        ));
    }

    #[test]
    fn update_returns_item_update_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqliteItemRepository::new(&conn);
        assert!(matches!(
            repo.update(&UpdateItemCommand {
                id: ItemId::new("1"),
                description: "Sledgehammer".to_string()
            }),
            Err(DomainError::ItemUpdateFailed)
        ));
    }

    #[test]
    fn find_by_id_returns_item_load_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqliteItemRepository::new(&conn);
        assert!(matches!(
            repo.find_by_id(&ItemId::new("1")),
            Err(DomainError::ItemLoadFailed)
        ));
    }

    #[test]
    fn find_all_returns_item_load_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqliteItemRepository::new(&conn);
        assert!(matches!(repo.find_all(), Err(DomainError::ItemLoadFailed)));
    }

    #[test]
    fn remove_returns_item_delete_failed_on_sql_error() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = SqliteItemRepository::new(&conn);
        assert!(matches!(
            repo.remove(&ItemId::new("1")),
            Err(DomainError::ItemDeleteFailed)
        ));
    }

    #[test]
    fn add_inserts_item_into_db() {
        let conn = setup_conn();
        let repo = SqliteItemRepository::new(&conn);
        let id = repo
            .add(&AddItemCommand {
                description: "Hammer".to_string(),
            })
            .unwrap();
        let description: String = conn
            .query_row(
                "SELECT description FROM items WHERE id = ?1",
                [id.value()],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(description, "Hammer");
    }

    #[test]
    fn find_by_id_returns_matching_item() {
        let conn = setup_conn();
        conn.execute("INSERT INTO items (description) VALUES (?1)", ["Hammer"])
            .unwrap();
        conn.execute("INSERT INTO items (description) VALUES (?1)", ["Drill"])
            .unwrap();
        let repo = SqliteItemRepository::new(&conn);

        let item = repo.find_by_id(&ItemId::new("2")).unwrap();
        assert_eq!(item.id.value(), "2");
        assert_eq!(item.description, "Drill");
    }

    #[test]
    fn find_by_id_returns_item_not_found_when_missing() {
        let conn = setup_conn();
        let repo = SqliteItemRepository::new(&conn);
        assert!(matches!(
            repo.find_by_id(&ItemId::new("99")),
            Err(DomainError::ItemNotFound)
        ));
    }

    #[test]
    fn find_all_returns_all_items() {
        let conn = setup_conn();
        conn.execute("INSERT INTO items (description) VALUES (?1)", ["Hammer"])
            .unwrap();
        conn.execute("INSERT INTO items (description) VALUES (?1)", ["Drill"])
            .unwrap();
        let repo = SqliteItemRepository::new(&conn);

        let items = repo.find_all().unwrap();
        let pairs: Vec<(&str, &str)> = items
            .iter()
            .map(|i| (i.id.value(), i.description.as_str()))
            .collect();
        assert_eq!(pairs.len(), 2);
        assert!(pairs.contains(&("1", "Hammer")));
        assert!(pairs.contains(&("2", "Drill")));
    }

    #[test]
    fn update_changes_description() {
        let conn = setup_conn();
        conn.execute("INSERT INTO items (description) VALUES (?1)", ["Hammer"])
            .unwrap();
        let repo = SqliteItemRepository::new(&conn);

        repo.update(&UpdateItemCommand {
            id: ItemId::new("1"),
            description: "Sledgehammer".to_string(),
        })
        .unwrap();
        let description: String = conn
            .query_row("SELECT description FROM items WHERE id = 1", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(description, "Sledgehammer");
    }

    #[test]
    fn remove_deletes_item_from_db() {
        let conn = setup_conn();
        conn.execute("INSERT INTO items (description) VALUES (?1)", ["Hammer"])
            .unwrap();
        let repo = SqliteItemRepository::new(&conn);

        repo.remove(&ItemId::new("1")).unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM items WHERE id = 1", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(count, 0);
    }
}
