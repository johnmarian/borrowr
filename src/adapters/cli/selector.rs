use crate::domain::item::Item;
use crate::domain::person::Person;
use crate::queries::loans::LoanView;
use inquire::{Select, Text};
#[cfg(test)]
use mockall::automock;
use std::fmt;

pub enum SelectOption<T: fmt::Display> {
    Value(T),
    Back,
}

impl<T: fmt::Display> fmt::Display for SelectOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SelectOption::Value(v) => write!(f, "{}", v),
            SelectOption::Back => write!(f, "Back"),
        }
    }
}

#[cfg_attr(test, automock)]
pub(super) trait Selector {
    fn select(&self, prompt: &str, options: Vec<String>) -> Option<String>;
    fn select_item(
        &self,
        prompt: &str,
        options: Vec<SelectOption<Item>>,
    ) -> Option<SelectOption<Item>>;
    fn select_person(
        &self,
        prompt: &str,
        options: Vec<SelectOption<Person>>,
    ) -> Option<SelectOption<Person>>;
    fn select_loan(
        &self,
        prompt: &str,
        options: Vec<SelectOption<LoanView>>,
    ) -> Option<SelectOption<LoanView>>;
    fn input(&self, prompt: &str) -> Option<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_displays_using_inner_display() {
        let opt = SelectOption::Value("Drill".to_string());
        assert_eq!(opt.to_string(), "Drill");
    }

    #[test]
    fn back_displays_as_back() {
        let opt: SelectOption<String> = SelectOption::Back;
        assert_eq!(opt.to_string(), "Back");
    }
}

pub(super) struct InquireSelector;

impl Selector for InquireSelector {
    fn select(&self, prompt: &str, options: Vec<String>) -> Option<String> {
        Select::new(prompt, options).prompt().ok()
    }

    fn select_item(
        &self,
        prompt: &str,
        options: Vec<SelectOption<Item>>,
    ) -> Option<SelectOption<Item>> {
        Select::new(prompt, options).prompt().ok()
    }

    fn select_person(
        &self,
        prompt: &str,
        options: Vec<SelectOption<Person>>,
    ) -> Option<SelectOption<Person>> {
        Select::new(prompt, options).prompt().ok()
    }

    fn select_loan(
        &self,
        prompt: &str,
        options: Vec<SelectOption<LoanView>>,
    ) -> Option<SelectOption<LoanView>> {
        Select::new(prompt, options).prompt().ok()
    }

    fn input(&self, prompt: &str) -> Option<String> {
        Text::new(prompt).prompt().ok()
    }
}
