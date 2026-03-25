use std::fmt;

pub struct ItemId(String);

impl ItemId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

pub struct Item {
    pub id: ItemId,
    pub description: String,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_uses_description() {
        let item = Item {
            id: ItemId::new("1"),
            description: "Cordless drill".to_string(),
        };
        assert_eq!(item.to_string(), "Cordless drill");
    }
}
