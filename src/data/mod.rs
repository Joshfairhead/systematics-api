//! Data module for populating the property graph.
//!
//! This module provides functions to build the complete graph with all
//! system data including Characters, Terms, Coordinates, Colours, and Links.

use crate::core::{
    Character, CoherenceAttribute, ConnectiveDesignation, Colour, Coordinate, Entry, Graph,
    Language, Link, Point3d, SystemName, Term, TermDesignation,
};
use std::f64::consts::PI;

/// Build the complete graph with all systems (1-12)
pub fn build_graph() -> Graph {
    let mut graph = Graph::new();

    // Add all system metadata
    add_system_metadata(&mut graph);

    // Add characters for canonical vocabulary
    add_canonical_characters(&mut graph);

    // Add terms, coordinates, colours, and links for each system
    for order in 1..=12 {
        add_system_entries(&mut graph, order);
    }

    graph
}

/// Add system-level metadata for all orders
fn add_system_metadata(graph: &mut Graph) {
    // System names
    let names = [
        (1, "Monad"),
        (2, "Dyad"),
        (3, "Triad"),
        (4, "Tetrad"),
        (5, "Pentad"),
        (6, "Hexad"),
        (7, "Heptad"),
        (8, "Octad"),
        (9, "Ennead"),
        (10, "Decad"),
        (11, "Undecad"),
        (12, "Dodecad"),
    ];

    for (order, name) in names {
        graph.add_entry(Entry::SystemName(SystemName::with_auto_id(order, name)));
    }

    // Coherence attributes
    let coherences = [
        (1, "Universality"),
        (2, "Complementarity"),
        (3, "Dynamism"),
        (4, "Activity Field"),
        (5, "Significance and Potential"),
        (6, "Coalescence"),
        (7, "Generation"),
        (8, "Self-Sufficiency"),
        (9, "Transformation"),
        (10, "Intrinsic Harmony"),
        (11, "Articulate Symmetry"),
        (12, "Harmony"),
    ];

    for (order, coherence) in coherences {
        graph.add_entry(Entry::CoherenceAttribute(CoherenceAttribute::with_auto_id(
            order, coherence,
        )));
    }

    // Term designations
    let term_designations = [
        (1, "Totality"),
        (2, "Poles"),
        (3, "Impulses"),
        (4, "Sources"),
        (5, "Limits"),
        (6, "Laws"),
        (7, "States"),
        (8, "Elements"),
        (9, "Principles"),
        (10, "Stages"),
        (11, "Categories"),
        (12, "Modalities"),
    ];

    for (order, designation) in term_designations {
        graph.add_entry(Entry::TermDesignation(TermDesignation::with_auto_id(
            order, designation,
        )));
    }

    // Connective designations
    let connective_designations = [
        (1, "Connectionless Unity"),
        (2, "Force"),
        (3, "Acts"),
        (4, "Interplays"),
        (5, "Mutualities"),
        (6, "Steps"),
        (7, "Intervals"),
        (8, "Components"),
        (9, "Transmutations"),
        (10, "Progressions"),
        (11, "Correlations"),
        (12, "Harmonies"),
    ];

    for (order, designation) in connective_designations {
        graph.add_entry(Entry::ConnectiveDesignation(
            ConnectiveDesignation::with_auto_id(order, designation),
        ));
    }
}

/// Add canonical vocabulary characters
fn add_canonical_characters(graph: &mut Graph) {
    let characters = [
        // Monad
        "Unity",
        // Dyad
        "Essence",
        "Existence",
        // Triad
        "Will",
        "Function",
        "Being",
        // Tetrad
        "Ideal",
        "Directive",
        "Instrumental",
        "Ground",
        // Pentad
        "Purpose",
        "Higher Potential",
        "Quintessence",
        "Lower Potential",
        "Source",
        // Hexad
        "Resources",
        "Values",
        "Options",
        "Criteria",
        "Facts",
        "Priorities",
        // Heptad
        "Insight",
        "Research",
        "Design",
        "Synthesis",
        "Application",
        "Delivery",
        "Value",
        // Octad
        "Smallest Significant Holon",
        "Critical Functions",
        "Supportive Platform",
        "Necessary Resourcing",
        "Integrative Totality",
        "Inherent Values",
        "Intrinsic Nature",
        "Organisational Modes",
    ];

    for value in characters {
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            value,
        )));
    }

    // Connective characters for Triad (Acts)
    for value in ["Act1", "Act2", "Act3"] {
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            value,
        )));
    }

    // Connective characters for Tetrad (Interplays)
    for value in [
        "Receptive Regard",
        "Effectual Compatibility",
        "Motivational Imperative",
        "Demonstrable Activity",
        "Material Mastery",
        "Technical Power",
    ] {
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            value,
        )));
    }

    // Generic terms for orders 9-12
    for i in 1..=12 {
        let value = format!("Term {}", i);
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            &value,
        )));
    }
}

/// Add entries for a specific system order
fn add_system_entries(graph: &mut Graph, order: u8) {
    let term_chars = get_term_characters(order);
    let coords = get_coordinates(order);
    let colours = get_colours(order);

    for position in 1..=order {
        let pos_idx = (position - 1) as usize;

        // Add term if character available
        if pos_idx < term_chars.len() {
            let char_id = format!(
                "char_canonical_{}",
                term_chars[pos_idx].to_lowercase().replace(' ', "_")
            );
            graph.add_entry(Entry::Term(Term::with_auto_id(order, position, &char_id)));
        }

        // Add coordinate
        if pos_idx < coords.len() {
            graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
                order,
                position,
                coords[pos_idx],
            )));
        }

        // Add colour (hex)
        if pos_idx < colours.len() {
            graph.add_entry(Entry::Colour(Colour::with_auto_id(
                order,
                position,
                Language::Hex,
                colours[pos_idx],
            )));
        }
    }

    // Add links (connectives and lines)
    add_system_links(graph, order);
}

/// Get term characters for an order
fn get_term_characters(order: u8) -> Vec<&'static str> {
    match order {
        1 => vec!["Unity"],
        2 => vec!["Essence", "Existence"],
        3 => vec!["Will", "Function", "Being"],
        4 => vec!["Ideal", "Directive", "Instrumental", "Ground"],
        5 => vec![
            "Purpose",
            "Higher Potential",
            "Quintessence",
            "Lower Potential",
            "Source",
        ],
        6 => vec![
            "Resources",
            "Values",
            "Options",
            "Criteria",
            "Facts",
            "Priorities",
        ],
        7 => vec![
            "Insight",
            "Research",
            "Design",
            "Synthesis",
            "Application",
            "Delivery",
            "Value",
        ],
        8 => vec![
            "Smallest Significant Holon",
            "Critical Functions",
            "Supportive Platform",
            "Necessary Resourcing",
            "Integrative Totality",
            "Inherent Values",
            "Intrinsic Nature",
            "Organisational Modes",
        ],
        // For orders 9-12, use generic terms
        9 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8", "Term 9",
        ],
        10 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8", "Term 9",
            "Term 10",
        ],
        11 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8", "Term 9",
            "Term 10", "Term 11",
        ],
        12 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8", "Term 9",
            "Term 10", "Term 11", "Term 12",
        ],
        _ => vec![],
    }
}

/// Get coordinates for an order (regular polygon vertices)
fn get_coordinates(order: u8) -> Vec<Point3d> {
    (0..order)
        .map(|i| {
            let angle = 2.0 * PI * (i as f64) / (order as f64) - PI / 2.0;
            Point3d::new(angle.cos(), angle.sin(), 0.0)
        })
        .collect()
}

/// Get position colours for an order
fn get_colours(order: u8) -> Vec<&'static str> {
    // Standard position colours
    let all_colours = [
        "#FF0000", // Red (position 1)
        "#0000FF", // Blue (position 2)
        "#E6E600", // Yellow (position 3)
        "#00FF00", // Green (position 4)
        "#FF00FF", // Magenta (position 5)
        "#00FFFF", // Cyan (position 6)
        "#FF8000", // Orange (position 7)
        "#8000FF", // Purple (position 8)
        "#FF0080", // Pink (position 9)
        "#80FF00", // Lime (position 10)
        "#0080FF", // Sky Blue (position 11)
        "#FFFFFF", // White (position 12)
    ];

    all_colours[0..(order as usize).min(12)].to_vec()
}

/// Add links (connectives and lines) for a system
fn add_system_links(graph: &mut Graph, order: u8) {
    // Add connective links for specific orders
    match order {
        3 => {
            // Triad: Acts between terms
            let acts = [("term_3_1", "term_3_2", "act1"), ("term_3_2", "term_3_3", "act2"), ("term_3_3", "term_3_1", "act3")];
            for (from, to, act) in acts {
                let char_id = format!("char_canonical_{}", act);
                graph.add_link(Link::connective(from, to, &char_id));
            }
        }
        4 => {
            // Tetrad: Interplays between all pairs
            let interplays = [
                ("term_4_1", "term_4_2", "receptive_regard"),
                ("term_4_1", "term_4_3", "effectual_compatibility"),
                ("term_4_1", "term_4_4", "motivational_imperative"),
                ("term_4_2", "term_4_3", "demonstrable_activity"),
                ("term_4_2", "term_4_4", "material_mastery"),
                ("term_4_3", "term_4_4", "technical_power"),
            ];
            for (from, to, name) in interplays {
                let char_id = format!("char_canonical_{}", name);
                graph.add_link(Link::connective(from, to, &char_id));
            }
        }
        _ => {
            // Generic edges for complete graph (all pairs)
            for i in 1..=order {
                for j in (i + 1)..=order {
                    graph.add_link(Link::edge(
                        format!("term_{}_{}", order, i),
                        format!("term_{}_{}", order, j),
                    ));
                }
            }
        }
    }

    // Add line links between all coordinates (complete graph)
    for i in 1..=order {
        for j in (i + 1)..=order {
            graph.add_link(Link::line(
                format!("coord_{}_{}", order, i),
                format!("coord_{}_{}", order, j),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_graph() {
        let graph = build_graph();

        // Should have entries for all 12 systems
        assert!(graph.system_name(1).is_some());
        assert!(graph.system_name(12).is_some());

        // Check triad
        assert_eq!(graph.system_name(3).unwrap().value, "Triad");
        assert_eq!(graph.coherence(3).unwrap().value, "Dynamism");
        assert_eq!(graph.term_designation(3).unwrap().value, "Impulses");

        // Check terms exist
        let triad_terms = graph.terms(3, None);
        assert_eq!(triad_terms.len(), 3);
    }

    #[test]
    fn test_coordinates() {
        let coords = get_coordinates(3);
        assert_eq!(coords.len(), 3);

        // Coordinates are on a unit circle starting at -PI/2 (top)
        // First coordinate should be near (0, 1, 0) - actually exactly (0, 1, 0) for the first point
        // cos(-PI/2) = 0, sin(-PI/2) = -1... wait, let's check the actual values
        // The formula gives us angle = 0 * 2PI/3 - PI/2 = -PI/2
        // x = cos(-PI/2) ≈ 0, y = sin(-PI/2) ≈ -1
        // So first point is at bottom, not top. That's fine for a regular polygon.
        assert!((coords[0].x).abs() < 0.01);
        assert!((coords[0].y + 1.0).abs() < 0.01); // y is approximately -1
    }

    #[test]
    fn test_slices() {
        let graph = build_graph();

        // Slice at (3, 1) should have Term, Coordinate, Colour
        let slice = graph.slice(3, 1);
        assert!(slice.len() >= 3);
    }
}
