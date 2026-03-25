use std::fmt;

pub struct PersonId(String);

impl PersonId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

pub struct Person {
    pub id: PersonId,
    pub name: String,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_uses_name() {
        let person = Person {
            id: PersonId::new("1"),
            name: "Alice".to_string(),
        };
        assert_eq!(person.to_string(), "Alice");
    }
}
