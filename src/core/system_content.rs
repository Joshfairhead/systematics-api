use std::fmt::Debug;

/// SystemContent represents swappable content/vocabulary for a System
/// This can be strings, enums, complex structs, or any data type
pub trait SystemContent: Clone + Debug {
    /// Provide content items for a system of given order
    /// Should return exactly `count` items
    fn provide(&self, count: usize) -> Vec<Self>;

    /// Optional: Get a display name for this content item
    fn display_name(&self) -> String {
        format!("{:?}", self)
    }
}

/// Example: String-based content
impl SystemContent for String {
    fn provide(&self, count: usize) -> Vec<Self> {
        (1..=count)
            .map(|i| format!("{} {}", self, i))
            .collect()
    }

    fn display_name(&self) -> String {
        self.clone()
    }
}

/// Example: Generic numbered content
#[derive(Debug, Clone, PartialEq)]
pub struct NumberedItem {
    pub label: String,
    pub number: usize,
}

impl SystemContent for NumberedItem {
    fn provide(&self, count: usize) -> Vec<Self> {
        (1..=count)
            .map(|i| NumberedItem {
                label: self.label.clone(),
                number: i,
            })
            .collect()
    }

    fn display_name(&self) -> String {
        format!("{} {}", self.label, self.number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_content() {
        let base = String::from("Item");
        let items = base.provide(3);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "Item 1");
        assert_eq!(items[2], "Item 3");
    }

}
