use chrono::NaiveDate;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub(super) trait Clock {
    fn today(&self) -> NaiveDate;
}

pub(super) struct SystemClock;

impl Clock for SystemClock {
    fn today(&self) -> NaiveDate {
        chrono::Local::now().date_naive()
    }
}
