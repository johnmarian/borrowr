use crate::domain::item::ItemId;
use crate::domain::loan::LoanId;
use crate::domain::person::PersonId;
use chrono::NaiveDate;

pub struct LendItemCommand {
    pub person_id: PersonId,
    pub item_id: ItemId,
    pub loan_date: NaiveDate,
}

pub struct BorrowItemCommand {
    pub person_id: PersonId,
    pub item_id: ItemId,
    pub loan_date: NaiveDate,
}

pub struct ReturnItemCommand {
    pub loan_id: LoanId,
    pub return_date: NaiveDate,
}
