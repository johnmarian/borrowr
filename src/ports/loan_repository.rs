use crate::commands::loans::{BorrowItemCommand, LendItemCommand, ReturnItemCommand};
use crate::domain::error::DomainError;
use crate::domain::loan::LoanId;
use crate::queries::loans::LoanView;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait LoanRepository {
    fn lend(&self, command: &LendItemCommand) -> Result<LoanId, DomainError>;
    fn borrow(&self, command: &BorrowItemCommand) -> Result<LoanId, DomainError>;
    fn return_item(&self, command: &ReturnItemCommand) -> Result<(), DomainError>;
    fn find_active(&self) -> Result<Vec<LoanView>, DomainError>;
}
