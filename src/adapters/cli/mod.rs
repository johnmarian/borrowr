mod clock;
mod context;
mod navigation;
mod printer;
mod selector;

use crate::adapters::cli::navigation::launch_main_menu;

pub use context::Ctx;
pub use navigation::NavError;
pub use selector::SelectOption;

pub fn run(ctx: &Ctx) -> Result<(), NavError> {
    println!("┌──────────────────────────────────────┐");
    println!("│               borrowr                │");
    println!("└──────────────────────────────────────┘");
    println!();

    launch_main_menu(ctx)
}

#[cfg(test)]
mod tests {
    //! Flow coverage map:
    //!
    //! Main Menu ("What would you like to do?")
    //!   → [`run_loads_main_menu_with_expected_text`], [`run_returns_ok_on_clean_exit`]
    //!   │ ESC → exit  → [`esc_from_main_menu_exits_cleanly`]
    //! ├── Lend something out ("Lend an existing item or a new one?")
    //!   → [`selecting_lend_presents_lend_submenu`]
    //!   │   ├── Back → Main Menu  → [`back_from_lend_submenu_returns_to_main_menu`]
    //!   │   │ ESC → Main Menu  → [`esc_from_lend_submenu_returns_to_main_menu`]
    //!   │   ├── Existing item → ("Select an item to lend") *(typed: select_item)*
    //!   │   │   → [`lend_existing_item_presents_item_list`], [`lend_existing_item_calls_find_all_on_item_repo`], [`lend_existing_item_list_includes_back`], [`lend_existing_item_list_uses_select_item`]
    //!   │   │   │ repo error  → [`lend_existing_item_repo_error_propagates`]
    //!   │   │   ├── Back → ("Lend an existing item or a new one?")  → [`back_from_lend_item_list_returns_to_lend_submenu`]
    //!   │   │   │ ESC → ("Lend an existing item or a new one?")  → [`esc_from_lend_item_list_returns_to_lend_submenu`]
    //!   │   │   └── \<item selected\> → ("Existing or new person?")
    //!   │   │       → [`after_selecting_item_presents_person_type_menu`]
    //!   │   │       │ unknown option → panic  → [`person_type_menu_panics_on_unknown_option`]
    //!   │   │       ├── Back → ("Select an item to lend")  → [`back_from_person_type_in_lend_flow_returns_to_item_list`]
    //!   │   │       │ ESC → ("Select an item to lend")  → [`esc_from_person_type_in_lend_flow_returns_to_item_list`]
    //!   │   │       ├── Existing person → ("Select a person to lend to") *(typed: select_person)*
    //!   │   │       │   → [`lend_existing_item_existing_person_presents_person_list`], [`lend_existing_item_existing_person_calls_find_all_on_person_repo`], [`lend_existing_item_existing_person_list_shows_people_from_repo`], [`lend_existing_person_selection_uses_select_person`], [`existing_person_selection_includes_back`]
    //!   │   │       │   │ repo error  → [`person_repo_error_in_person_type_menu_propagates`]
    //!   │   │       │   ├── Back → ("Existing or new person?")  → [`back_from_existing_person_selection_returns_to_person_type_menu`]
    //!   │   │       │   │ ESC → ("Existing or new person?")  → [`esc_from_existing_person_selection_returns_to_person_type_menu`]
    //!   │   │       │   └── \<person selected\> → ("Lend X to Y?")
    //!   │   │       │       → [`lend_confirmation_shown_after_selecting_person`]
    //!   │   │       │       │ item repo error  → [`confirm_item_repo_error_propagates`]
    //!   │   │       │       │ person repo error  → [`confirm_person_repo_error_propagates`]
    //!   │   │       │       │ item not found → panic  → [`confirm_panics_when_item_not_found_in_repo`]
    //!   │   │       │       │ person not found → panic  → [`confirm_panics_when_person_not_found_in_repo`]
    //!   │   │       │       ├── Confirm → loan saved → "Lent \"<item>\" to <person>."  → [`lend_existing_item_existing_person_confirm_calls_lend_shows_success_and_returns_to_main`], [`lend_existing_item_existing_person_shows_success_message`]
    //!   │   │       │       │   └── OK → Main Menu  → [`lend_existing_item_existing_person_confirm_calls_lend_shows_success_and_returns_to_main`], [`lend_existing_item_existing_person_ok_returns_to_main`]
    //!   │   │       │       │   error → "<error message>"  → [`lend_existing_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │   │       │       │       ├── Try again → ("Lend X to Y?")  → [`lend_existing_item_existing_person_loan_save_error_try_again_reshows_confirm`]
    //!   │   │       │       │       └── Main Menu → Main Menu  → [`lend_existing_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │   │       │       └── Cancel → ("Existing or new person?")  → [`lend_cancel_confirmation_returns_to_person_type`]
    //!   │   │       └── New person → ("Enter person name")
    //!   │   │           → [`lend_existing_item_new_person_presents_name_input`]
    //!   │   │           ├── ESC → ("Existing or new person?")  → [`esc_from_lend_new_person_name_returns_to_person_type_menu`]
    //!   │   │           └── \<name entered\> → ("Lend X to Y?")
    //!   │   │               → [`lend_existing_item_new_person_calls_add_on_person_repo`], [`lend_existing_item_new_person_shows_confirmation`]
    //!   │   │               │ add error  → [`lend_existing_item_new_person_add_error_propagates`]
    //!   │   │               ├── Confirm → loan saved → "Lent \"<item>\" to <person>."  → [`lend_existing_item_new_person_confirm_calls_lend_shows_success_and_returns_to_main`], [`lend_existing_item_new_person_shows_success_message`]
    //!   │   │               │   └── OK → Main Menu  → [`lend_existing_item_new_person_confirm_calls_lend_shows_success_and_returns_to_main`], [`lend_existing_item_new_person_ok_returns_to_main`]
    //!   │   │               │   error → "<error message>"  → [`lend_existing_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │   │               │       ├── Try again → ("Lend X to Y?")  → [`lend_existing_item_new_person_loan_save_error_try_again_reshows_confirm`]
    //!   │   │               │       └── Main Menu → Main Menu  → [`lend_existing_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │   │               └── Cancel → ("Existing or new person?")  → [`lend_existing_item_new_person_cancel_confirmation_returns_to_person_type`]
    //!   │   └── New item → ("Enter item name")
    //!   │       → [`lend_new_item_presents_name_input`]
    //!   │       ├── ESC → ("Lend an existing item or a new one?")  → [`esc_from_lend_new_item_name_returns_to_lend_submenu`]
    //!   │       └── \<name entered\> → ("Existing or new person?")
    //!   │           → [`lend_new_item_after_name_presents_person_type_menu`], [`lend_new_item_calls_add_on_item_repo`]
    //!   │           │ add error  → [`lend_new_item_add_error_propagates`]
    //!   │           ├── Back → ("Lend an existing item or a new one?")  → [`back_from_person_type_in_lend_new_item_flow_returns_to_lend_submenu`]
    //!   │           │ ESC → ("Lend an existing item or a new one?")  → [`esc_from_person_type_in_lend_new_item_flow_returns_to_lend_submenu`]
    //!   │           ├── Existing person → ("Select a person to lend to") *(typed: select_person)*
    //!   │           │   → [`lend_new_item_existing_person_presents_person_list`], [`lend_new_item_existing_person_calls_find_all_on_person_repo`], [`lend_new_item_existing_person_list_shows_people_from_repo`]
    //!   │           │   ├── Back → ("Existing or new person?")  → [`back_from_existing_person_selection_in_lend_new_item_flow_returns_to_person_type`]
    //!   │           │   │ ESC → ("Existing or new person?")  → [`esc_from_existing_person_selection_in_lend_new_item_flow_returns_to_person_type`]
    //!   │           │   └── \<person selected\> → ("Lend X to Y?")
    //!   │           │       ├── Confirm → loan saved → "Lent \"<item>\" to <person>."  → [`lend_new_item_existing_person_confirm_calls_lend_shows_success_and_returns_to_main`], [`lend_new_item_existing_person_shows_success_message`]
    //!   │           │       │   └── OK → Main Menu  → [`lend_new_item_existing_person_confirm_calls_lend_shows_success_and_returns_to_main`], [`lend_new_item_existing_person_ok_returns_to_main`]
    //!   │           │       │   error → "<error message>"  → [`lend_new_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │           │       │       ├── Try again → ("Lend X to Y?")  → [`lend_new_item_existing_person_loan_save_error_try_again_reshows_confirm`]
    //!   │           │       │       └── Main Menu → Main Menu  → [`lend_new_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │           │       └── Cancel → ("Existing or new person?")  → [`lend_new_item_existing_person_cancel_confirmation_returns_to_person_type`]
    //!   │           └── New person → ("Enter person name:")
    //!   │               → [`lend_new_item_new_person_presents_name_input`]
    //!   │               │ add error  → [`lend_new_item_new_person_add_error_propagates`]
    //!   │               ├── ESC → ("Existing or new person?")  → [`esc_from_lend_new_item_new_person_name_returns_to_person_type`]
    //!   │               └── \<name entered\> → ("Lend X to Y?")
    //!   │                   → [`lend_new_item_new_person_shows_confirmation`], [`lend_new_item_new_person_calls_add_on_person_repo`]
    //!   │                   ├── Confirm → loan saved → "Lent \"<item>\" to <person>."  → [`lend_new_item_new_person_confirm_calls_lend_shows_success_and_returns_to_main`], [`lend_new_item_new_person_shows_success_message`]
    //!   │                   │   └── OK → Main Menu  → [`lend_new_item_new_person_confirm_calls_lend_shows_success_and_returns_to_main`], [`lend_new_item_new_person_ok_returns_to_main`]
    //!   │                   │   error → "<error message>"  → [`lend_new_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │                   │       ├── Try again → ("Lend X to Y?")  → [`lend_new_item_new_person_loan_save_error_try_again_reshows_confirm`]
    //!   │                   │       └── Main Menu → Main Menu  → [`lend_new_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │                   └── Cancel → ("Existing or new person?")  → [`lend_new_item_new_person_cancel_confirmation_returns_to_person_type`]
    //! ├── View active loans → ("Select a loan")
    //!   → [`view_active_loans_presents_loan_list`], [`view_active_loans_calls_find_active_on_loan_repo`], [`view_active_loans_shows_loans_from_repo`], [`view_active_loans_list_includes_back`], [`view_active_loans_shows_loans_oldest_first`]
    //!   │   repo error → [`view_active_loans_repo_error_propagates`]
    //!   │   ├── Back → Main Menu  → [`back_from_loan_list_returns_to_main_menu`]
    //!   │   │ ESC → Main Menu  → [`esc_from_loan_list_returns_to_main_menu`]
    //!   │   └── \<loan selected\> → ("What would you like to do?")
    //!   │       → [`selecting_loan_presents_loan_actions`]
    //!   │       │ unknown option → panic  → [`loan_actions_panics_on_unknown_option`]
    //!   │       ├── Back → ("Select a loan")  → [`back_from_loan_actions_returns_to_loan_list`]
    //!   │       │ ESC → ("Select a loan")  → [`esc_from_loan_actions_returns_to_loan_list`]
    //!   │       └── Mark as returned → ("Enter return date")
    //!   │           → [`mark_as_returned_presents_date_input`], [`mark_as_returned_prompt_includes_todays_date`]
    //!   │           │ empty input → today used  → [`mark_as_returned_empty_input_defaults_to_today`]
    //!   │           │ invalid date → error + re-prompt  → [`mark_as_returned_invalid_date_reprompts`], [`mark_as_returned_invalid_then_valid_proceeds_to_confirmation`]
    //!   │           │ ESC → ("What would you like to do?")  → [`esc_from_date_input_returns_to_loan_actions`]
    //!   │           └── \<date entered\> → ("Confirm return?")
    //!   │               → [`confirm_return_presents_confirm_options`]
    //!   │               ├── Confirm → loan updated → "Returned \"<item>\"."  → [`confirm_return_calls_return_item_on_repo`], [`confirm_return_shows_success_message`], [`confirm_return_ok_returns_to_main`]
    //!   │               │   error → "<error message>"  → [`confirm_return_repo_error_shows_error_screen`]
    //!   │               │       ├── Try again → ("Confirm return?")  → [`confirm_return_error_try_again_reshows_confirm`]
    //!   │               │       ├── ESC → Main Menu  → [`esc_from_return_error_screen_returns_to_main`]
    //!   │               │       └── Main Menu → Main Menu  → [`confirm_return_error_main_menu_returns_to_main`]
    //!   │               ├── Cancel → ("Enter return date")  → [`cancel_return_confirmation_returns_to_date_input`]
    //!   │               └── ESC → ("Enter return date")  → [`esc_from_confirm_return_returns_to_date_input`]
    //! ├── Borrow something ("Borrow an existing item or a new one?")
    //!   → [`selecting_borrow_presents_borrow_submenu`]
    //!   │   ├── Back → Main Menu  → [`back_from_borrow_submenu_returns_to_main_menu`]
    //!   │   │ ESC → Main Menu  → [`esc_from_borrow_submenu_returns_to_main_menu`]
    //!   │   ├── Existing item → ("Select an item you're borrowing") *(typed: select_item)*
    //!   │   │   → [`borrow_existing_item_presents_item_list`], [`borrow_existing_item_list_includes_back`], [`borrow_existing_item_list_uses_select_item`]
    //!   │   │   │ repo error  → [`borrow_existing_item_repo_error_propagates`]
    //!   │   │   ├── Back → ("Borrow an existing item or a new one?")  → [`back_from_borrow_item_list_returns_to_borrow_submenu`]
    //!   │   │   │ ESC → ("Borrow an existing item or a new one?")  → [`esc_from_borrow_item_list_returns_to_borrow_submenu`]
    //!   │   │   └── \<item selected\> → ("Existing or new person?")
    //!   │   │       ├── Back → ("Select an item you're borrowing")  → [`back_from_person_type_in_borrow_flow_returns_to_item_list`]
    //!   │   │       │ ESC → ("Select an item you're borrowing")  → [`esc_from_person_type_in_borrow_flow_returns_to_item_list`]
    //!   │   │       ├── Existing person → ("Who is lending it to you?") *(typed: select_person)*
    //!   │   │       │   → [`borrow_existing_item_existing_person_presents_person_list`], [`borrow_existing_item_existing_person_calls_find_all_on_person_repo`], [`borrow_existing_item_existing_person_list_shows_people_from_repo`], [`borrow_existing_person_selection_uses_select_person`]
    //!   │   │       │   ├── Back → ("Existing or new person?")  → [`back_from_existing_person_selection_in_borrow_existing_item_flow_returns_to_person_type`]
    //!   │   │       │   │ ESC → ("Existing or new person?")  → [`esc_from_existing_person_selection_in_borrow_existing_item_flow_returns_to_person_type`]
    //!   │   │       │   └── \<person selected\> → ("Borrow X from Y?")
    //!   │   │       │       → [`borrow_confirmation_shown_after_selecting_person`]
    //!   │   │       │       ├── Confirm → loan saved → "Borrowed \"<item>\" from <person>."  → [`borrow_existing_item_existing_person_confirm_calls_borrow_shows_success_and_returns_to_main`], [`borrow_existing_item_existing_person_shows_success_message`]
    //!   │   │       │       │   └── OK → Main Menu  → [`borrow_existing_item_existing_person_confirm_calls_borrow_shows_success_and_returns_to_main`], [`borrow_existing_item_existing_person_ok_returns_to_main`]
    //!   │   │       │       │   error → "<error message>"  → [`borrow_existing_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │   │       │       │       ├── Try again → ("Borrow X from Y?")  → [`borrow_existing_item_existing_person_loan_save_error_try_again_reshows_confirm`]
    //!   │   │       │       │       └── Main Menu → Main Menu  → [`borrow_existing_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │   │       │       └── Cancel → ("Existing or new person?")  → [`borrow_existing_item_existing_person_cancel_confirmation_returns_to_person_type`]
    //!   │   │       └── New person → ("Enter person name:")
    //!   │   │           → [`borrow_existing_item_new_person_presents_name_input`]
    //!   │   │           │ add error  → [`borrow_existing_item_new_person_add_error_propagates`]
    //!   │   │           ├── ESC → ("Existing or new person?")  → [`esc_from_borrow_new_person_name_returns_to_person_type_menu`]
    //!   │   │           └── \<name entered\> → ("Borrow X from Y?")
    //!   │   │               → [`borrow_existing_item_new_person_shows_confirmation`], [`borrow_existing_item_new_person_calls_add_on_person_repo`]
    //!   │   │               ├── Confirm → loan saved → "Borrowed \"<item>\" from <person>."  → [`borrow_existing_item_new_person_confirm_calls_borrow_shows_success_and_returns_to_main`], [`borrow_existing_item_new_person_shows_success_message`]
    //!   │   │               │   └── OK → Main Menu  → [`borrow_existing_item_new_person_confirm_calls_borrow_shows_success_and_returns_to_main`], [`borrow_existing_item_new_person_ok_returns_to_main`]
    //!   │   │               │   error → "<error message>"  → [`borrow_existing_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │   │               │       ├── Try again → ("Borrow X from Y?")  → [`borrow_existing_item_new_person_loan_save_error_try_again_reshows_confirm`]
    //!   │   │               │       └── Main Menu → Main Menu  → [`borrow_existing_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │   │               └── Cancel → ("Existing or new person?")  → [`borrow_existing_item_new_person_cancel_confirmation_returns_to_person_type`]
    //!   │   └── New item → ("Enter item name")
    //!   │       → [`borrow_new_item_presents_name_input`]
    //!   │       ├── ESC → ("Borrow an existing item or a new one?")  → [`esc_from_borrow_new_item_name_returns_to_borrow_submenu`]
    //!   │       └── \<name entered\> → ("Existing or new person?")
    //!   │           │ add error  → [`borrow_new_item_add_error_propagates`]
    //!   │           ├── Back → ("Borrow an existing item or a new one?")  → [`back_from_person_type_in_borrow_new_item_flow_returns_to_borrow_submenu`]
    //!   │           │ ESC → ("Borrow an existing item or a new one?")  → [`esc_from_person_type_in_borrow_new_item_flow_returns_to_borrow_submenu`]
    //!   │           ├── Existing person → ("Who is lending it to you?") *(typed: select_person)*
    //!   │           │   → [`borrow_new_item_existing_person_presents_person_list`], [`borrow_new_item_existing_person_calls_find_all_on_person_repo`], [`borrow_new_item_existing_person_list_shows_people_from_repo`]
    //!   │           │   ├── Back → ("Existing or new person?")  → [`back_from_existing_person_selection_in_borrow_new_item_flow_returns_to_person_type`]
    //!   │           │   │ ESC → ("Existing or new person?")  → [`esc_from_existing_person_selection_in_borrow_new_item_flow_returns_to_person_type`]
    //!   │           │   └── \<person selected\> → ("Borrow X from Y?")
    //!   │           │       ├── Confirm → loan saved → "Borrowed \"<item>\" from <person>."  → [`borrow_new_item_existing_person_confirm_calls_borrow_shows_success_and_returns_to_main`], [`borrow_new_item_existing_person_shows_success_message`]
    //!   │           │       │   └── OK → Main Menu  → [`borrow_new_item_existing_person_confirm_calls_borrow_shows_success_and_returns_to_main`], [`borrow_new_item_existing_person_ok_returns_to_main`]
    //!   │           │       │   error → "<error message>"  → [`borrow_new_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │           │       │       ├── Try again → ("Borrow X from Y?")  → [`borrow_new_item_existing_person_loan_save_error_try_again_reshows_confirm`]
    //!   │           │       │       └── Main Menu → Main Menu  → [`borrow_new_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │           │       └── Cancel → ("Existing or new person?")  → [`borrow_new_item_existing_person_cancel_confirmation_returns_to_person_type`]
    //!   │           └── New person → ("Enter person name:")
    //!   │               → [`borrow_new_item_new_person_presents_name_input`]
    //!   │               │ add error  → [`borrow_new_item_new_person_add_error_propagates`]
    //!   │               ├── ESC → ("Existing or new person?")  → [`esc_from_borrow_new_item_new_person_name_returns_to_person_type`]
    //!   │               └── \<name entered\> → ("Borrow X from Y?")
    //!   │                   → [`borrow_new_item_new_person_shows_confirmation`], [`borrow_new_item_new_person_calls_add_on_person_repo`]
    //!   │                   ├── Confirm → loan saved → "Borrowed \"<item>\" from <person>."  → [`borrow_new_item_new_person_confirm_calls_borrow_shows_success_and_returns_to_main`], [`borrow_new_item_new_person_shows_success_message`]
    //!   │                   │   └── OK → Main Menu  → [`borrow_new_item_new_person_confirm_calls_borrow_shows_success_and_returns_to_main`], [`borrow_new_item_new_person_ok_returns_to_main`]
    //!   │                   │   error → "<error message>"  → [`borrow_new_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │                   │       ├── Try again → ("Borrow X from Y?")  → [`borrow_new_item_new_person_loan_save_error_try_again_reshows_confirm`]
    //!   │                   │       └── Main Menu → Main Menu  → [`borrow_new_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main`]
    //!   │                   └── Cancel → ("Existing or new person?")  → [`borrow_new_item_new_person_cancel_confirmation_returns_to_person_type`]
    //! └── Quit  *(no test — relies on 3rd party library behavior)*

    use super::printer::MockPrinter;
    use super::selector::MockSelector;
    use super::*;
    use crate::adapters::cli::clock::MockClock;
    use crate::commands::loans::{BorrowItemCommand, LendItemCommand, ReturnItemCommand};
    use crate::domain::error::DomainError;
    use crate::domain::item::{Item, ItemId};
    use crate::domain::loan::{Direction, LoanId};
    use crate::domain::person::{Person, PersonId};
    use crate::ports::item_repository::MockItemRepository;
    use crate::ports::loan_repository::MockLoanRepository;
    use crate::ports::person_repository::MockPersonRepository;
    use crate::queries::loans::LoanView;
    use chrono::NaiveDate;
    use mockall::predicate;

    fn nav(value: &'static str) -> impl Fn(&str, Vec<String>) -> Option<String> {
        move |_, _| Some(value.to_string())
    }

    fn nav_item(
        id: &'static str,
        description: &'static str,
    ) -> impl Fn(&str, Vec<SelectOption<Item>>) -> Option<SelectOption<Item>> {
        move |_, _| {
            Some(SelectOption::Value(Item {
                id: ItemId::new(id),
                description: description.to_string(),
            }))
        }
    }

    fn nav_person(
        id: &'static str,
        name: &'static str,
    ) -> impl Fn(&str, Vec<SelectOption<Person>>) -> Option<SelectOption<Person>> {
        move |_, _| {
            Some(SelectOption::Value(Person {
                id: PersonId::new(id),
                name: name.to_string(),
            }))
        }
    }

    fn end_item() -> impl Fn(&str, Vec<SelectOption<Item>>) -> Option<SelectOption<Item>> {
        |_, _| None
    }

    fn end_person() -> impl Fn(&str, Vec<SelectOption<Person>>) -> Option<SelectOption<Person>> {
        |_, _| None
    }

    fn back_item() -> impl Fn(&str, Vec<SelectOption<Item>>) -> Option<SelectOption<Item>> {
        |_, _| Some(SelectOption::Back)
    }

    fn back_person() -> impl Fn(&str, Vec<SelectOption<Person>>) -> Option<SelectOption<Person>> {
        |_, _| Some(SelectOption::Back)
    }

    fn back_loan() -> impl Fn(&str, Vec<SelectOption<LoanView>>) -> Option<SelectOption<LoanView>> {
        |_, _| Some(SelectOption::Back)
    }

    fn end_loan() -> impl Fn(&str, Vec<SelectOption<LoanView>>) -> Option<SelectOption<LoanView>> {
        |_, _| None
    }

    fn nav_loan() -> impl Fn(&str, Vec<SelectOption<LoanView>>) -> Option<SelectOption<LoanView>> {
        |_, _| {
            Some(SelectOption::Value(LoanView {
                loan_id: LoanId::new("1"),
                item_description: "Drill".to_string(),
                person_name: "Alice".to_string(),
                direction: Direction::Lend,
                loan_date: NaiveDate::from_ymd_opt(2026, 3, 24).unwrap(),
            }))
        }
    }

    fn nav_input(value: &'static str) -> impl Fn(&str) -> Option<String> {
        move |_| Some(value.to_string())
    }

    fn end() -> impl Fn(&str, Vec<String>) -> Option<String> {
        |_, _| None
    }

    fn end_input() -> impl Fn(&str) -> Option<String> {
        |_| None
    }

    #[test]
    fn run_loads_main_menu_with_expected_text() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::eq(vec![
                    "Lend something out".to_string(),
                    "View active loans".to_string(),
                    "Borrow something".to_string(),
                    "Quit".to_string(),
                ]),
            )
            .times(1)
            .returning(nav("Quit"));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn selecting_lend_presents_lend_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::eq(vec![
                    "Existing item".to_string(),
                    "New item".to_string(),
                    "Back".to_string(),
                ]),
            )
            .times(1)
            .returning(nav("Back"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // back from lend submenu → main menu → Quit

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- ESC from text inputs ---

    #[test]
    fn esc_from_lend_new_item_name_returns_to_lend_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item")); // lend submenu
        sel.expect_input().times(1).returning(end_input()); // ESC on item name
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → main menu
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_borrow_new_item_name_returns_to_borrow_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item")); // borrow submenu
        sel.expect_input().times(1).returning(end_input()); // ESC on item name
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → main menu
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_lend_new_person_name_returns_to_person_type_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item")); // item list
        sel.expect_select().times(1).returning(nav("New person")); // person type
        sel.expect_input().times(1).returning(end_input()); // ESC on person name
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_borrow_new_person_name_returns_to_person_type_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item")); // item list
        sel.expect_select().times(1).returning(nav("New person")); // person type
        sel.expect_input().times(1).returning(end_input()); // ESC on person name
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- Borrow: existing item → new person sub-flow ---

    #[test]
    fn borrow_existing_item_new_person_presents_name_input() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person")); // person type
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice")); // name input → confirm
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_new_person_calls_add_on_person_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .withf(|cmd| cmd.name == "Alice")
            .times(1)
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_new_person_add_error_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input().times(1).returning(nav_input("Alice"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Err(DomainError::UnknownError));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();

        assert_eq!(err.code, 1);
        assert_eq!(err.message, "unknown error");
    }

    #[test]
    fn borrow_existing_item_new_person_shows_confirmation() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Borrow \"Drill\" from Alice?"),
                predicate::eq(vec!["Confirm".to_string(), "Cancel".to_string()]),
            )
            .times(1)
            .returning(nav("Cancel")); // cancel → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- Lend: existing item flow ---

    #[test]
    fn lend_existing_item_presents_item_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- Back navigation ---

    #[test]
    fn back_from_lend_submenu_returns_to_main_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Back")); // lend submenu
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu re-displays → Quit

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_lend_submenu_returns_to_main_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(end()); // ESC on lend submenu → main menu re-displays
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn back_from_borrow_submenu_returns_to_main_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Back")); // borrow submenu
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu re-displays → Quit

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_borrow_submenu_returns_to_main_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(end()); // ESC on borrow submenu → main menu re-displays
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn back_from_person_type_in_lend_flow_returns_to_item_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item")); // item list
        sel.expect_select().times(1).returning(nav("Back")); // person type menu
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list re-displays → Back
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().times(2).returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_person_type_in_lend_flow_returns_to_item_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item")); // item list
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(end()); // ESC on person type menu
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().times(2).returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn back_from_person_type_in_borrow_flow_returns_to_item_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item")); // item list
        sel.expect_select().times(1).returning(nav("Back")); // person type menu
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list re-displays → Back
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().times(2).returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_person_type_in_borrow_flow_returns_to_item_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item")); // item list
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(end()); // ESC on person type menu
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().times(2).returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn back_from_loan_actions_returns_to_loan_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan()); // loan list → select loan
        sel.expect_select().times(1).returning(nav("Back")); // loan actions → Back → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn esc_from_loan_actions_returns_to_loan_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan()); // loan list → select loan
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(end()); // ESC on loan actions → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn loan_actions_panics_on_unknown_option() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(|_, _| Some("__unknown__".to_string()));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_list_includes_back() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::function(|opts: &Vec<SelectOption<Item>>| {
                    opts.iter().any(|o| matches!(o, SelectOption::Back))
                }),
            )
            .times(1)
            .returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn back_from_lend_item_list_returns_to_lend_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item")); // lend submenu 1st
        sel.expect_select_item()
            .times(1)
            .returning(|_, _| Some(SelectOption::Back)); // item list → Back
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu 2nd → Back
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().times(1).returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_lend_item_list_returns_to_lend_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item")); // lend submenu
        sel.expect_select_item().times(1).returning(end_item()); // ESC on item list
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(end()); // lend submenu re-displays
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(end());

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().times(1).returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn back_from_borrow_item_list_returns_to_borrow_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item")); // borrow submenu 1st
        sel.expect_select_item()
            .times(1)
            .returning(|_, _| Some(SelectOption::Back)); // item list → Back
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu 2nd → Back
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().times(1).returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_borrow_item_list_returns_to_borrow_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item")); // borrow submenu
        sel.expect_select_item().times(1).returning(end_item()); // ESC on item list
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(end()); // borrow submenu re-displays
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(end());

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().times(1).returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_list_includes_back() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::function(|opts: &Vec<SelectOption<Item>>| {
                    opts.iter().any(|o| matches!(o, SelectOption::Back))
                }),
            )
            .times(1)
            .returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- Item repository integration ---

    #[test]
    fn lend_existing_item_calls_find_all_on_item_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item().times(1).returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().times(1).returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn after_selecting_item_presents_person_type_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item"));
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::eq(vec![
                    "Existing person".to_string(),
                    "New person".to_string(),
                    "Back".to_string(),
                ]),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().times(2).returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_existing_person_presents_person_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .with(
                predicate::eq("Select a person to lend to:"),
                predicate::always(),
            )
            .times(1)
            .returning(nav_person("2", "Alice")); // select a person → confirm dialog
        sel.expect_select().times(1).returning(nav("Cancel")); // cancel confirmation → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Some Item".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_new_person_presents_name_input() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirm dialog → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Some Item".to_string(),
            })
        });

        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_new_person_calls_add_on_person_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input().times(1).returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirm dialog → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });

        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .withf(|cmd| cmd.name == "Alice")
            .times(1)
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_new_person_shows_confirmation() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input().times(1).returning(nav_input("Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Lend \"Drill\" to Alice?"),
                predicate::eq(vec!["Confirm".to_string(), "Cancel".to_string()]),
            )
            .times(1)
            .returning(nav("Cancel")); // cancel → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });

        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn confirm_item_not_found_propagates_as_error() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input().times(1).returning(nav_input("Alice"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo
            .expect_find_by_id()
            .returning(|_| Err(DomainError::ItemNotFound));

        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();
        assert_eq!(err.message, "item not found");
    }

    #[test]
    fn confirm_person_not_found_propagates_as_error() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input().times(1).returning(nav_input("Alice"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });

        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo
            .expect_find_by_id()
            .returning(|_| Err(DomainError::PersonNotFound));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();
        assert_eq!(err.message, "person not found");
    }

    #[test]
    fn lend_existing_item_new_person_add_error_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input().times(1).returning(nav_input("Alice"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));

        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Err(DomainError::UnknownError));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();

        assert_eq!(err.code, 1);
        assert_eq!(err.message, "unknown error");
    }

    // --- Lend: new item flow ---

    #[test]
    fn back_from_person_type_in_lend_new_item_flow_returns_to_lend_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill")); // item name entered
        sel.expect_select().times(1).returning(nav("Back")); // person type → Back
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu re-displays → Back
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_add().returning(|_| Ok(ItemId::new("1")));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_person_type_in_lend_new_item_flow_returns_to_lend_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill")); // item name entered
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(end()); // ESC on person type menu
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → main menu
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_add().returning(|_| Ok(ItemId::new("1")));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn back_from_person_type_in_borrow_new_item_flow_returns_to_borrow_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill")); // item name entered
        sel.expect_select().times(1).returning(nav("Back")); // person type → Back
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu re-displays → Back
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_add().returning(|_| Ok(ItemId::new("1")));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_person_type_in_borrow_new_item_flow_returns_to_borrow_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill")); // item name entered
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(end()); // ESC on person type menu
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → main menu
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_add().returning(|_| Ok(ItemId::new("1")));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn person_type_menu_panics_on_unknown_option() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item"));
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(|_, _| Some("__unknown__".to_string()));

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_presents_name_input() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_add().returning(|_| Ok(ItemId::new("1")));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_after_name_presents_person_type_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("My Item"));
        sel.expect_select().times(1).returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_add().returning(|_| Ok(ItemId::new("1")));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_calls_add_on_item_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("My Drill"));
        sel.expect_select().times(1).returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_add()
            .withf(|cmd| cmd.description == "My Drill")
            .times(1)
            .returning(|_| Ok(ItemId::new("1")));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- Lend: new item → existing person sub-flow ---

    #[test]
    fn lend_new_item_existing_person_presents_person_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type
        sel.expect_select_person()
            .with(
                predicate::eq("Select a person to lend to:"),
                predicate::always(),
            )
            .times(1)
            .returning(nav_person("2", "Alice")); // person list → confirm
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_existing_person_calls_find_all_on_person_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_find_all()
            .times(1)
            .returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_existing_person_list_shows_people_from_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .with(
                predicate::eq("Select a person to lend to:"),
                predicate::function(|opts: &Vec<SelectOption<Person>>| {
                    let names: Vec<&str> = opts
                        .iter()
                        .filter_map(|o| match o {
                            SelectOption::Value(p) => Some(p.name.as_str()),
                            SelectOption::Back => None,
                        })
                        .collect();
                    names == vec!["Alice", "Bob"]
                }),
            )
            .times(1)
            .returning(nav_person("1", "Alice")); // select a person → confirm
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| {
            Ok(vec![
                Person {
                    id: PersonId::new("1"),
                    name: "Alice".to_string(),
                },
                Person {
                    id: PersonId::new("2"),
                    name: "Bob".to_string(),
                },
            ])
        });
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("1"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn back_from_existing_person_selection_in_lend_new_item_flow_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type
        sel.expect_select_person()
            .with(
                predicate::eq("Select a person to lend to:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_person()); // Back on person list → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_existing_person_selection_in_lend_new_item_flow_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type
        sel.expect_select_person()
            .with(
                predicate::eq("Select a person to lend to:"),
                predicate::always(),
            )
            .times(1)
            .returning(end_person()); // ESC on person list → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- Lend: new item → new person sub-flow ---

    #[test]
    fn lend_new_item_new_person_presents_name_input() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person")); // person type
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice")); // name input → confirm
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_lend_new_item_new_person_name_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person")); // person type
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(end_input()); // ESC on person name → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_new_person_calls_add_on_person_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .withf(|cmd| cmd.name == "Alice")
            .times(1)
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_new_person_add_error_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input().times(1).returning(nav_input("Alice"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Err(DomainError::UnknownError));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();

        assert_eq!(err.code, 1);
        assert_eq!(err.message, "unknown error");
    }

    #[test]
    fn lend_new_item_new_person_shows_confirmation() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Lend \"Drill\" to Alice?"),
                predicate::eq(vec!["Confirm".to_string(), "Cancel".to_string()]),
            )
            .times(1)
            .returning(nav("Cancel")); // cancel → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- Borrow: existing item flow ---

    #[test]
    fn selecting_borrow_presents_borrow_submenu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::eq(vec![
                    "Existing item".to_string(),
                    "New item".to_string(),
                    "Back".to_string(),
                ]),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_presents_item_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_existing_person_presents_person_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .with(
                predicate::eq("Who is lending it to you?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav_person("2", "Alice")); // select a person → confirm dialog
        sel.expect_select().times(1).returning(nav("Cancel")); // cancel confirmation → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Some Item".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_presents_name_input() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("Back")); // person type → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_add().returning(|_| Ok(ItemId::new("1")));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- Borrow: new item → existing person sub-flow ---

    #[test]
    fn borrow_new_item_existing_person_presents_person_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type
        sel.expect_select_person()
            .with(
                predicate::eq("Who is lending it to you?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav_person("2", "Alice")); // person list → confirm
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_existing_person_calls_find_all_on_person_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_find_all()
            .times(1)
            .returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_existing_person_list_shows_people_from_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .with(
                predicate::eq("Who is lending it to you?"),
                predicate::function(|opts: &Vec<SelectOption<Person>>| {
                    let names: Vec<&str> = opts
                        .iter()
                        .filter_map(|o| match o {
                            SelectOption::Value(p) => Some(p.name.as_str()),
                            SelectOption::Back => None,
                        })
                        .collect();
                    names == vec!["Alice", "Bob"]
                }),
            )
            .times(1)
            .returning(nav_person("1", "Alice")); // select a person → confirm
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| {
            Ok(vec![
                Person {
                    id: PersonId::new("1"),
                    name: "Alice".to_string(),
                },
                Person {
                    id: PersonId::new("2"),
                    name: "Bob".to_string(),
                },
            ])
        });
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("1"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn back_from_existing_person_selection_in_borrow_new_item_flow_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type
        sel.expect_select_person()
            .with(
                predicate::eq("Who is lending it to you?"),
                predicate::always(),
            )
            .times(1)
            .returning(back_person()); // Back on person list → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_existing_person_selection_in_borrow_new_item_flow_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type
        sel.expect_select_person()
            .with(
                predicate::eq("Who is lending it to you?"),
                predicate::always(),
            )
            .times(1)
            .returning(end_person()); // ESC on person list → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- Borrow: new item → new person sub-flow ---

    #[test]
    fn borrow_new_item_new_person_presents_name_input() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person")); // person type
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice")); // name input → confirm
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_borrow_new_item_new_person_name_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person")); // person type
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(end_input()); // ESC on person name → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_new_person_calls_add_on_person_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .withf(|cmd| cmd.name == "Alice")
            .times(1)
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_new_person_add_error_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input().times(1).returning(nav_input("Alice"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Err(DomainError::UnknownError));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();

        assert_eq!(err.code, 1);
        assert_eq!(err.message, "unknown error");
    }

    #[test]
    fn borrow_new_item_new_person_shows_confirmation() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Borrow \"Drill\" from Alice?"),
                predicate::eq(vec!["Confirm".to_string(), "Cancel".to_string()]),
            )
            .times(1)
            .returning(nav("Cancel")); // cancel → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- Person repository integration ---

    #[test]
    fn lend_existing_item_existing_person_calls_find_all_on_person_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // cancel confirmation → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Some Item".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_find_all()
            .times(1)
            .returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_existing_person_list_shows_people_from_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .with(
                predicate::eq("Select a person to lend to:"),
                predicate::function(|opts: &Vec<SelectOption<Person>>| {
                    let names: Vec<&str> = opts
                        .iter()
                        .filter_map(|o| match o {
                            SelectOption::Value(p) => Some(p.name.as_str()),
                            SelectOption::Back => None,
                        })
                        .collect();
                    names == vec!["Alice", "Bob"]
                }),
            )
            .times(1)
            .returning(nav_person("1", "Alice")); // select a person → confirm dialog
        sel.expect_select().times(1).returning(nav("Cancel")); // cancel confirmation → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Some Item".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| {
            Ok(vec![
                Person {
                    id: PersonId::new("1"),
                    name: "Alice".to_string(),
                },
                Person {
                    id: PersonId::new("2"),
                    name: "Bob".to_string(),
                },
            ])
        });
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("1"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_existing_person_calls_find_all_on_person_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // cancel confirmation → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Some Item".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_find_all()
            .times(1)
            .returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_existing_person_list_shows_people_from_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Some Item"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .with(
                predicate::eq("Who is lending it to you?"),
                predicate::function(|opts: &Vec<SelectOption<Person>>| {
                    let names: Vec<&str> = opts
                        .iter()
                        .filter_map(|o| match o {
                            SelectOption::Value(p) => Some(p.name.as_str()),
                            SelectOption::Back => None,
                        })
                        .collect();
                    names == vec!["Alice", "Bob"]
                }),
            )
            .times(1)
            .returning(nav_person("1", "Alice")); // select a person → confirm dialog
        sel.expect_select().times(1).returning(nav("Cancel")); // cancel confirmation → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Some Item".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| {
            Ok(vec![
                Person {
                    id: PersonId::new("1"),
                    name: "Alice".to_string(),
                },
                Person {
                    id: PersonId::new("2"),
                    name: "Bob".to_string(),
                },
            ])
        });
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("1"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn back_from_existing_person_selection_in_borrow_existing_item_flow_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type
        sel.expect_select_person()
            .with(
                predicate::eq("Who is lending it to you?"),
                predicate::always(),
            )
            .times(1)
            .returning(back_person()); // Back on person list → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_existing_person_selection_in_borrow_existing_item_flow_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type
        sel.expect_select_person()
            .with(
                predicate::eq("Who is lending it to you?"),
                predicate::always(),
            )
            .times(1)
            .returning(end_person()); // ESC on person list → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- Typed selection ---

    #[test]
    fn lend_existing_item_list_uses_select_item() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_list_uses_select_item() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut repo = MockItemRepository::new();
        repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_person_selection_uses_select_person() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type menu
        sel.expect_select_person()
            .with(
                predicate::eq("Select a person to lend to:"),
                predicate::always(),
            )
            .times(1)
            .returning(nav_person("2", "Alice")); // select a person → confirm dialog
        sel.expect_select().times(1).returning(nav("Cancel")); // cancel confirmation → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_person_selection_uses_select_person() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type menu
        sel.expect_select_person()
            .with(
                predicate::eq("Who is lending it to you?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav_person("2", "Alice")); // select a person → confirm dialog
        sel.expect_select().times(1).returning(nav("Cancel")); // cancel confirmation → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn existing_person_selection_includes_back() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .with(
                predicate::always(),
                predicate::function(|opts: &Vec<SelectOption<Person>>| {
                    opts.iter().any(|o| matches!(o, SelectOption::Back))
                }),
            )
            .times(1)
            .returning(back_person());
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back"));
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn back_from_existing_person_selection_returns_to_person_type_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .with(
                predicate::eq("Select a person to lend to:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_person()); // Back on person list → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_existing_person_selection_returns_to_person_type_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .with(
                predicate::eq("Select a person to lend to:"),
                predicate::always(),
            )
            .times(1)
            .returning(end_person()); // ESC on person list → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_confirmation_shown_after_selecting_person() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Lend \"Drill\" to Alice?"),
                predicate::eq(vec!["Confirm".to_string(), "Cancel".to_string()]),
            )
            .times(1)
            .returning(nav("Cancel")); // cancel confirmation → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_confirmation_shown_after_selecting_person() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Borrow \"Drill\" from Alice?"),
                predicate::eq(vec!["Confirm".to_string(), "Cancel".to_string()]),
            )
            .times(1)
            .returning(nav("Cancel")); // cancel confirmation → person type re-displays
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_existing_person_confirm_calls_lend_shows_success_and_returns_to_main() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .with(predicate::function(|cmd: &LendItemCommand| {
                cmd.item_id.value() == "1"
                    && cmd.person_id.value() == "2"
                    && cmd.loan_date == NaiveDate::from_ymd_opt(2026, 3, 24).unwrap()
            }))
            .times(1)
            .returning(|_| Ok(LoanId::new("1")));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_existing_person_shows_success_message() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_lend().returning(|_| Ok(LoanId::new("1")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_existing_person_ok_returns_to_main() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::always(),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_lend().returning(|_| Ok(LoanId::new("1")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main()
     {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Main Menu"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_existing_person_loan_save_error_try_again_reshows_confirm() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Lend \"Drill\" to Alice?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Try again"));
        sel.expect_select()
            .with(
                predicate::eq("Lend \"Drill\" to Alice?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Cancel"));
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back"));
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_cancel_confirmation_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type 1st
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit")); // main menu → Quit

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_find_all()
            .times(1)
            .returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_new_person_cancel_confirmation_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person")); // person type 1st
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type 2nd → Back → LEND_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_existing_person_cancel_confirmation_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type 1st
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type 2nd → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_new_person_cancel_confirmation_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person")); // person type 1st
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type 2nd → Back → LEND
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // lend submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_existing_person_cancel_confirmation_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type 1st
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type 2nd → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_new_person_cancel_confirmation_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person")); // person type 1st
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type 2nd → Back → BORROW_EXISTING
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item you're borrowing:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item()); // item list → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_existing_person_cancel_confirmation_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person")); // person type 1st
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type 2nd → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_new_person_cancel_confirmation_returns_to_person_type() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person")); // person type 1st
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Cancel")); // confirmation → Cancel
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // person type 2nd → Back → BORROW
        sel.expect_select()
            .with(
                predicate::eq("Borrow an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // borrow submenu → Back → MAIN
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    // --- View active loans flow ---

    #[test]
    fn view_active_loans_presents_loan_list() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn selecting_loan_presents_loan_actions() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .with(
                predicate::always(),
                predicate::eq(vec!["Mark as returned".to_string(), "Back".to_string()]),
            )
            .times(1)
            .returning(nav("Back")); // Back → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn mark_as_returned_prompt_includes_todays_date() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .with(predicate::eq(
                "Enter return date (YYYY-MM-DD, default: 1900-01-01):",
            ))
            .times(1)
            .returning(end_input()); // ESC → loan actions
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // Back → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn mark_as_returned_empty_input_defaults_to_today() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .times(1)
            .returning(|_| Some("".to_string())); // empty → default to today
        sel.expect_select()
            .with(
                predicate::eq("Confirm return on 1900-01-01?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Cancel")); // Cancel → date input
        sel.expect_input().times(1).returning(end_input()); // ESC → loan actions
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // Back → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn mark_as_returned_invalid_date_reprompts() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .times(1)
            .returning(|_| Some("not-a-date".to_string()));
        sel.expect_input().times(1).returning(end_input()); // ESC after re-prompt → loan actions
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // Back → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut printer = MockPrinter::new();
        printer
            .expect_print_error()
            .with(predicate::eq("Invalid date. Please use YYYY-MM-DD format."))
            .times(1)
            .return_const(());
        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &printer,
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn mark_as_returned_invalid_then_valid_proceeds_to_confirmation() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .times(1)
            .returning(|_| Some("not-a-date".to_string()));
        sel.expect_input()
            .times(1)
            .returning(nav_input("1901-06-15"));
        sel.expect_select()
            .with(
                predicate::eq("Confirm return on 1901-06-15?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Cancel")); // Cancel → re-prompt
        sel.expect_input().times(1).returning(end_input()); // ESC → loan actions
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // Back → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut printer = MockPrinter::new();
        printer
            .expect_print_error()
            .with(predicate::eq("Invalid date. Please use YYYY-MM-DD format."))
            .times(1)
            .return_const(());
        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &printer,
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn mark_as_returned_presents_date_input() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .with(predicate::eq(
                "Enter return date (YYYY-MM-DD, default: 1900-01-01):",
            ))
            .times(1)
            .returning(end_input()); // ESC → loan actions
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // Back → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn confirm_return_presents_confirm_options() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .times(1)
            .returning(nav_input("1901-06-15"));
        sel.expect_select()
            .with(
                predicate::always(),
                predicate::eq(vec!["Confirm".to_string(), "Cancel".to_string()]),
            )
            .times(1)
            .returning(nav("Cancel")); // Cancel → date input re-displays
        sel.expect_input().times(1).returning(end_input()); // ESC → loan actions
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // Back → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn esc_from_confirm_return_returns_to_date_input() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .with(predicate::eq(
                "Enter return date (YYYY-MM-DD, default: 1900-01-01):",
            ))
            .times(1)
            .returning(nav_input("1902-11-30"));
        sel.expect_select().times(1).returning(end()); // ESC on confirm → date input
        sel.expect_input()
            .with(predicate::eq(
                "Enter return date (YYYY-MM-DD, default: 1900-01-01):",
            ))
            .times(1)
            .returning(end_input()); // ESC → loan actions
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // Back → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn cancel_return_confirmation_returns_to_date_input() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .with(predicate::eq(
                "Enter return date (YYYY-MM-DD, default: 1900-01-01):",
            ))
            .times(1)
            .returning(nav_input("1902-11-30"));
        sel.expect_select().times(1).returning(nav("Cancel")); // Cancel → date input
        sel.expect_input()
            .with(predicate::eq(
                "Enter return date (YYYY-MM-DD, default: 1900-01-01):",
            ))
            .times(1)
            .returning(end_input()); // ESC → loan actions
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // Back → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn confirm_return_calls_return_item_on_repo() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .times(1)
            .returning(nav_input("1901-06-15"));
        sel.expect_select()
            .with(
                predicate::eq("Confirm return on 1901-06-15?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(predicate::eq("Returned \"Drill\"."), predicate::always())
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        loan_repo
            .expect_return_item()
            .with(predicate::function(|cmd: &ReturnItemCommand| {
                cmd.loan_id.value() == "1"
                    && cmd.return_date == NaiveDate::from_ymd_opt(1901, 6, 15).unwrap()
            }))
            .times(1)
            .returning(|_| Ok(()));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn confirm_return_shows_success_message() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .times(1)
            .returning(nav_input("1901-06-15"));
        sel.expect_select()
            .with(
                predicate::eq("Confirm return on 1901-06-15?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Returned \"Drill\"."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        loan_repo
            .expect_return_item()
            .times(1)
            .returning(|_| Ok(()));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn confirm_return_ok_returns_to_main() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .times(1)
            .returning(nav_input("1901-06-15"));
        sel.expect_select()
            .with(
                predicate::eq("Confirm return on 1901-06-15?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(predicate::eq("Returned \"Drill\"."), predicate::always())
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        loan_repo
            .expect_return_item()
            .times(1)
            .returning(|_| Ok(()));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn confirm_return_repo_error_shows_error_screen() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .times(1)
            .returning(nav_input("1901-06-15"));
        sel.expect_select()
            .with(
                predicate::eq("Confirm return on 1901-06-15?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to update loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Main Menu"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        loan_repo
            .expect_return_item()
            .times(1)
            .returning(|_| Err(DomainError::LoanUpdateFailed));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn confirm_return_error_main_menu_returns_to_main() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .times(1)
            .returning(nav_input("1901-06-15"));
        sel.expect_select()
            .with(
                predicate::eq("Confirm return on 1901-06-15?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(predicate::eq("failed to update loan"), predicate::always())
            .times(1)
            .returning(nav("Main Menu"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        loan_repo
            .expect_return_item()
            .times(1)
            .returning(|_| Err(DomainError::LoanUpdateFailed));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn confirm_return_error_try_again_reshows_confirm() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .times(1)
            .returning(nav_input("1901-06-15"));
        sel.expect_select()
            .with(
                predicate::eq("Confirm return on 1901-06-15?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(predicate::eq("failed to update loan"), predicate::always())
            .times(1)
            .returning(nav("Try again"));
        sel.expect_select()
            .with(
                predicate::eq("Confirm return on 1901-06-15?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(predicate::eq("Returned \"Drill\"."), predicate::always())
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        loan_repo
            .expect_return_item()
            .times(1)
            .returning(|_| Err(DomainError::LoanUpdateFailed));
        loan_repo
            .expect_return_item()
            .times(1)
            .returning(|_| Ok(()));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn esc_from_return_error_screen_returns_to_main() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan().times(1).returning(nav_loan());
        sel.expect_select()
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .times(1)
            .returning(nav_input("1901-06-15"));
        sel.expect_select()
            .with(
                predicate::eq("Confirm return on 1901-06-15?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(predicate::eq("failed to update loan"), predicate::always())
            .times(1)
            .returning(end()); // ESC → Main Menu
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        loan_repo
            .expect_return_item()
            .times(1)
            .returning(|_| Err(DomainError::LoanUpdateFailed));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn run_returns_ok_on_clean_exit() {
        let mut sel = MockSelector::new();
        sel.expect_select().times(1).returning(nav("Quit"));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn esc_from_main_menu_exits_cleanly() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(end());

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_repo_error_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));

        let mut item_repo = MockItemRepository::new();
        item_repo
            .expect_find_all()
            .returning(|| Err(DomainError::ItemLoadFailed));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();

        assert_eq!(err.code, 1);
        assert_eq!(err.message, "failed to load item");
    }

    #[test]
    fn borrow_existing_item_repo_error_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));

        let mut item_repo = MockItemRepository::new();
        item_repo
            .expect_find_all()
            .returning(|| Err(DomainError::ItemLoadFailed));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();

        assert_eq!(err.code, 1);
        assert_eq!(err.message, "failed to load item");
    }

    #[test]
    fn person_repo_error_in_person_type_menu_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_find_all()
            .returning(|| Err(DomainError::PersonLoadFailed));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();

        assert_eq!(err.code, 1);
        assert_eq!(err.message, "failed to load person");
    }

    #[test]
    fn confirm_item_repo_error_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("1", "Alice"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo
            .expect_find_by_id()
            .returning(|_| Err(DomainError::ItemLoadFailed));

        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();

        assert_eq!(err.code, 1);
        assert_eq!(err.message, "failed to load item");
    }

    #[test]
    fn confirm_person_repo_error_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("1", "Alice"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });

        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo
            .expect_find_by_id()
            .returning(|_| Err(DomainError::PersonLoadFailed));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();

        assert_eq!(err.code, 1);
        assert_eq!(err.message, "failed to load person");
    }

    #[test]
    fn lend_new_item_add_error_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));

        let mut item_repo = MockItemRepository::new();
        item_repo
            .expect_add()
            .returning(|_| Err(DomainError::ItemSaveFailed));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();

        assert_eq!(err.code, 1);
        assert_eq!(err.message, "failed to save item");
    }

    #[test]
    fn esc_from_loan_list_returns_to_main_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → should return to main menu
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn back_from_loan_list_returns_to_main_menu() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(back_loan());
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn view_active_loans_calls_find_active_on_loan_repo() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(back_loan());
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_find_active()
            .times(1)
            .returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn view_active_loans_shows_loans_oldest_first() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan()
            .with(
                predicate::eq("Select a loan:"),
                predicate::function(|opts: &Vec<SelectOption<LoanView>>| {
                    opts[0].to_string().contains("1901-01-01")
                        && opts[1].to_string().contains("1902-01-01")
                }),
            )
            .times(1)
            .returning(back_loan());
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| {
            Ok(vec![
                LoanView {
                    loan_id: LoanId::new("1"),
                    item_description: "Drill".to_string(),
                    person_name: "Alice".to_string(),
                    direction: Direction::Lend,
                    loan_date: NaiveDate::from_ymd_opt(1902, 1, 1).unwrap(),
                },
                LoanView {
                    loan_id: LoanId::new("2"),
                    item_description: "Hammer".to_string(),
                    person_name: "Bob".to_string(),
                    direction: Direction::Borrow,
                    loan_date: NaiveDate::from_ymd_opt(1901, 1, 1).unwrap(),
                },
            ])
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn view_active_loans_shows_loans_from_repo() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan()
            .with(
                predicate::eq("Select a loan:"),
                predicate::function(|opts: &Vec<SelectOption<LoanView>>| {
                    opts.iter()
                        .any(|o| o.to_string() == "Lent \"Drill\" to Alice on 2026-03-24")
                        && opts
                            .iter()
                            .any(|o| o.to_string() == "Borrowed \"Hammer\" from Bob on 2026-03-24")
                }),
            )
            .times(1)
            .returning(back_loan());
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(move || {
            Ok(vec![
                LoanView {
                    loan_id: LoanId::new("1"),
                    item_description: "Drill".to_string(),
                    person_name: "Alice".to_string(),
                    direction: Direction::Lend,
                    loan_date,
                },
                LoanView {
                    loan_id: LoanId::new("2"),
                    item_description: "Hammer".to_string(),
                    person_name: "Bob".to_string(),
                    direction: Direction::Borrow,
                    loan_date,
                },
            ])
        });

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn view_active_loans_list_includes_back() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan()
            .with(
                predicate::eq("Select a loan:"),
                predicate::function(|opts: &Vec<SelectOption<LoanView>>| {
                    opts.iter().any(|o| o.to_string() == "Back")
                }),
            )
            .times(1)
            .returning(back_loan());
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));

        run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn view_active_loans_repo_error_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("View active loans"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_find_active()
            .returning(|| Err(DomainError::LoanLoadFailed));

        let result = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        });
        assert!(result.is_err());
    }

    #[test]
    fn esc_from_date_input_returns_to_loan_actions() {
        let today = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("View active loans"));
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(nav_loan());
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Mark as returned"));
        sel.expect_input()
            .with(predicate::eq(
                "Enter return date (YYYY-MM-DD, default: 1900-01-01):",
            ))
            .times(1)
            .returning(end_input()); // ESC → loan actions
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back")); // Back → loan list
        sel.expect_select_loan()
            .with(predicate::eq("Select a loan:"), predicate::always())
            .times(1)
            .returning(end_loan()); // ESC → Main
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_find_active().returning(|| Ok(vec![]));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || today);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &MockItemRepository::new(),
            person_repo: &MockPersonRepository::new(),
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_add_error_propagates() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input().times(1).returning(nav_input("Drill"));

        let mut item_repo = MockItemRepository::new();
        item_repo
            .expect_add()
            .returning(|_| Err(DomainError::ItemSaveFailed));

        let err = run(&Ctx {
            selector: &sel,
            clock: &MockClock::new(),
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &MockPersonRepository::new(),
            loan_repo: &MockLoanRepository::new(),
        })
        .unwrap_err();

        assert_eq!(err.code, 1);
        assert_eq!(err.message, "failed to save item");
    }

    #[test]
    fn lend_existing_item_new_person_confirm_calls_lend_shows_success_and_returns_to_main() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .with(predicate::function(|cmd: &LendItemCommand| {
                cmd.item_id.value() == "1"
                    && cmd.person_id.value() == "2"
                    && cmd.loan_date == NaiveDate::from_ymd_opt(2026, 3, 24).unwrap()
            }))
            .times(1)
            .returning(|_| Ok(LoanId::new("1")));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_new_person_shows_success_message() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_lend().returning(|_| Ok(LoanId::new("1")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_new_person_ok_returns_to_main() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::always(),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_lend().returning(|_| Ok(LoanId::new("1")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main()
    {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Main Menu"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_existing_item_new_person_loan_save_error_try_again_reshows_confirm() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Lend \"Drill\" to Alice?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Try again"));
        sel.expect_select()
            .with(
                predicate::eq("Lend \"Drill\" to Alice?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Cancel"));
        sel.expect_select()
            .with(
                predicate::eq("Existing or new person?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back"));
        sel.expect_select_item()
            .with(
                predicate::eq("Select an item to lend:"),
                predicate::always(),
            )
            .times(1)
            .returning(back_item());
        sel.expect_select()
            .with(
                predicate::eq("Lend an existing item or a new one?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Back"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        let mut clock = MockClock::new();
        clock.expect_today().times(1).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_existing_person_confirm_calls_borrow_shows_success_and_returns_to_main()
    {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .with(predicate::function(move |cmd: &BorrowItemCommand| {
                cmd.item_id.value() == "1"
                    && cmd.person_id.value() == "2"
                    && cmd.loan_date == loan_date
            }))
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_existing_person_shows_success_message() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_existing_person_ok_returns_to_main() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::always(),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main()
     {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Main Menu"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_existing_person_loan_save_error_try_again_reshows_confirm() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Borrow \"Drill\" from Alice?"),
                predicate::always(),
            )
            .times(2)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Try again"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().times(2).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_new_person_confirm_calls_borrow_shows_success_and_returns_to_main() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .with(predicate::function(move |cmd: &BorrowItemCommand| {
                cmd.item_id.value() == "1"
                    && cmd.person_id.value() == "2"
                    && cmd.loan_date == loan_date
            }))
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_new_person_shows_success_message() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_new_person_ok_returns_to_main() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::always(),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main()
     {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Main Menu"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_existing_item_new_person_loan_save_error_try_again_reshows_confirm() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("Existing item"));
        sel.expect_select_item()
            .times(1)
            .returning(nav_item("1", "Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Borrow \"Drill\" from Alice?"),
                predicate::always(),
            )
            .times(2)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Try again"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_find_all().returning(|| Ok(vec![]));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().times(2).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_existing_person_confirm_calls_lend_shows_success_and_returns_to_main() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .with(predicate::function(move |cmd: &LendItemCommand| {
                cmd.item_id.value() == "1"
                    && cmd.person_id.value() == "2"
                    && cmd.loan_date == loan_date
            }))
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_existing_person_shows_success_message() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_lend().returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_existing_person_ok_returns_to_main() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::always(),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_lend().returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main()
    {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Main Menu"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_existing_person_loan_save_error_try_again_reshows_confirm() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        // First confirm → error → try again → confirm shown again
        sel.expect_select()
            .with(
                predicate::eq("Lend \"Drill\" to Alice?"),
                predicate::always(),
            )
            .times(2)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Try again"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        loan_repo
            .expect_lend()
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().times(2).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_new_person_confirm_calls_lend_shows_success_and_returns_to_main() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .with(predicate::function(move |cmd: &LendItemCommand| {
                cmd.item_id.value() == "1"
                    && cmd.person_id.value() == "2"
                    && cmd.loan_date == loan_date
            }))
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_new_person_shows_success_message() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_lend().returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_new_person_ok_returns_to_main() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::always(),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo.expect_lend().returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Main Menu"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn lend_new_item_new_person_loan_save_error_try_again_reshows_confirm() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Lend something out"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Lend \"Drill\" to Alice?"),
                predicate::always(),
            )
            .times(2)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Try again"));
        sel.expect_select()
            .with(
                predicate::eq("Lent \"Drill\" to Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_lend()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        loan_repo
            .expect_lend()
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().times(2).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_existing_person_confirm_calls_borrow_shows_success_and_returns_to_main() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .with(predicate::function(move |cmd: &BorrowItemCommand| {
                cmd.item_id.value() == "1"
                    && cmd.person_id.value() == "2"
                    && cmd.loan_date == loan_date
            }))
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_existing_person_shows_success_message() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_existing_person_ok_returns_to_main() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::always(),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_existing_person_loan_save_error_shows_error_screen_main_menu_returns_to_main()
     {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Main Menu"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_existing_person_loan_save_error_try_again_reshows_confirm() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select()
            .times(1)
            .returning(nav("Existing person"));
        sel.expect_select_person()
            .times(1)
            .returning(nav_person("2", "Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Borrow \"Drill\" from Alice?"),
                predicate::always(),
            )
            .times(2)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Try again"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo.expect_find_all().returning(|| Ok(vec![]));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().times(2).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_new_person_confirm_calls_borrow_shows_success_and_returns_to_main() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .with(predicate::function(move |cmd: &BorrowItemCommand| {
                cmd.item_id.value() == "1"
                    && cmd.person_id.value() == "2"
                    && cmd.loan_date == loan_date
            }))
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_new_person_shows_success_message() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_new_person_ok_returns_to_main() {
        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::always(),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock
            .expect_today()
            .returning(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_new_person_loan_save_error_shows_error_screen_main_menu_returns_to_main() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select().times(1).returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Main Menu"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        let mut clock = MockClock::new();
        clock.expect_today().returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }

    #[test]
    fn borrow_new_item_new_person_loan_save_error_try_again_reshows_confirm() {
        let loan_date = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();

        let mut sel = MockSelector::new();
        sel.expect_select()
            .times(1)
            .returning(nav("Borrow something"));
        sel.expect_select().times(1).returning(nav("New item"));
        sel.expect_input()
            .with(predicate::eq("Enter item name:"))
            .times(1)
            .returning(nav_input("Drill"));
        sel.expect_select().times(1).returning(nav("New person"));
        sel.expect_input()
            .with(predicate::eq("Enter person name:"))
            .times(1)
            .returning(nav_input("Alice"));
        sel.expect_select()
            .with(
                predicate::eq("Borrow \"Drill\" from Alice?"),
                predicate::always(),
            )
            .times(2)
            .returning(nav("Confirm"));
        sel.expect_select()
            .with(
                predicate::eq("failed to save loan"),
                predicate::eq(vec!["Try again".to_string(), "Main Menu".to_string()]),
            )
            .times(1)
            .returning(nav("Try again"));
        sel.expect_select()
            .with(
                predicate::eq("Borrowed \"Drill\" from Alice."),
                predicate::eq(vec!["OK".to_string()]),
            )
            .times(1)
            .returning(nav("OK"));
        sel.expect_select()
            .with(
                predicate::eq("What would you like to do?"),
                predicate::always(),
            )
            .times(1)
            .returning(nav("Quit"));

        let mut item_repo = MockItemRepository::new();
        item_repo.expect_add().returning(|_| Ok(ItemId::new("1")));
        item_repo.expect_find_by_id().returning(|_| {
            Ok(Item {
                id: ItemId::new("1"),
                description: "Drill".to_string(),
            })
        });
        let mut person_repo = MockPersonRepository::new();
        person_repo
            .expect_add()
            .returning(|_| Ok(PersonId::new("2")));
        person_repo.expect_find_by_id().returning(|_| {
            Ok(Person {
                id: PersonId::new("2"),
                name: "Alice".to_string(),
            })
        });
        let mut loan_repo = MockLoanRepository::new();
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Err(DomainError::LoanSaveFailed));
        loan_repo
            .expect_borrow()
            .times(1)
            .returning(|_| Ok(LoanId::new("10")));
        let mut clock = MockClock::new();
        clock.expect_today().times(2).returning(move || loan_date);

        run(&Ctx {
            selector: &sel,
            clock: &clock,
            printer: &MockPrinter::new(),
            item_repo: &item_repo,
            person_repo: &person_repo,
            loan_repo: &loan_repo,
        })
        .unwrap();
    }
}
