//! Data module for populating the property graph.
//!
//! This module provides functions to build the complete graph with all
//! system data including Characters, Terms, Coordinates, Colours, and Links.

use crate::core::{
    Character, CoherenceAttribute, ConnectiveDesignation, Colour, Coordinate, Entry, Graph,
    Language, Link, Point3d, SystemName, Term, TermDesignation,
};

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

    // Index-based terms for orders 9-12
    for i in 1..=12 {
        let value = format!("Index {}", i);
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            &value,
        )));
    }
}


/// Get position sequence for orders 9-12 (position = term number)
/// For orders 1-8, positions are sequential 1..=order
fn get_position_sequence(order: u8) -> Vec<u8> {
    match order {
        // Ennead: positions clockwise from top-right
        9 => vec![5, 3, 6, 2, 8, 4, 9, 7, 1],
        // Decad: positions clockwise from top-left
        10 => vec![5, 3, 6, 9, 2, 8, 4, 10, 7, 1],
        // Undecad: positions clockwise from 12 o'clock
        11 => vec![11, 3, 6, 9, 2, 8, 4, 10, 7, 1, 5],
        // Dodecad: positions clockwise from 12 o'clock
        12 => vec![9, 6, 3, 5, 2, 10, 7, 8, 4, 11, 12, 1],
        // For orders 1-8, sequential positions
        _ => (1..=order).collect(),
    }
}

/// Add entries for a specific system order
fn add_system_entries(graph: &mut Graph, order: u8) {
    let term_chars = get_term_characters(order);
    let coords = get_coordinates(order);
    let colours = get_colours(order);
    let positions = get_position_sequence(order);

    for (idx, &position) in positions.iter().enumerate() {
        // Add term - position matches term number for orders 9-12
        if idx < term_chars.len() {
            let char_id = format!(
                "char_canonical_{}",
                term_chars[idx].to_lowercase().replace(' ', "_")
            );
            graph.add_entry(Entry::Term(Term::with_auto_id(order, position, &char_id)));
        }

        // Add coordinate
        if idx < coords.len() {
            graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
                order,
                position,
                coords[idx],
            )));
        }

        // Add colour
        if idx < colours.len() {
            graph.add_entry(Entry::Colour(Colour::with_auto_id(
                order,
                position,
                Language::Hex,
                colours[idx],
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
        4 => vec!["Ideal", "Ground", "Directive", "Instrumental"],
        // Pentad: 1=Quintessence, 2=Source, 3=Higher Potential, 4=Lower Potential, 5=Purpose
        5 => vec![
            "Quintessence",
            "Source",
            "Higher Potential",
            "Lower Potential",
            "Purpose",
        ],
        // Hexad: 1=Priorities, 2=Criteria, 3=Values, 4=Resources, 5=Options, 6=Facts
        6 => vec![
            "Priorities",
            "Criteria",
            "Values",
            "Resources",
            "Options",
            "Facts",
        ],
        // Heptad: 1=Insight, 2=Application, 3=Design, 4=Research, 5=Synthesis, 6=Delivery, 7=Value
        7 => vec![
            "Insight",
            "Application",
            "Design",
            "Research",
            "Synthesis",
            "Delivery",
            "Value",
        ],
        // Octad: Position N = Term mapping
        // 1=Inherent Values, 2=Critical Functions, 3=Organisational Modes, 4=Necessary Resourcing
        // 5=Intrinsic Nature, 6=Smallest Holon, 7=Integrative Totality, 8=Supportive Platform
        8 => vec![
            "Inherent Values",
            "Critical Functions",
            "Organisational Modes",
            "Necessary Resourcing",
            "Intrinsic Nature",
            "Smallest Significant Holon",
            "Integrative Totality",
            "Supportive Platform",
        ],
        // Ennead: Term N at Position N, sequence: 5, 3, 6, 2, 8, 4, 9, 7, 1
        9 => vec![
            "Term 5", "Term 3", "Term 6", "Term 2", "Term 8", "Term 4", "Term 9", "Term 7", "Term 1",
        ],
        // Decad: Term N at Position N, sequence: 5, 3, 6, 9, 2, 8, 4, 10, 7, 1
        10 => vec![
            "Term 5", "Term 3", "Term 6", "Term 9", "Term 2", "Term 8", "Term 4", "Term 10", "Term 7", "Term 1",
        ],
        // Undecad: Term N at Position N, sequence: 11, 3, 6, 9, 2, 8, 4, 10, 7, 1, 5
        11 => vec![
            "Term 11", "Term 3", "Term 6", "Term 9", "Term 2", "Term 8", "Term 4", "Term 10", "Term 7", "Term 1", "Term 5",
        ],
        // Dodecad: Term N at Position N, sequence: 9, 6, 3, 5, 2, 10, 7, 8, 4, 11, 12, 1
        12 => vec![
            "Term 9", "Term 6", "Term 3", "Term 5", "Term 2", "Term 10",
            "Term 7", "Term 8", "Term 4", "Term 11", "Term 12", "Term 1",
        ],
        _ => vec![],
    }
}

/// Get coordinates for an order (from curated data files)
fn get_coordinates(order: u8) -> Vec<Point3d> {
    match order {
        1 => vec![Point3d::new(0.0, 0.0, 0.0)],
        2 => vec![
            Point3d::new(-1.0, 0.0, 0.0),  // Essence (left)
            Point3d::new(1.0, 0.0, 0.0),   // Existence (right)
        ],
        3 => vec![
            Point3d::new(0.0, 1.0, 0.0),   // Will (top left)
            Point3d::new(0.0, -1.0, 0.0),  // Function (bottom left)
            Point3d::new(1.0, 0.0, 0.0),   // Being (right, midpoint vertically)
        ],
        4 => vec![
            Point3d::new(0.0, 1.0, 0.0),   // Ideal (top)
            Point3d::new(0.0, -1.0, 0.0),  // Ground (bottom)
            Point3d::new(1.0, 0.0, 0.0),   // Directive (right)
            Point3d::new(-1.0, 0.0, 0.0),  // Instrumental (left)
        ],
        5 => vec![
            Point3d::new(-0.75, 0.0, 0.0),   // Quintessence (left-center, middle)
            Point3d::new(1.0, -0.75, 0.0),   // Source (right, bottom)
            Point3d::new(0.0, 0.5, 0.0),     // Higher Potential (center, upper)
            Point3d::new(0.0, -0.5, 0.0),    // Lower Potential (center, lower)
            Point3d::new(1.0, 0.75, 0.0),    // Purpose (right, top)
        ],
        6 => vec![
            Point3d::new(-0.866, -0.5, 0.0),   // Priorities (lower left)
            Point3d::new(0.866, -0.5, 0.0),    // Criteria (lower right)
            Point3d::new(0.0, 1.0, 0.0),       // Values (top)
            Point3d::new(-0.866, 0.5, 0.0),    // Resources (upper left)
            Point3d::new(0.866, 0.5, 0.0),     // Options (upper right)
            Point3d::new(0.0, -1.0, 0.0),      // Facts (bottom)
        ],
        7 => vec![
            Point3d::new(0.0, 1.0, 0.0),              // Insight (top center)
            Point3d::new(-0.433884, -0.900969, 0.0),  // Application
            Point3d::new(0.974370, -0.222521, 0.0),   // Design
            Point3d::new(0.781831, 0.623489, 0.0),    // Research
            Point3d::new(0.433884, -0.900969, 0.0),   // Synthesis
            Point3d::new(-0.974370, -0.222521, 0.0),  // Delivery
            Point3d::new(-0.781831, 0.623489, 0.0),   // Value
        ],
        8 => vec![
            Point3d::new(-0.70710678118, 0.70710678118, 0.0),   // Inherent Values (upper left)
            Point3d::new(0.70710678118, -0.70710678118, 0.0),   // Critical Functions (lower right)
            Point3d::new(0.70710678118, 0.70710678118, 0.0),    // Organisational Modes (upper right)
            Point3d::new(-0.70710678118, -0.70710678118, 0.0),  // Necessary Resourcing (lower left)
            Point3d::new(0.0, 1.0, 0.0),                        // Intrinsic Nature (top)
            Point3d::new(1.0, 0.0, 0.0),                        // Smallest Significant Holon (right)
            Point3d::new(-1.0, 0.0, 0.0),                       // Integrative Totality (left)
            Point3d::new(0.0, -1.0, 0.0),                       // Supportive Platform (bottom)
        ],
        // Ennead: 9 points clockwise from 12 o'clock
        // Sequence: 5, 3, 6, 2, 8, 4, 9, 7, 1
        9 => vec![
            Point3d::new(0.0, 1.0, 0.0),                       // Position 5: 12 o'clock
            Point3d::new(0.64278760968, 0.76604444311, 0.0),   // Position 3
            Point3d::new(0.98480775301, 0.17364817767, 0.0),   // Position 6
            Point3d::new(0.86602540378, -0.5, 0.0),            // Position 2
            Point3d::new(0.34202014333, -0.93969262079, 0.0),  // Position 8
            Point3d::new(-0.34202014333, -0.93969262079, 0.0), // Position 4
            Point3d::new(-0.86602540378, -0.5, 0.0),           // Position 9
            Point3d::new(-0.98480775301, 0.17364817767, 0.0),  // Position 7
            Point3d::new(-0.64278760968, 0.76604444311, 0.0),  // Position 1
        ],
        // Decad: 10 points clockwise from top-left (position 1)
        // 36 degrees per step, starting at ~108 degrees (top-left)
        10 => vec![
            Point3d::new(-0.30901699437, 0.95105651630, 0.0),  // Position 1: top-left (~108°)
            Point3d::new(0.30901699437, 0.95105651630, 0.0),   // Position 2: top-right
            Point3d::new(0.80901699437, 0.58778525229, 0.0),   // Position 3
            Point3d::new(1.0, 0.0, 0.0),                       // Position 4: 3 o'clock
            Point3d::new(0.80901699437, -0.58778525229, 0.0),  // Position 5
            Point3d::new(0.30901699437, -0.95105651630, 0.0),  // Position 6
            Point3d::new(-0.30901699437, -0.95105651630, 0.0), // Position 7
            Point3d::new(-0.80901699437, -0.58778525229, 0.0), // Position 8
            Point3d::new(-1.0, 0.0, 0.0),                      // Position 9: 9 o'clock
            Point3d::new(-0.80901699437, 0.58778525229, 0.0),  // Position 10
        ],
        // Undecad: 11 points clockwise from 12 o'clock (position 1 at top)
        11 => vec![
            Point3d::new(0.0, 1.0, 0.0),                      // Position 1: 12 o'clock (top)
            Point3d::new(0.54064081745, 0.84125353283, 0.0),  // Position 2
            Point3d::new(0.909632, 0.415415, 0.0),            // Position 3
            Point3d::new(0.989821, -0.142315, 0.0),           // Position 4
            Point3d::new(0.755750, -0.654861, 0.0),           // Position 5
            Point3d::new(0.281733, -0.959493, 0.0),           // Position 6
            Point3d::new(-0.281733, -0.959493, 0.0),          // Position 7
            Point3d::new(-0.755750, -0.654861, 0.0),          // Position 8
            Point3d::new(-0.989821, -0.142315, 0.0),          // Position 9
            Point3d::new(-0.909632, 0.415415, 0.0),           // Position 10
            Point3d::new(-0.54064081745, 0.84125353283, 0.0), // Position 11
        ],
        // Dodecad: 12 points clockwise from 12 o'clock (position 1 at top)
        12 => vec![
            Point3d::new(0.0, 1.0, 0.0),                 // Position 1: 12 o'clock (top)
            Point3d::new(0.5, 0.86602540378, 0.0),       // Position 2: 1 o'clock
            Point3d::new(0.86602540378, 0.5, 0.0),       // Position 3: 2 o'clock
            Point3d::new(1.0, 0.0, 0.0),                 // Position 4: 3 o'clock
            Point3d::new(0.86602540378, -0.5, 0.0),      // Position 5: 4 o'clock
            Point3d::new(0.5, -0.86602540378, 0.0),      // Position 6: 5 o'clock
            Point3d::new(0.0, -1.0, 0.0),                // Position 7: 6 o'clock (bottom)
            Point3d::new(-0.5, -0.86602540378, 0.0),     // Position 8: 7 o'clock
            Point3d::new(-0.86602540378, -0.5, 0.0),     // Position 9: 8 o'clock
            Point3d::new(-1.0, 0.0, 0.0),                // Position 10: 9 o'clock
            Point3d::new(-0.86602540378, 0.5, 0.0),      // Position 11: 10 o'clock
            Point3d::new(-0.5, 0.86602540378, 0.0),      // Position 12: 11 o'clock
        ],
        _ => vec![],
    }
}

/// Get position colours for an order
/// Colors are assigned to positions starting at 12 o'clock, moving clockwise
fn get_colours(order: u8) -> Vec<&'static str> {
    // Color palette
    const RED: &str = "#FF0000";
    const BLUE: &str = "#0000FF";
    const YELLOW: &str = "#FFFF00";
    const GREEN: &str = "#099902";
    const PURPLE: &str = "#9900FF";
    const ORANGE: &str = "#FFA500";
    const LIGHT_BLUE: &str = "#00FFFF";
    const BROWN: &str = "#8B4513";
    const MAGENTA: &str = "#FF00FF";
    const WHITE: &str = "#FFFFFF";
    const SILVER: &str = "#C0C0C0";
    const GOLD: &str = "#FFD700";

    // Colors assigned to positions clockwise from 12 o'clock
    match order {
        1 => vec![RED],
        2 => vec![RED, BLUE],
        3 => vec![RED, BLUE, YELLOW],
        4 => vec![RED, BLUE, YELLOW, GREEN],
        5 => vec![RED, BLUE, YELLOW, GREEN, PURPLE],
        6 => vec![RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE],
        7 => vec![RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE, LIGHT_BLUE],
        8 => vec![RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE, LIGHT_BLUE, BROWN],
        // Ennead: colors match terms clockwise from top-left: 5=Purple, 3=Yellow, 6=Orange, 2=Blue, 8=Brown, 4=Green, 9=Magenta, 7=LightBlue, 1=Red
        9 => vec![PURPLE, YELLOW, ORANGE, BLUE, BROWN, GREEN, MAGENTA, LIGHT_BLUE, RED],
        // Decad: colors match terms clockwise from top-left: 5=Purple, 3=Yellow, 6=Orange, 9=Magenta, 2=Blue, 8=Brown, 4=Green, 10=White, 7=LightBlue, 1=Red
        10 => vec![PURPLE, YELLOW, ORANGE, MAGENTA, BLUE, BROWN, GREEN, WHITE, LIGHT_BLUE, RED],
        // Undecad: colors match terms clockwise from 12 o'clock: 11=Silver, 3=Yellow, 6=Orange, 9=Magenta, 2=Blue, 8=Brown, 4=Green, 10=White, 7=LightBlue, 1=Red, 5=Purple
        11 => vec![SILVER, YELLOW, ORANGE, MAGENTA, BLUE, BROWN, GREEN, WHITE, LIGHT_BLUE, RED, PURPLE],
        12 => vec![MAGENTA, ORANGE, YELLOW, PURPLE, BLUE, WHITE, LIGHT_BLUE, BROWN, GREEN, SILVER, GOLD, RED],
        _ => vec![],
    }
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
