#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub(super) trait Printer {
    fn print_error(&self, message: &str);
}

pub(super) struct ConsolePrinter;

impl Printer for ConsolePrinter {
    fn print_error(&self, message: &str) {
        println!("{}", message);
    }
}
