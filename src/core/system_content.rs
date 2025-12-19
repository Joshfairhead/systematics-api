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

/// Example: Simple Color vocabulary enum
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Orange,
    Purple,
    Pink,
    Brown,
    Black,
    White,
}

impl SystemContent for Color {
    fn provide(&self, count: usize) -> Vec<Self> {
        let colors = vec![
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Cyan,
            Color::Magenta,
            Color::Orange,
            Color::Purple,
            Color::Pink,
            Color::Brown,
            Color::Black,
            Color::White,
        ];

        colors.into_iter().take(count).collect()
    }

    fn display_name(&self) -> String {
        format!("{:?}", self)
    }
}

/// Example: Aristotelian Causes for Tetrad
#[derive(Debug, Clone, PartialEq)]
pub enum AristotelianCause {
    Formal,    // Ideal
    Final,     // Directive
    Efficient, // Instrumental
    Material,  // Ground
}

impl SystemContent for AristotelianCause {
    fn provide(&self, count: usize) -> Vec<Self> {
        let causes = vec![
            AristotelianCause::Formal,
            AristotelianCause::Final,
            AristotelianCause::Efficient,
            AristotelianCause::Material,
        ];

        causes.into_iter().take(count).collect()
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

    #[test]
    fn test_color_content() {
        let red = Color::Red;
        let colors = red.provide(3);
        assert_eq!(colors.len(), 3);
        assert_eq!(colors[0], Color::Red);
        assert_eq!(colors[1], Color::Green);
        assert_eq!(colors[2], Color::Blue);
    }

    #[test]
    fn test_aristotelian_causes() {
        let formal = AristotelianCause::Formal;
        let causes = formal.provide(4);
        assert_eq!(causes.len(), 4);
        assert_eq!(causes[0], AristotelianCause::Formal);
    }
}
