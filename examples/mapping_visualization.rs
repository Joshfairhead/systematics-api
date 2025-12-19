use systematics::core::{
    hyparchic_registry::{Color, HyparchicRegistry, Index},
    system_topology::SystemTopology,
};

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("     SYSTEMATICS MAPPING VISUALIZATION");
    println!("     Coordinate → Index → Color → Term Chain");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Display the Hyparchic Registry first
    print_hyparchic_registry();

    // Display mappings for each system
    for order in 1..=12 {
        print_system_mappings(order);
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("VERIFICATION CHECKLIST:");
    println!("═══════════════════════════════════════════════════════════════");
    println!("☐ Are coordinate positions geometrically correct?");
    println!("☐ Does Color→Index mapping match (1→Red, 2→Blue, 3→Yellow, etc.)?");
    println!("☐ Are terms assigned to correct coordinate positions?");
    println!("☐ Does the structure make sense for each system type?");
    println!("═══════════════════════════════════════════════════════════════\n");
}

fn print_hyparchic_registry() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  HYPARCHIC REGISTRY (Universal Color ↔ Index Mapping)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let pairs = HyparchicRegistry::all_pairs();
    for (index, color) in pairs {
        println!("  Index {:2} ↔ {:<20}", index.value(), format!("{}", color));
    }
    println!();
}

fn print_system_mappings(order: u8) {
    let system_name = get_system_name(order);

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ {} (Order: {})", system_name, order);
    println!("└─────────────────────────────────────────────────────────────┘");

    match SystemTopology::get_coordinates(order) {
        Ok(coords) => {
            println!("  Coordinates → Index → Color");
            println!("  ─────────────────────────────────────────────────────────");

            for (position, coord) in coords.iter().enumerate() {
                let index = Index::from_zero_based(position).unwrap();
                let color = HyparchicRegistry::get_color(index);

                println!(
                    "  Position {} │ ({:7.3}, {:7.3}, {:7.3}) → Index {:2} → {}",
                    position,
                    coord.x,
                    coord.y,
                    coord.z,
                    index.value(),
                    color
                );
            }

            // Show connectivity information
            let matrix = SystemTopology::get_adjacency_matrix(order);
            let connection_count = count_connections(&matrix);
            println!("\n  Connectivity: {} total edges (complete graph: K{})",
                     connection_count, order);
        }
        Err(e) => {
            println!("  ERROR: {}", e);
        }
    }

    println!();
}

fn get_system_name(order: u8) -> &'static str {
    match order {
        1 => "MONAD",
        2 => "DYAD",
        3 => "TRIAD",
        4 => "TETRAD",
        5 => "PENTAD",
        6 => "HEXAD",
        7 => "HEPTAD",
        8 => "OCTAD",
        9 => "ENNEAD",
        10 => "DECAD",
        11 => "UNDECAD",
        12 => "DODECAD",
        _ => "UNKNOWN",
    }
}

fn count_connections(matrix: &[Vec<bool>]) -> usize {
    let mut count = 0;
    for row in matrix {
        for &connected in row {
            if connected {
                count += 1;
            }
        }
    }
    count / 2 // Each edge counted twice
}
