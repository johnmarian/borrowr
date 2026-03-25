#[derive(Clone)]
pub struct LoanId(String);

impl LoanId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Clone)]
pub enum Direction {
    Lend,   // I lent it to them
    Borrow, // I borrowed it from them
}
