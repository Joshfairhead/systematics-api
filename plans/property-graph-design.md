# Property Graph Design: Typed Entries + Explicit Links

## Design Principle

- **Entries** have structural info (order, position, type) embedded
- **Links** are explicit relationships between entries (Line, Edge, Connective)
- Some relationships are implicit (query by matching order/position)

**Note**: `Dictionary` (vocab type) and `ColourType` represent a *perspective* - relevant for AD4M integration later.

---

## Entry Types

### 1. Character (reusable across Terms and Connectives)
```rust
struct Character {
    id: String,
    language: Language,     // Canonical, Energy, Values, Society
    value: String,          // "Will", "act1", etc.
}
```

**Note**: Character is the semantic content, independent of structural position. Same Character can appear as a Term (at a position) or referenced by a Connective (as a link).

### 2. Term (positional entry referencing a Character)
```rust
struct Term {
    id: String,
    order: u8,              // 1-12
    position: u8,           // 1 to order
    character: String,      // ID of Character entry ("Will", "Function", "Being")
}
```

### 3. Coordinate
```rust
struct Coordinate {
    id: String,
    order: u8,
    position: u8,
    value: Point3d,         // {1.0, 0.0, 0.0}
}
```

### 5. TermDesignation (per-order, applies to all terms in system)
```rust
struct TermDesignation {
    id: String,
    order: u8,              // Order 3 → "Impulses" for all terms
    value: String,          // "Impulses", "Sources", "Limits", etc.
}
```

### 6. ConnectiveDesignation (per-order, applies to all connectives in system)
```rust
struct ConnectiveDesignation {
    id: String,
    order: u8,              // Order 3 → "Acts" for all connectives
    value: String,          // "Acts", "Interplays", "Steps", etc.
}
```

### 7. CoherenceAttribute (per-order)
```rust
struct CoherenceAttribute {
    id: String,
    order: u8,
    value: String,          // "Dynamism"
}
```

### 8. Colour
```rust
struct Colour {
    id: String,
    order: u8,              // System colour: Triad=Yellow, Tetrad=Green, etc.
    position: u8,           // Position colour within system
    language: Language,     // Hex, Name
    value: String,          // "#FF0000" or "Red"
}
```

### 9. SystemName (optional - can also be looked up from order)
```rust
struct SystemName {
    id: String,
    order: u8,
    value: String,          // "Triad", "Tetrad", etc.
}
```

---

## Link Types

Links are relationships **between entries**. All links share a common structure:

### Base Link Structure
```rust
struct Link {
    id: String,
    base: String,           // Entry ID (source)
    target: String,         // Entry ID (target)
    link_type: LinkType,    // Line, Edge, Connective
    tag: Option<String>,    // Optional payload
}
```

### Link Type Enum
```rust
enum LinkType {
    Line,                   // Coordinate → Coordinate
    Edge,                   // Node → Node (positional)
    Connective(Character),  // Term → Term (references a Character entry)
}
```

**Note**: Connective references a Character entry (the same Character type used by Term). The `base` and `target` on Link provide the Term entry IDs. The `tag` field can also reference a Character for additional metadata.

---

## Implicit Links (Derived by Query)

Many relationships are **derived** from matching fields - no explicit link needed:

| Relationship | Query | Morphism |
|-------------|-------|----------|
| Entries at same position | Any entry where `order == X && position == Y` (terms, coords, colours) | |
| Coherence for order | `CoherenceAttribute` where `order == X` | |
| Term designation for order | `TermDesignation` where `order == X` | |
| Connective designation for order | `ConnectiveDesignation` where `order == X` | |
| Connectives for order | `Link` where `link_type == Connective(..)` and base/target Terms have `order == X` | |
| Connectives for term | `Link` where `link_type == Connective(..)` and `(base == term.id OR target == term.id)` | |
| Cross-language isomorphism | Terms where `order == X && position == Y` with Characters in different languages | |
| Term ↔ Coordinate | Same `order` and `position` | |
| Term ↔ Colour | Same `order` and `position` | |

---

## Enums

### Entry Enum
```rust
/// Entry is a sum type for storing heterogeneous entries in a single collection (Vec<Entry>).
/// This enables the Graph to hold all entry types in one `entries` field, allowing unified
/// iteration and queries across all entry kinds.
///
/// Ordering: System-level metadata first, then positional entries
enum Entry {
    // System-level (per-order, no position)
    SystemName(SystemName),
    CoherenceAttribute(CoherenceAttribute),
    TermDesignation(TermDesignation),
    ConnectiveDesignation(ConnectiveDesignation),

    // Positional entries (per order+position)
    Term(Term),
    Colour(Colour),
    Coordinate(Coordinate),

    // Semantic content (reusable expressions)
    Character(Character),
}
```

### Language
```rust
enum Language {
    // Semantic vocabularies (for Character entries)
    Canonical,
    Energy,
    Values,
    Society,

    // Representation types (for Colour entries)
    Hex,
    Name,
}
```

---

## Example: Canonical Triad

### Entries
```
// System-level metadata
SystemName { order: 3, value: "Triad" }
CoherenceAttribute { order: 3, value: "Dynamism" }
TermDesignation { order: 3, value: "Impulses" }
ConnectiveDesignation { order: 3, value: "Acts" }

// Characters (semantic content, reusable)
Character { id: "char_will", language: Canonical, value: "Will" }
Character { id: "char_function", language: Canonical, value: "Function" }
Character { id: "char_being", language: Canonical, value: "Being" }
Character { id: "char_act1", language: Canonical, value: "act1" }
Character { id: "char_act2", language: Canonical, value: "act2" }
Character { id: "char_act3", language: Canonical, value: "act3" }

// Terms (positional, reference Characters)
Term { order: 3, position: 1, character: "char_will" }
Term { order: 3, position: 2, character: "char_function" }
Term { order: 3, position: 3, character: "char_being" }

// Coordinates
Coordinate { order: 3, position: 1, value: {0.0, 1.0, 0.0} }
Coordinate { order: 3, position: 2, value: {-0.866, -0.5, 0.0} }
Coordinate { order: 3, position: 3, value: {0.866, -0.5, 0.0} }

// Colours
Colour { order: 3, position: 1, language: Hex, value: "#FF0000" }
Colour { order: 3, position: 2, language: Hex, value: "#0000FF" }
Colour { order: 3, position: 3, language: Hex, value: "#E6E600" }
```

### Links
```
// Connective links (term → term, referencing Character entries)
Link { base: term_1.id, target: term_2.id, link_type: Connective(char_act1) }
Link { base: term_1.id, target: term_3.id, link_type: Connective(char_act2) }
Link { base: term_2.id, target: term_3.id, link_type: Connective(char_act3) }

// Line links (coordinate → coordinate)
Link { base: coord_1.id, target: coord_2.id, link_type: Line }
Link { base: coord_1.id, target: coord_3.id, link_type: Line }
Link { base: coord_2.id, target: coord_3.id, link_type: Line }
```

### Implicit Links (derived by query)
```
// Term at position 1's fiber (same order+position)
fiber(3, 1) = [term_1, coord_1, colour_1]

// Coherence for order
coherence(3) = Dynamism

// Connectives for term
connectives_for(3, 1) = [act1 (1→2), act2 (1→3)]

// Cross-vocab at same position (different language Characters)
isomorphic_terms(3, 1) = [Will (Canonical), Affirming (Energy), ...]
```

---

## Graph Structure

```rust
// AD4M: Graph is a Perspective
struct Graph {
    entries: Vec<Entry>,
    links: Vec<Link>,
}

impl Graph {
    // System query: all entries for order
    fn system(&self, order: u8) -> Vec<&Entry>;

    // Coherence for order
    fn coherence(&self, order: u8) -> Option<&CoherenceAttribute>;

    // Term designation for order
    fn term_designation(&self, order: u8) -> Option<&TermDesignation>;

    // Terms for order (optionally filtered by language)
    fn terms(&self, order: u8, language: Option<Language>) -> Vec<&Term>;

    // Character expressions for a language
    fn characters(&self, language: Language) -> Vec<&Character>;

    // Connective designation for order
    fn connective_designation(&self, order: u8) -> Option<&ConnectiveDesignation>;

    // Connectives: query by order, optionally by base/target position
    fn connectives(&self, order: u8, base: Option<u8>, target: Option<u8>) -> Vec<&Link>;

    // Fiber query: all entries at order+position
    fn fiber(&self, order: u8, position: u8) -> Vec<&Entry>;

    // Lines for order (explicit coordinate links)
    fn lines(&self, order: u8) -> Vec<&Link>;
}
```

---

## Benefits of This Design

1. **Character reuse** - Same Character type for Terms and Connectives; semantic content separate from structure
2. **Self-contained entries** - Each entry has all its structural info (order, position)
3. **Minimal explicit links** - Only Line/Edge/Connective need explicit storage
4. **Unified Language enum** - Single type for vocabularies (Canonical, Energy) and representations (Hex, Name)
5. **Simple queries** - Filter by order, position, language
6. **Type safety** - Different structs for different entry kinds
7. **Holochain/AD4M compatible** - Entries are content-addressable, Language maps to AD4M Language

---

## Notes

- **IDs**: Examples use human-readable IDs (e.g., `"char_will"`) for clarity. In production, these would be content-addressed hashes derived from the entry's content. This enables deduplication, immutability, and tamper-proof references. For the prototype, simple string IDs are sufficient.

---

## Files to Create

| File | Content |
|------|---------|
| `src/core/graph.rs` | Entry enum, Link struct, LinkType enum, Graph struct, queries |
| `src/core/entries.rs` | Character, Term, Coordinate, Colour, SystemName, CoherenceAttribute, TermDesignation, ConnectiveDesignation |
| `src/core/language.rs` | Language enum (Canonical, Energy, Values, Society, Hex, Name) |
| `src/core/mod.rs` | Add exports |
