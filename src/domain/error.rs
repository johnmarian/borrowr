#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("failed to load item")]
    ItemLoadFailed,
    #[error("failed to save item")]
    ItemSaveFailed,
    #[error("failed to update item")]
    ItemUpdateFailed,
    #[error("failed to delete item")]
    ItemDeleteFailed,
    #[error("failed to load person")]
    PersonLoadFailed,
    #[error("failed to save person")]
    PersonSaveFailed,
    #[error("failed to update person")]
    PersonUpdateFailed,
    #[error("failed to delete person")]
    PersonDeleteFailed,
    #[error("failed to load loan")]
    LoanLoadFailed,
    #[error("failed to save loan")]
    LoanSaveFailed,
    #[error("failed to update loan")]
    LoanUpdateFailed,
    #[error("item not found")]
    ItemNotFound,
    #[error("person not found")]
    PersonNotFound,
    #[error("loan not found")]
    LoanNotFound,
    #[error("unknown error")]
    UnknownError,
    #[error("invalid data")]
    InvalidData,
}
