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
        // For orders 9-12, use index-based terms
        9 => vec![
            "Index 1", "Index 5", "Index 3", "Index 6", "Index 2", "Index 8", "Index 4", "Index 9", "Index 7",
        ],
        10 => vec![
            "Index 5", "Index 3", "Index 6", "Index 9", "Index 2", "Index 8", "Index 4", "Index 10", "Index 7", "Index 1",
        ],
        11 => vec![
            "Index 11", "Index 3", "Index 6", "Index 9", "Index 2", "Index 8", "Index 4", "Index 10", "Index 7", "Index 1", "Index 5",
        ],
        12 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8", "Term 9",
            "Term 10", "Term 11", "Term 12",
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
        9 => vec![
            Point3d::new(0.64278760968, 0.76604444311, 0.0),
            Point3d::new(0.98480775301, 0.17364817767, 0.0),
            Point3d::new(0.86602540378, -0.5, 0.0),
            Point3d::new(0.34202014333, -0.93969262079, 0.0),
            Point3d::new(-0.34202014333, -0.93969262079, 0.0),
            Point3d::new(-0.86602540378, -0.5, 0.0),
            Point3d::new(-0.98480775301, 0.17364817767, 0.0),
            Point3d::new(-0.64278760968, 0.76604444311, 0.0),
            Point3d::new(0.0, 1.0, 0.0),
        ],
        10 => vec![
            Point3d::new(-0.30901699437, 0.95105651630, 0.0),
            Point3d::new(0.30901699437, 0.95105651630, 0.0),
            Point3d::new(0.80901699437, 0.58778525229, 0.0),
            Point3d::new(1.0, 0.0, 0.0),
            Point3d::new(0.80901699437, -0.58778525229, 0.0),
            Point3d::new(0.30901699437, -0.95105651630, 0.0),
            Point3d::new(-0.30901699437, -0.95105651630, 0.0),
            Point3d::new(-0.80901699437, -0.58778525229, 0.0),
            Point3d::new(-1.0, 0.0, 0.0),
            Point3d::new(-0.80901699437, 0.58778525229, 0.0),
        ],
        11 => vec![
            Point3d::new(0.0, 1.0, 0.0),
            Point3d::new(0.54064081745, 0.84125353283, 0.0),
            Point3d::new(0.909632, 0.415415, 0.0),
            Point3d::new(0.989821, -0.142315, 0.0),
            Point3d::new(0.755750, -0.654861, 0.0),
            Point3d::new(0.281733, -0.959493, 0.0),
            Point3d::new(-0.281733, -0.959493, 0.0),
            Point3d::new(-0.755750, -0.654861, 0.0),
            Point3d::new(-0.989821, -0.142315, 0.0),
            Point3d::new(-0.909632, 0.415415, 0.0),
            Point3d::new(-0.54064081745, 0.84125353283, 0.0),
        ],
        12 => vec![
            Point3d::new(0.86602540378, -0.5, 0.0),      // Wholeness
            Point3d::new(-0.5, -0.86602540378, 0.0),     // Polarity
            Point3d::new(0.5, -0.86602540378, 0.0),      // Relatedness
            Point3d::new(0.0, -1.0, 0.0),                // Subsistence
            Point3d::new(-0.86602540378, -0.5, 0.0),     // Potentiality
            Point3d::new(-1.0, 0.0, 0.0),                // Repetition
            Point3d::new(1.0, 0.0, 0.0),                 // Structure
            Point3d::new(0.86602540378, 0.5, 0.0),       // Individuality
            Point3d::new(-0.86602540378, 0.5, 0.0),      // Pattern
            Point3d::new(0.5, 0.86602540378, 0.0),       // Creativity
            Point3d::new(-0.5, 0.86602540378, 0.0),      // Domination
            Point3d::new(0.0, 1.0, 0.0),                 // Autocracy
        ],
        _ => vec![],
    }
}

/// Get position colours for an order
/// Maps positions to their universal hyparchic color index
fn get_colours(order: u8) -> Vec<&'static str> {
    // Universal hyparchic color palette (by index)
    let hyparchic_colors = [
        "#FF0000", // Index 1: Red
        "#0000FF", // Index 2: Blue
        "#FFFF00", // Index 3: Yellow
        "#099902", // Index 4: Green
        "#9900FF", // Index 5: Purple
        "#FFA500", // Index 6: Orange
        "#00FFFF", // Index 7: Light Blue/Cyan
        "#8B4513", // Index 8: Brown
        "#FF00FF", // Index 9: Magenta
        "#FFFFFF", // Index 10: White
        "#C0C0C0", // Index 11: Grey/Silver
        "#FFD700", // Index 12: Bright Orange/Gold
    ];

    // Position-to-index mappings for each system
    match order {
        1 => vec![hyparchic_colors[0]], // Monad: position 1 → index 1 (red)
        2 => vec![
            hyparchic_colors[0], // Position 1 (Essence) → index 1 (red)
            hyparchic_colors[1], // Position 2 (Existence) → index 2 (blue)
        ],
        3 => vec![
            hyparchic_colors[0], // Position 1 (Will) → index 1 (red)
            hyparchic_colors[1], // Position 2 (Function) → index 2 (blue)
            hyparchic_colors[2], // Position 3 (Being) → index 3 (yellow)
        ],
        4 => vec![
            hyparchic_colors[0], // Position 1 (Ideal) → index 1 (red)
            hyparchic_colors[1], // Position 2 (Ground) → index 2 (blue)
            hyparchic_colors[2], // Position 3 (Directive) → index 3 (yellow)
            hyparchic_colors[3], // Position 4 (Instrumental) → index 4 (green)
        ],
        5 => vec![
            hyparchic_colors[0], // Position 1 (Quintessence) → index 1 (red)
            hyparchic_colors[1], // Position 2 (Source) → index 2 (blue)
            hyparchic_colors[2], // Position 3 (Higher Potential) → index 3 (yellow)
            hyparchic_colors[3], // Position 4 (Lower Potential) → index 4 (green)
            hyparchic_colors[4], // Position 5 (Purpose) → index 5 (purple)
        ],
        6 => vec![
            hyparchic_colors[0], // Position 1 (Priorities) → index 1 (red)
            hyparchic_colors[1], // Position 2 (Criteria) → index 2 (blue)
            hyparchic_colors[2], // Position 3 (Values) → index 3 (yellow)
            hyparchic_colors[3], // Position 4 (Resources) → index 4 (green)
            hyparchic_colors[4], // Position 5 (Options) → index 5 (purple)
            hyparchic_colors[5], // Position 6 (Facts) → index 6 (orange)
        ],
        7 => vec![
            hyparchic_colors[0], // Position 1 (Insight) → index 1 (red)
            hyparchic_colors[1], // Position 2 (Application) → index 2 (blue)
            hyparchic_colors[2], // Position 3 (Design) → index 3 (yellow)
            hyparchic_colors[3], // Position 4 (Research) → index 4 (green)
            hyparchic_colors[4], // Position 5 (Synthesis) → index 5 (purple)
            hyparchic_colors[5], // Position 6 (Delivery) → index 6 (orange)
            hyparchic_colors[6], // Position 7 (Value) → index 7 (light blue)
        ],
        8 => vec![
            hyparchic_colors[0], // Position 1 (Inherent Values) → index 1 (red)
            hyparchic_colors[1], // Position 2 (Critical Functions) → index 2 (blue)
            hyparchic_colors[2], // Position 3 (Organisational Modes) → index 3 (yellow)
            hyparchic_colors[3], // Position 4 (Necessary Resourcing) → index 4 (green)
            hyparchic_colors[4], // Position 5 (Intrinsic Nature) → index 5 (purple)
            hyparchic_colors[5], // Position 6 (Smallest Significant Holon) → index 6 (orange)
            hyparchic_colors[6], // Position 7 (Integrative Totality) → index 7 (light blue)
            hyparchic_colors[7], // Position 8 (Supportive Platform) → index 8 (brown)
        ],
        9 => vec![
            hyparchic_colors[0], // Position 1 (left of 12 o'clock) → index 1 (red)
            hyparchic_colors[4], // Position 2 (12 o'clock) → index 5 (purple)
            hyparchic_colors[2], // Position 3 → index 3 (yellow)
            hyparchic_colors[5], // Position 4 → index 6 (orange)
            hyparchic_colors[1], // Position 5 → index 2 (blue)
            hyparchic_colors[7], // Position 6 → index 8 (brown)
            hyparchic_colors[3], // Position 7 → index 4 (green)
            hyparchic_colors[8], // Position 8 → index 9 (pink)
            hyparchic_colors[6], // Position 9 → index 7 (light blue)
        ],
        10 => vec![
            hyparchic_colors[4], // Position 1 (left topmost) → index 5 (purple)
            hyparchic_colors[2], // Position 2 → index 3 (yellow)
            hyparchic_colors[5], // Position 3 → index 6 (orange)
            hyparchic_colors[8], // Position 4 → index 9 (pink)
            hyparchic_colors[1], // Position 5 → index 2 (blue)
            hyparchic_colors[7], // Position 6 → index 8 (brown)
            hyparchic_colors[3], // Position 7 → index 4 (green)
            hyparchic_colors[9], // Position 8 → index 10 (white)
            hyparchic_colors[6], // Position 9 → index 7 (light blue)
            hyparchic_colors[0], // Position 10 → index 1 (red)
        ],
        11 => vec![
            hyparchic_colors[10], // Position 1 (top, clockwise start) → index 11 (grey/silver)
            hyparchic_colors[2],  // Position 2 → index 3 (yellow)
            hyparchic_colors[5],  // Position 3 → index 6 (orange)
            hyparchic_colors[8],  // Position 4 → index 9 (magenta)
            hyparchic_colors[1],  // Position 5 → index 2 (blue)
            hyparchic_colors[7],  // Position 6 → index 8 (brown)
            hyparchic_colors[3],  // Position 7 → index 4 (green)
            hyparchic_colors[9],  // Position 8 → index 10 (white)
            hyparchic_colors[6],  // Position 9 → index 7 (light blue)
            hyparchic_colors[0],  // Position 10 → index 1 (red)
            hyparchic_colors[4],  // Position 11 → index 5 (purple)
        ],
        12 => vec![
            hyparchic_colors[0],  // Position 1 → index 1 (red)
            hyparchic_colors[1],  // Position 2 → index 2 (blue)
            hyparchic_colors[2],  // Position 3 → index 3 (yellow)
            hyparchic_colors[3],  // Position 4 → index 4 (green)
            hyparchic_colors[4],  // Position 5 → index 5 (purple)
            hyparchic_colors[5],  // Position 6 → index 6 (orange)
            hyparchic_colors[6],  // Position 7 → index 7 (light blue)
            hyparchic_colors[7],  // Position 8 → index 8 (brown)
            hyparchic_colors[8],  // Position 9 → index 9 (pink)
            hyparchic_colors[9],  // Position 10 → index 10 (white)
            hyparchic_colors[10], // Position 11 → index 11 (grey/silver)
            hyparchic_colors[11], // Position 12 → index 12 (bright orange/gold)
        ],
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
