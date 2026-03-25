use crate::domain::loan::{Direction, LoanId};
use chrono::NaiveDate;
use std::fmt;

impl fmt::Display for LoanView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.direction {
            Direction::Lend => write!(
                f,
                "Lent \"{}\" to {} on {}",
                self.item_description, self.person_name, self.loan_date
            ),
            Direction::Borrow => write!(
                f,
                "Borrowed \"{}\" from {} on {}",
                self.item_description, self.person_name, self.loan_date
            ),
        }
    }
}

pub struct GetActiveLoans;

#[derive(Clone)]
pub struct LoanView {
    pub loan_id: LoanId,
    pub item_description: String,
    pub person_name: String,
    pub direction: Direction,
    pub loan_date: NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_displays_as_borrowed_item_from_person_on_date() {
        let view = LoanView {
            loan_id: LoanId::new("1"),
            item_description: "Drill".to_string(),
            person_name: "Alice".to_string(),
            direction: Direction::Borrow,
            loan_date: NaiveDate::from_ymd_opt(2026, 3, 24).unwrap(),
        };
        assert_eq!(
            view.to_string(),
            "Borrowed \"Drill\" from Alice on 2026-03-24"
        );
    }

    #[test]
    fn lend_displays_as_lent_item_to_person_on_date() {
        let view = LoanView {
            loan_id: LoanId::new("1"),
            item_description: "Drill".to_string(),
            person_name: "Alice".to_string(),
            direction: Direction::Lend,
            loan_date: NaiveDate::from_ymd_opt(2026, 3, 24).unwrap(),
        };
        assert_eq!(view.to_string(), "Lent \"Drill\" to Alice on 2026-03-24");
    }
}
