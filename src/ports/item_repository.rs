use crate::commands::items::{AddItemCommand, UpdateItemCommand};
use crate::domain::error::DomainError;
use crate::domain::item::{Item, ItemId};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait ItemRepository {
    fn add(&self, command: &AddItemCommand) -> Result<ItemId, DomainError>;
    fn update(&self, command: &UpdateItemCommand) -> Result<(), DomainError>;
    fn find_by_id(&self, id: &ItemId) -> Result<Item, DomainError>;
    fn find_all(&self) -> Result<Vec<Item>, DomainError>;
    fn remove(&self, id: &ItemId) -> Result<(), DomainError>;
}
