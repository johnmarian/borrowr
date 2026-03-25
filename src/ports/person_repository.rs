use crate::commands::people::{AddPersonCommand, UpdatePersonCommand};
use crate::domain::error::DomainError;
use crate::domain::person::{Person, PersonId};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait PersonRepository {
    fn add(&self, command: &AddPersonCommand) -> Result<PersonId, DomainError>;
    fn update(&self, command: &UpdatePersonCommand) -> Result<(), DomainError>;
    fn find_by_id(&self, id: &PersonId) -> Result<Person, DomainError>;
    fn find_all(&self) -> Result<Vec<Person>, DomainError>;
    fn remove(&self, id: &PersonId) -> Result<(), DomainError>;
}
