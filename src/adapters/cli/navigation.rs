use crate::adapters::cli::context::Ctx;
use crate::adapters::cli::selector::SelectOption;
use crate::commands::loans::{BorrowItemCommand, LendItemCommand, ReturnItemCommand};
use crate::domain::error::DomainError;
use crate::domain::item::ItemId;
use crate::domain::loan::{Direction, LoanId};
use crate::domain::person::PersonId;
use crate::queries::loans::LoanView;
use chrono::NaiveDate;
use std::collections::HashMap;

fn load_select_options<T: std::fmt::Display>(
    result: Result<Vec<T>, DomainError>,
) -> Result<Vec<SelectOption<T>>, Nav<'static>> {
    match result {
        Ok(items) => Ok(items.into_iter().map(SelectOption::Value).collect()),
        Err(e) => Err(Nav::Exit(Err(NavError::from(e)))),
    }
}

#[derive(Debug)]
pub struct NavError {
    pub code: i32,
    pub message: String,
}

impl From<DomainError> for NavError {
    fn from(e: DomainError) -> Self {
        let code = 1;
        NavError {
            code,
            message: e.to_string(),
        }
    }
}

enum Nav<'a> {
    Go(&'a dyn NavItem),
    Exit(Result<(), NavError>),
}

trait NavItem: Sync {
    fn step<'a>(&'a self, ctx: &'a Ctx) -> Nav<'a>;
}

struct MenuItem {
    label: &'static str,
    target: &'static dyn NavItem,
}

struct BasicNav {
    prompt: &'static str,
    items: &'static [MenuItem],
    back_nav: &'static dyn NavItem,
    back_nav_label: Option<&'static str>,
}

impl NavItem for BasicNav {
    fn step<'a>(&'a self, ctx: &'a Ctx) -> Nav<'a> {
        let mut labels = Vec::new();
        let mut nav_items: HashMap<&str, &dyn NavItem> = HashMap::new();

        for item in self.items.iter() {
            labels.push(item.label.to_string());
            nav_items.insert(item.label, item.target);
        }

        let back_label = self.back_nav_label.unwrap_or("Back");
        labels.push(back_label.to_string());
        nav_items.insert(back_label, self.back_nav);

        match ctx.selector.select(self.prompt, labels).as_deref() {
            Some(label) => Nav::Go(
                *nav_items
                    .get(label)
                    .unwrap_or_else(|| panic!("selected label {label:?} not in nav map")),
            ),
            None => Nav::Go(self.back_nav),
        }
    }
}

// --- Person type submenu (shared between lend and borrow) ---

struct PersonTypeMenu {
    item_id: ItemId,
    direction: Direction,
    on_existing: &'static str,
    on_new: &'static str,
    on_back: &'static dyn NavItem,
    confirm_prompt: fn(&str, &str) -> String,
}

impl PersonTypeMenu {
    fn run(&self, ctx: &Ctx) -> Nav<'static> {
        let options = vec![
            "Existing person".to_string(),
            "New person".to_string(),
            "Back".to_string(),
        ];
        match ctx
            .selector
            .select("Existing or new person?", options)
            .as_deref()
        {
            Some("Existing person") => {
                let mut persons = match load_select_options(ctx.person_repo.find_all()) {
                    Ok(opts) => opts,
                    Err(nav) => return nav,
                };
                persons.push(SelectOption::Back);
                match ctx.selector.select_person(self.on_existing, persons) {
                    Some(SelectOption::Value(person)) => self.confirm(ctx, &person.id),
                    Some(SelectOption::Back) | None => self.run(ctx),
                }
            }
            Some("New person") => match ctx.selector.input(self.on_new) {
                Some(name) => {
                    match ctx
                        .person_repo
                        .add(&crate::commands::people::AddPersonCommand { name })
                    {
                        Ok(person_id) => self.confirm(ctx, &person_id),
                        Err(e) => Nav::Exit(Err(NavError::from(e))),
                    }
                }
                None => self.run(ctx),
            },
            Some("Back") | None => Nav::Go(self.on_back),
            Some(label) => panic!("selected label {label:?} not in person type menu options"),
        }
    }

    fn confirm(&self, ctx: &Ctx, person_id: &PersonId) -> Nav<'static> {
        let item = match ctx.item_repo.find_by_id(&self.item_id) {
            Ok(item) => item,
            Err(e) => return Nav::Exit(Err(NavError::from(e))),
        };
        let person = match ctx.person_repo.find_by_id(person_id) {
            Ok(person) => person,
            Err(e) => return Nav::Exit(Err(NavError::from(e))),
        };
        let prompt = (self.confirm_prompt)(&item.description, &person.name);
        let options = vec!["Confirm".to_string(), "Cancel".to_string()];
        match ctx.selector.select(&prompt, options).as_deref() {
            Some("Confirm") => {
                let loan_date = ctx.clock.today();
                let result = match self.direction {
                    Direction::Lend => ctx
                        .loan_repo
                        .lend(&LendItemCommand {
                            item_id: ItemId::new(self.item_id.value()),
                            person_id: PersonId::new(person_id.value()),
                            loan_date,
                        })
                        .map(|_| ()),
                    Direction::Borrow => ctx
                        .loan_repo
                        .borrow(&BorrowItemCommand {
                            item_id: ItemId::new(self.item_id.value()),
                            person_id: PersonId::new(person_id.value()),
                            loan_date,
                        })
                        .map(|_| ()),
                };
                match result {
                    Ok(()) => {
                        let msg = match self.direction {
                            Direction::Lend => {
                                format!("Lent \"{}\" to {}.", item.description, person.name)
                            }
                            Direction::Borrow => {
                                format!("Borrowed \"{}\" from {}.", item.description, person.name)
                            }
                        };
                        ctx.selector.select(&msg, vec!["OK".to_string()]);
                        Nav::Go(&MAIN)
                    }
                    Err(e) => {
                        let options = vec!["Try again".to_string(), "Main Menu".to_string()];
                        match ctx.selector.select(&e.to_string(), options).as_deref() {
                            Some("Try again") => self.confirm(ctx, person_id),
                            _ => Nav::Go(&MAIN),
                        }
                    }
                }
            }
            _ => self.run(ctx),
        }
    }
}

// --- Exit ---

struct Exit;
impl NavItem for Exit {
    fn step<'a>(&'a self, _ctx: &'a Ctx) -> Nav<'a> {
        Nav::Exit(Ok(()))
    }
}

// --- Lend ---

struct LendExisting;
impl NavItem for LendExisting {
    fn step<'a>(&'a self, ctx: &'a Ctx) -> Nav<'a> {
        let mut options = match load_select_options(ctx.item_repo.find_all()) {
            Ok(opts) => opts,
            Err(nav) => return nav,
        };
        options.push(SelectOption::Back);
        match ctx.selector.select_item("Select an item to lend:", options) {
            Some(SelectOption::Value(item)) => PersonTypeMenu {
                item_id: item.id,
                direction: Direction::Lend,
                on_existing: "Select a person to lend to:",
                on_new: "Enter person name:",
                on_back: &LEND_EXISTING,
                confirm_prompt: |item, person| format!("Lend \"{}\" to {}?", item, person),
            }
            .run(ctx),
            Some(SelectOption::Back) | None => Nav::Go(&LEND),
        }
    }
}

struct LendNew;
impl NavItem for LendNew {
    fn step<'a>(&'a self, ctx: &'a Ctx) -> Nav<'a> {
        match ctx.selector.input("Enter item name:") {
            Some(name) => {
                match ctx
                    .item_repo
                    .add(&crate::commands::items::AddItemCommand { description: name })
                {
                    Ok(item_id) => PersonTypeMenu {
                        item_id,
                        direction: Direction::Lend,
                        on_existing: "Select a person to lend to:",
                        on_new: "Enter person name:",
                        on_back: &LEND,
                        confirm_prompt: |item, person| format!("Lend \"{}\" to {}?", item, person),
                    }
                    .run(ctx),
                    Err(e) => Nav::Exit(Err(NavError::from(e))),
                }
            }
            None => Nav::Go(&LEND),
        }
    }
}

// --- Borrow ---

struct BorrowExisting;
impl NavItem for BorrowExisting {
    fn step<'a>(&'a self, ctx: &'a Ctx) -> Nav<'a> {
        let mut options = match load_select_options(ctx.item_repo.find_all()) {
            Ok(opts) => opts,
            Err(nav) => return nav,
        };
        options.push(SelectOption::Back);
        match ctx
            .selector
            .select_item("Select an item you're borrowing:", options)
        {
            Some(SelectOption::Value(item)) => PersonTypeMenu {
                item_id: item.id,
                direction: Direction::Borrow,
                on_existing: "Who is lending it to you?",
                on_new: "Enter person name:",
                on_back: &BORROW_EXISTING,
                confirm_prompt: |item, person| format!("Borrow \"{}\" from {}?", item, person),
            }
            .run(ctx),
            Some(SelectOption::Back) | None => Nav::Go(&BORROW),
        }
    }
}

struct BorrowNew;
impl NavItem for BorrowNew {
    fn step<'a>(&'a self, ctx: &'a Ctx) -> Nav<'a> {
        match ctx.selector.input("Enter item name:") {
            Some(name) => {
                match ctx
                    .item_repo
                    .add(&crate::commands::items::AddItemCommand { description: name })
                {
                    Ok(item_id) => PersonTypeMenu {
                        item_id,
                        direction: Direction::Borrow,
                        on_existing: "Who is lending it to you?",
                        on_new: "Enter person name:",
                        on_back: &BORROW,
                        confirm_prompt: |item, person| {
                            format!("Borrow \"{}\" from {}?", item, person)
                        },
                    }
                    .run(ctx),
                    Err(e) => Nav::Exit(Err(NavError::from(e))),
                }
            }
            None => Nav::Go(&BORROW),
        }
    }
}

// --- View active loans ---

struct MarkAsReturned {
    loan: LoanView,
}

impl MarkAsReturned {
    fn run(&self, ctx: &Ctx) -> Nav<'static> {
        let today = ctx.clock.today();
        let input_prompt = format!("Enter return date (YYYY-MM-DD, default: {}):", today);
        loop {
            match ctx.selector.input(&input_prompt) {
                None => {
                    return LoanActions {
                        loan: self.loan.clone(),
                    }
                    .run(ctx);
                }
                Some(input) => {
                    let date_str = if input.is_empty() {
                        today.to_string()
                    } else {
                        input
                    };
                    let return_date = match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                        Ok(date) => date,
                        Err(_) => {
                            ctx.printer
                                .print_error("Invalid date. Please use YYYY-MM-DD format.");
                            continue;
                        }
                    };
                    let prompt = format!("Confirm return on {}?", date_str);
                    loop {
                        let options = vec!["Confirm".to_string(), "Cancel".to_string()];
                        match ctx.selector.select(&prompt, options).as_deref() {
                            Some("Confirm") => {
                                match ctx.loan_repo.return_item(&ReturnItemCommand {
                                    loan_id: LoanId::new(self.loan.loan_id.value()),
                                    return_date,
                                }) {
                                    Ok(()) => {
                                        let msg =
                                            format!("Returned \"{}\".", self.loan.item_description);
                                        ctx.selector.select(&msg, vec!["OK".to_string()]);
                                        return Nav::Go(&MAIN);
                                    }
                                    Err(e) => {
                                        let error_options =
                                            vec!["Try again".to_string(), "Main Menu".to_string()];
                                        match ctx
                                            .selector
                                            .select(&e.to_string(), error_options)
                                            .as_deref()
                                        {
                                            Some("Try again") => continue,
                                            Some("Main Menu") => return Nav::Go(&MAIN),
                                            _ => return Nav::Go(&MAIN),
                                        }
                                    }
                                }
                            }
                            _ => break,
                        }
                    }
                }
            }
        }
    }
}

struct LoanActions {
    loan: LoanView,
}

impl LoanActions {
    fn run(&self, ctx: &Ctx) -> Nav<'static> {
        let options = vec!["Mark as returned".to_string(), "Back".to_string()];
        match ctx
            .selector
            .select("What would you like to do?", options)
            .as_deref()
        {
            Some("Mark as returned") => MarkAsReturned {
                loan: self.loan.clone(),
            }
            .run(ctx),
            Some("Back") | None => Nav::Go(&VIEW_LOANS),
            Some(label) => panic!("selected label {label:?} not in loan actions options"),
        }
    }
}

struct ViewLoans;
impl NavItem for ViewLoans {
    fn step<'a>(&'a self, ctx: &'a Ctx) -> Nav<'a> {
        let mut loans = match ctx.loan_repo.find_active() {
            Ok(loans) => loans,
            Err(e) => return Nav::Exit(Err(NavError::from(e))),
        };
        loans.sort_by_key(|l| l.loan_date);
        let mut options: Vec<SelectOption<LoanView>> =
            loans.into_iter().map(SelectOption::Value).collect();
        options.push(SelectOption::Back);
        match ctx.selector.select_loan("Select a loan:", options) {
            Some(SelectOption::Value(loan)) => LoanActions { loan }.run(ctx),
            Some(SelectOption::Back) | None => Nav::Go(&MAIN),
        }
    }
}

// --- Statics ---

static EXIT: Exit = Exit;
static LEND_EXISTING: LendExisting = LendExisting;
static LEND_NEW: LendNew = LendNew;
static BORROW_EXISTING: BorrowExisting = BorrowExisting;
static BORROW_NEW: BorrowNew = BorrowNew;
static VIEW_LOANS: ViewLoans = ViewLoans;

static LEND: BasicNav = BasicNav {
    prompt: "Lend an existing item or a new one?",
    items: &[
        MenuItem {
            label: "Existing item",
            target: &LEND_EXISTING,
        },
        MenuItem {
            label: "New item",
            target: &LEND_NEW,
        },
    ],
    back_nav: &MAIN,
    back_nav_label: None,
};

static BORROW: BasicNav = BasicNav {
    prompt: "Borrow an existing item or a new one?",
    items: &[
        MenuItem {
            label: "Existing item",
            target: &BORROW_EXISTING,
        },
        MenuItem {
            label: "New item",
            target: &BORROW_NEW,
        },
    ],
    back_nav: &MAIN,
    back_nav_label: None,
};

static MAIN: BasicNav = BasicNav {
    prompt: "What would you like to do?",
    items: &[
        MenuItem {
            label: "Lend something out",
            target: &LEND,
        },
        MenuItem {
            label: "View active loans",
            target: &VIEW_LOANS,
        },
        MenuItem {
            label: "Borrow something",
            target: &BORROW,
        },
    ],
    back_nav: &EXIT,
    back_nav_label: Some("Quit"),
};

#[cfg(test)]
mod tests {
    use super::super::selector::MockSelector;
    use super::*;
    use crate::adapters::cli::clock::MockClock;
    use crate::adapters::cli::printer::MockPrinter;
    use crate::ports::item_repository::MockItemRepository;
    use crate::ports::loan_repository::MockLoanRepository;
    use crate::ports::person_repository::MockPersonRepository;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn make_ctx<'a>(
        selector: &'a MockSelector,
        clock: &'a MockClock,
        printer: &'a MockPrinter,
        item_repo: &'a MockItemRepository,
        person_repo: &'a MockPersonRepository,
        loan_repo: &'a MockLoanRepository,
    ) -> Ctx<'a> {
        Ctx {
            selector,
            clock,
            printer,
            item_repo,
            person_repo,
            loan_repo,
        }
    }

    struct ExitNode {
        step_count: AtomicUsize,
        result: Result<(), (i32, &'static str)>,
    }

    impl ExitNode {
        fn clean() -> Self {
            Self {
                step_count: AtomicUsize::new(0),
                result: Ok(()),
            }
        }

        fn error(code: i32, msg: &'static str) -> Self {
            Self {
                step_count: AtomicUsize::new(0),
                result: Err((code, msg)),
            }
        }
    }

    impl NavItem for ExitNode {
        fn step<'a>(&'a self, _ctx: &'a Ctx) -> Nav<'a> {
            self.step_count.fetch_add(1, Ordering::SeqCst);
            Nav::Exit(self.result.map_err(|(code, msg)| NavError {
                code,
                message: msg.to_string(),
            }))
        }
    }

    struct GoNode<'n> {
        target: &'n dyn NavItem,
        step_count: AtomicUsize,
    }

    impl<'n> NavItem for GoNode<'n> {
        fn step<'a>(&'a self, _ctx: &'a Ctx) -> Nav<'a> {
            self.step_count.fetch_add(1, Ordering::SeqCst);
            Nav::Go(self.target)
        }
    }

    #[test]
    fn router_exits_on_initial_exit_route() {
        let selector = MockSelector::new();
        let item_repo = MockItemRepository::new();
        let person_repo = MockPersonRepository::new();
        let loan_repo = MockLoanRepository::new();
        let clock = MockClock::new();
        let printer = MockPrinter::new();
        let ctx = make_ctx(
            &selector,
            &clock,
            &printer,
            &item_repo,
            &person_repo,
            &loan_repo,
        );

        let node = ExitNode::clean();
        run_app_router(&ctx, &node).unwrap();

        assert_eq!(node.step_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn router_follows_go_then_exits() {
        let selector = MockSelector::new();
        let item_repo = MockItemRepository::new();
        let person_repo = MockPersonRepository::new();
        let loan_repo = MockLoanRepository::new();
        let clock = MockClock::new();
        let printer = MockPrinter::new();
        let ctx = make_ctx(
            &selector,
            &clock,
            &printer,
            &item_repo,
            &person_repo,
            &loan_repo,
        );

        let exit = ExitNode::clean();
        let go = GoNode {
            target: &exit,
            step_count: AtomicUsize::new(0),
        };

        run_app_router(&ctx, &go).unwrap();

        assert_eq!(go.step_count.load(Ordering::SeqCst), 1);
        assert_eq!(exit.step_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn router_returns_err_with_code_and_message_on_error_exit() {
        let selector = MockSelector::new();
        let item_repo = MockItemRepository::new();
        let person_repo = MockPersonRepository::new();
        let loan_repo = MockLoanRepository::new();
        let clock = MockClock::new();
        let printer = MockPrinter::new();
        let ctx = make_ctx(
            &selector,
            &clock,
            &printer,
            &item_repo,
            &person_repo,
            &loan_repo,
        );

        let node = ExitNode::error(1, "something went wrong");
        let result = run_app_router(&ctx, &node);

        let err = result.unwrap_err();
        assert_eq!(err.code, 1);
        assert_eq!(err.message, "something went wrong");
    }

    #[test]
    fn router_returns_ok_on_clean_exit() {
        let selector = MockSelector::new();
        let item_repo = MockItemRepository::new();
        let person_repo = MockPersonRepository::new();
        let loan_repo = MockLoanRepository::new();
        let clock = MockClock::new();
        let printer = MockPrinter::new();
        let ctx = make_ctx(
            &selector,
            &clock,
            &printer,
            &item_repo,
            &person_repo,
            &loan_repo,
        );

        let node = ExitNode::clean();
        let result = run_app_router(&ctx, &node);

        assert!(result.is_ok());
    }

    struct LoopNode<'n> {
        next: &'n dyn NavItem,
        remaining: AtomicUsize,
        step_count: AtomicUsize,
    }

    impl<'n> NavItem for LoopNode<'n> {
        fn step<'a>(&'a self, _ctx: &'a Ctx) -> Nav<'a> {
            self.step_count.fetch_add(1, Ordering::SeqCst);
            if self.remaining.fetch_sub(1, Ordering::SeqCst) > 1 {
                Nav::Go(self)
            } else {
                Nav::Go(self.next)
            }
        }
    }

    #[test]
    fn router_visits_node_n_times_before_moving_on() {
        let selector = MockSelector::new();
        let item_repo = MockItemRepository::new();
        let person_repo = MockPersonRepository::new();
        let loan_repo = MockLoanRepository::new();
        let clock = MockClock::new();
        let printer = MockPrinter::new();
        let ctx = make_ctx(
            &selector,
            &clock,
            &printer,
            &item_repo,
            &person_repo,
            &loan_repo,
        );

        let exit = ExitNode::clean();
        let node = LoopNode {
            next: &exit,
            remaining: AtomicUsize::new(5),
            step_count: AtomicUsize::new(0),
        };

        run_app_router(&ctx, &node).unwrap();

        assert_eq!(node.step_count.load(Ordering::SeqCst), 5);
        assert_eq!(node.remaining.load(Ordering::SeqCst), 0);
        assert_eq!(exit.step_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn router_follows_chain_of_go_nodes() {
        let selector = MockSelector::new();
        let item_repo = MockItemRepository::new();
        let person_repo = MockPersonRepository::new();
        let loan_repo = MockLoanRepository::new();
        let clock = MockClock::new();
        let printer = MockPrinter::new();
        let ctx = make_ctx(
            &selector,
            &clock,
            &printer,
            &item_repo,
            &person_repo,
            &loan_repo,
        );

        let exit = ExitNode::clean();
        let b = GoNode {
            target: &exit,
            step_count: AtomicUsize::new(0),
        };
        let a = GoNode {
            target: &b,
            step_count: AtomicUsize::new(0),
        };
        let c = GoNode {
            target: &b,
            step_count: AtomicUsize::new(0),
        };

        run_app_router(&ctx, &a).unwrap();

        assert_eq!(a.step_count.load(Ordering::SeqCst), 1);
        assert_eq!(b.step_count.load(Ordering::SeqCst), 1);
        assert_eq!(c.step_count.load(Ordering::SeqCst), 0);
        assert_eq!(exit.step_count.load(Ordering::SeqCst), 1);
    }
}

pub fn launch_main_menu(ctx: &Ctx) -> Result<(), NavError> {
    run_app_router(ctx, &MAIN)
}

fn run_app_router(ctx: &Ctx, initial_route: &dyn NavItem) -> Result<(), NavError> {
    let mut current: &dyn NavItem = initial_route;
    loop {
        match current.step(ctx) {
            Nav::Go(next) => current = next,
            Nav::Exit(result) => return result,
        }
    }
}
