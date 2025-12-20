use systematics::core::{
    fiber::Fiber,
    system::System,
    system_topology::SystemTopology,
    universal_index::UniversalIndex,
};

fn main() {
    println!("=== Systematics Framework: Bidirectional Construction Examples ===\n");

    // ==================== METHOD A: TOP-DOWN FACTORY ====================
    println!("METHOD A: Top-Down Factory Pattern");
    println!("-----------------------------------\n");

    example_1_triad_with_colors();
    println!();

    example_2_tetrad_with_causes();
    println!();

    example_3_pentad_with_strings();
    println!();

    // ==================== METHOD B: BOTTOM-UP ASSEMBLY ====================
    println!("\nMETHOD B: Bottom-Up Assembly (Inference)");
    println!("------------------------------------------\n");

    example_4_assemble_dyad();
    println!();

    example_5_assemble_hexad();
    println!();

    example_6_invalid_assembly();
    println!();

    // ==================== COMBINED EXAMPLE ====================
    println!("\nCOMBINED: Using Both Patterns");
    println!("------------------------------\n");

    example_7_generate_and_decompose();
}

/// Example 1: Generate a Triad using String content
fn example_1_triad_with_colors() {
    println!("Example 1: Creating a Triad with String Content");

    match System::generate(3, String::from("Item")) {
        Ok(triad) => {
            println!("✓ Generated Triad (Order: {})", triad.order());

            for fiber in triad.fibers() {
                println!(
                    "  Fiber {}: {} | Content: {:?} | Position: ({:.2}, {:.2})",
                    fiber.index().value(),
                    fiber.system_name(),
                    fiber.content(),
                    fiber.coordinates().x,
                    fiber.coordinates().y
                );
            }

            println!("  Coherence: {}", triad.fibers()[0].coherence());
            println!(
                "  Term Designation: {:?}",
                triad.fibers()[0].term_designation()
            );
        }
        Err(e) => println!("✗ Error: {}", e),
    }
}

/// Example 2: Generate a Tetrad using String content
fn example_2_tetrad_with_causes() {
    println!("Example 2: Creating a Tetrad with String Content");

    match System::generate(4, String::from("Element")) {
        Ok(tetrad) => {
            println!("✓ Generated Tetrad (Order: {})", tetrad.order());

            for fiber in tetrad.fibers() {
                println!(
                    "  Fiber {}: {:?} | Essence: {} ({})",
                    fiber.index().value(),
                    fiber.content(),
                    fiber.system_name(),
                    fiber.coherence()
                );
            }

            // Check connectivity
            let idx1 = UniversalIndex::new(1).unwrap();
            let idx2 = UniversalIndex::new(2).unwrap();
            println!(
                "  Fiber 1 → Fiber 2 connected: {}",
                tetrad.is_connected(idx1, idx2).unwrap()
            );
        }
        Err(e) => println!("✗ Error: {}", e),
    }
}

/// Example 3: Generate a Pentad using Strings
fn example_3_pentad_with_strings() {
    println!("Example 3: Creating a Pentad with String content");

    match System::generate(5, String::from("Level")) {
        Ok(pentad) => {
            println!("✓ Generated Pentad (Order: {})", pentad.order());

            for fiber in pentad.fibers() {
                println!(
                    "  Fiber {}: {} | Coherence: {}",
                    fiber.index().value(),
                    fiber.content(),
                    fiber.coherence()
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
}

/// Example 4: Assemble a Dyad from raw fibers
fn example_4_assemble_dyad() {
    println!("Example 4: Assembling a Dyad from raw Fibers");

    let coords = SystemTopology::get_coordinates(2).unwrap();

    let fiber1 = Fiber::new(
        UniversalIndex::new(1).unwrap(),
        coords[0],
        String::from("Essence"),
    );

    let fiber2 = Fiber::new(
        UniversalIndex::new(2).unwrap(),
        coords[1],
        String::from("Existence"),
    );

    match System::try_assemble(vec![fiber1, fiber2]) {
        Ok(dyad) => {
            println!("✓ Assembled Dyad (Order: {})", dyad.order());
            println!("  Fibers validated and assembled successfully");

            for fiber in dyad.fibers() {
                println!(
                    "  Fiber {}: {} | System: {}",
                    fiber.index().value(),
                    fiber.content(),
                    fiber.system_name()
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
}

/// Example 5: Assemble a Hexad with validation
fn example_5_assemble_hexad() {
    println!("Example 5: Assembling a Hexad (6 fibers) from components");

    let coords = SystemTopology::get_coordinates(6).unwrap();

    let labels = vec![
        "Priorities",
        "Facts",
        "Criteria",
        "Options",
        "Values",
        "Resources",
    ];

    let mut fibers = Vec::new();
    for (i, label) in labels.iter().enumerate() {
        let fiber = Fiber::new(
            UniversalIndex::new((i + 1) as u8).unwrap(),
            coords[i],
            String::from(*label),
        );
        fibers.push(fiber);
    }

    match System::try_assemble(fibers) {
        Ok(hexad) => {
            println!("✓ Assembled Hexad (Order: {})", hexad.order());
            println!("  System Name: {}", hexad.fibers()[0].system_name());
            println!("  Coherence: {}", hexad.fibers()[0].coherence());

            // Show all connections for first fiber
            let idx1 = UniversalIndex::new(1).unwrap();
            let connections = hexad.connections(idx1).unwrap();
            println!(
                "  Fiber 1 is connected to {} other fibers",
                connections.len()
            );
        }
        Err(e) => println!("✗ Error: {}", e),
    }
}

/// Example 6: Attempt invalid assembly (demonstrates validation)
fn example_6_invalid_assembly() {
    println!("Example 6: Invalid Assembly (missing index)");

    let coords = SystemTopology::get_coordinates(3).unwrap();

    // Missing index 1, has 2 and 3 instead
    let fiber2 = Fiber::new(
        UniversalIndex::new(2).unwrap(),
        coords[1],
        String::from("Second"),
    );

    let fiber3 = Fiber::new(
        UniversalIndex::new(3).unwrap(),
        coords[2],
        String::from("Third"),
    );

    match System::try_assemble(vec![fiber2, fiber3]) {
        Ok(_) => println!("✓ Unexpectedly succeeded"),
        Err(e) => println!("✗ Expected error caught: {}", e),
    }
}

/// Example 7: Generate, extract fibers, and reassemble
fn example_7_generate_and_decompose() {
    println!("Example 7: Generate → Decompose → Reassemble");

    // Step 1: Generate a Triad
    println!("Step 1: Generate original Triad");
    let original = System::generate(3, String::from("Test")).unwrap();
    println!("  ✓ Created with {} fibers", original.fibers().len());

    // Step 2: Extract fibers (simulate receiving them as data)
    println!("\nStep 2: Extract fibers as raw data");
    let mut extracted_fibers = Vec::new();
    for fiber in original.fibers() {
        println!("  → Extracted Fiber {}", fiber.index().value());
        extracted_fibers.push(fiber.clone());
    }

    // Step 3: Reassemble from fibers
    println!("\nStep 3: Reassemble from raw fibers");
    match System::try_assemble(extracted_fibers) {
        Ok(reassembled) => {
            println!("  ✓ Reassembled successfully!");
            println!("  Order: {}", reassembled.order());
            println!("  System validated: {}", reassembled.fibers()[0].system_name());

            // Verify connectivity preserved
            let idx1 = UniversalIndex::new(1).unwrap();
            let connections = reassembled.connections(idx1).unwrap();
            println!("  Connectivity preserved: {} connections", connections.len());
        }
        Err(e) => println!("  ✗ Error: {}", e),
    }
}
