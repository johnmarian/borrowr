use crate::adapters::cli::clock::{Clock, SystemClock};
use crate::adapters::cli::printer::{ConsolePrinter, Printer};
use crate::adapters::cli::selector::{InquireSelector, Selector};
use crate::ports::item_repository::ItemRepository;
use crate::ports::loan_repository::LoanRepository;
use crate::ports::person_repository::PersonRepository;

pub struct Ctx<'a> {
    pub(super) selector: &'a dyn Selector,
    pub(super) clock: &'a dyn Clock,
    pub(super) printer: &'a dyn Printer,
    pub item_repo: &'a dyn ItemRepository,
    pub person_repo: &'a dyn PersonRepository,
    pub loan_repo: &'a dyn LoanRepository,
}

impl<'a> Ctx<'a> {
    pub fn new(
        item_repo: &'a dyn ItemRepository,
        person_repo: &'a dyn PersonRepository,
        loan_repo: &'a dyn LoanRepository,
    ) -> Self {
        static SELECTOR: InquireSelector = InquireSelector;
        static CLOCK: SystemClock = SystemClock;
        static PRINTER: ConsolePrinter = ConsolePrinter;
        Self {
            selector: &SELECTOR,
            clock: &CLOCK,
            printer: &PRINTER,
            item_repo,
            person_repo,
            loan_repo,
        }
    }
}
