use async_graphql::*;
use crate::core::topology::Node;
use crate::core::geometry::Coordinates as CoreCoordinates;

/// A coordinate in 2D or 3D space
#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[Object]
impl Coordinate {
    async fn x(&self) -> f64 {
        self.x
    }

    async fn y(&self) -> f64 {
        self.y
    }

    async fn z(&self) -> Option<f64> {
        self.z
    }
}

impl From<CoreCoordinates> for Coordinate {
    fn from(coord: CoreCoordinates) -> Self {
        Self {
            x: coord.x,
            y: coord.y,
            z: coord.z,
        }
    }
}

/// A line connecting two coordinates
#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub start: Coordinate,
    pub end: Coordinate,
}

#[Object]
impl Line {
    async fn start(&self) -> Coordinate {
        self.start
    }

    async fn end(&self) -> Coordinate {
        self.end
    }
}

/// A topological edge connecting two indices
#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub from: Node,
    pub to: Node,
}

#[Object]
impl Edge {
    async fn from(&self) -> i32 {
        self.from as i32
    }

    async fn to(&self) -> i32 {
        self.to as i32
    }
}

/// A connector between two terms
#[derive(Clone, Debug)]
pub struct Connector {
    pub name: String,
    pub from_term: String,
    pub to_term: String,
}

#[Object]
impl Connector {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn from_term(&self) -> &str {
        &self.from_term
    }

    async fn to_term(&self) -> &str {
        &self.to_term
    }
}

/// Metadata about a system
#[derive(Clone, Debug)]
pub struct SystemMetadata {
    pub name: String,
    pub coherence_attribute: String,
    pub term_designation: String,
    pub connective_designation: String,
    pub source: String,
}

#[Object]
impl SystemMetadata {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn coherence_attribute(&self) -> &str {
        &self.coherence_attribute
    }

    async fn term_designation(&self) -> &str {
        &self.term_designation
    }

    async fn connective_designation(&self) -> &str {
        &self.connective_designation
    }

    async fn source(&self) -> &str {
        &self.source
    }
}

/// A complete system with all its data
#[derive(Clone, Debug)]
pub struct System {
    pub name: String,
    pub coherence_attribute: String,
    pub term_designation: String,
    pub connective_designation: String,
    pub source: String,
    pub term_characters: Vec<String>,
    pub connective_characters: Vec<Connector>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub points: Vec<Coordinate>,
    pub lines: Vec<Line>,
}

#[Object]
impl System {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn coherence_attribute(&self) -> &str {
        &self.coherence_attribute
    }

    async fn term_designation(&self) -> &str {
        &self.term_designation
    }

    async fn connective_designation(&self) -> &str {
        &self.connective_designation
    }

    async fn source(&self) -> &str {
        &self.source
    }

    async fn term_characters(&self) -> Vec<Term> {
        self.term_characters
            .iter()
            .enumerate()
            .map(|(i, name)| Term {
                name: name.clone(),
                system_name: self.name.clone(),
                node: i,
                coordinate: self.points.get(i).copied(),
            })
            .collect()
    }

    async fn connective_characters(&self) -> &[Connector] {
        &self.connective_characters
    }

    async fn nodes(&self) -> Vec<i32> {
        self.nodes.iter().map(|&i| i as i32).collect()
    }

    async fn edges(&self) -> &[Edge] {
        &self.edges
    }

    async fn points(&self) -> &[Coordinate] {
        &self.points
    }

    async fn lines(&self) -> &[Line] {
        &self.lines
    }
}

/// A term within a system
#[derive(Clone, Debug)]
pub struct Term {
    pub name: String,
    pub system_name: String,
    pub node: usize,
    pub coordinate: Option<Coordinate>,
}

#[Object]
impl Term {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn system_name(&self) -> &str {
        &self.system_name
    }

    async fn node(&self) -> i32 {
        self.node as i32
    }

    async fn coordinate(&self) -> Option<Coordinate> {
        self.coordinate
    }

    async fn system(&self, ctx: &Context<'_>) -> Result<Option<System>> {
        let query_root = ctx.data::<QueryRoot>()?;
        Ok(query_root.get_system(&self.system_name))
    }
}

/// Root query object
#[derive(Clone, Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get a system by name
    async fn system(&self, name: String) -> Option<System> {
        self.get_system(&name)
    }

    /// Get all available systems
    async fn all_systems(&self) -> Vec<System> {
        self.get_all_systems()
    }

    /// Get just the geometry (points) for a system
    async fn system_geometry(&self, name: String) -> Option<Vec<Coordinate>> {
        self.get_system(&name).map(|s| s.points)
    }

    /// Get just the topology (edges) for a system
    async fn system_topology(&self, name: String) -> Option<Vec<Edge>> {
        self.get_system(&name).map(|s| s.edges)
    }

    /// Get just the vocabulary (terms) for a system
    async fn system_vocabulary(&self, name: String) -> Option<Vec<Term>> {
        self.get_system(&name).map(|s| {
            s.term_characters
                .iter()
                .enumerate()
                .map(|(i, name_str)| Term {
                    name: name_str.clone(),
                    system_name: s.name.clone(),
                    node: i,
                    coordinate: s.points.get(i).copied(),
                })
                .collect()
        })
    }

    /// Get just the metadata for a system
    async fn system_metadata(&self, name: String) -> Option<SystemMetadata> {
        self.get_system(&name).map(|s| SystemMetadata {
            name: s.name,
            coherence_attribute: s.coherence_attribute,
            term_designation: s.term_designation,
            connective_designation: s.connective_designation,
            source: s.source,
        })
    }

    /// Find a term by name across all systems
    async fn term(&self, name: String) -> Option<Term> {
        for system in self.get_all_systems() {
            for (i, term_name) in system.term_characters.iter().enumerate() {
                if term_name.eq_ignore_ascii_case(&name) {
                    return Some(Term {
                        name: term_name.clone(),
                        system_name: system.name.clone(),
                        node: i,
                        coordinate: system.points.get(i).copied(),
                    });
                }
            }
        }
        None
    }

    /// Get all terms in a specific system
    async fn terms_in_system(&self, system_name: String) -> Option<Vec<Term>> {
        self.get_system(&system_name).map(|s| {
            s.term_characters
                .iter()
                .enumerate()
                .map(|(i, name_str)| Term {
                    name: name_str.clone(),
                    system_name: s.name.clone(),
                    node: i,
                    coordinate: s.points.get(i).copied(),
                })
                .collect()
        })
    }

    /// Find systems by coherence attribute
    async fn systems_by_coherence_attribute(&self, attribute: String) -> Vec<System> {
        self.get_all_systems()
            .into_iter()
            .filter(|s| s.coherence_attribute.eq_ignore_ascii_case(&attribute))
            .collect()
    }

    /// Find systems by term designation
    async fn systems_by_term_designation(&self, designation: String) -> Vec<System> {
        self.get_all_systems()
            .into_iter()
            .filter(|s| s.term_designation.eq_ignore_ascii_case(&designation))
            .collect()
    }
}

impl QueryRoot {
    pub fn get_system(&self, name: &str) -> Option<System> {
        match name.to_lowercase().as_str() {
            "monad" => Some(self.build_monad()),
            "dyad" => Some(self.build_dyad()),
            "triad" => Some(self.build_triad()),
            "tetrad" => Some(self.build_tetrad()),
            "pentad" => Some(self.build_pentad()),
            "hexad" => Some(self.build_hexad()),
            "heptad" => Some(self.build_heptad()),
            "octad" => Some(self.build_octad()),
            "ennead" => Some(self.build_ennead()),
            "decad" => Some(self.build_decad()),
            "undecad" => Some(self.build_undecad()),
            "dodecad" => Some(self.build_dodecad()),
            _ => None,
        }
    }

    pub fn get_all_systems(&self) -> Vec<System> {
        vec![
            self.build_monad(),
            self.build_dyad(),
            self.build_triad(),
            self.build_tetrad(),
            self.build_pentad(),
            self.build_hexad(),
            self.build_heptad(),
            self.build_octad(),
            self.build_ennead(),
            self.build_decad(),
            self.build_undecad(),
            self.build_dodecad(),
        ]
    }

    fn build_monad(&self) -> System {
        use crate::data::by_system::monad::MonadSystem;
        System {
            name: MonadSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: MonadSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: MonadSystem::TERM_DESIGNATION.to_string(),
            connective_designation: MonadSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: MonadSystem::SOURCE.to_string(),
            term_characters: MonadSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: MonadSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: MonadSystem::NODES.to_vec(),
            edges: MonadSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: MonadSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: MonadSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }

    fn build_dyad(&self) -> System {
        use crate::data::by_system::dyad::DyadSystem;
        System {
            name: DyadSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: DyadSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: DyadSystem::TERM_DESIGNATION.to_string(),
            connective_designation: DyadSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: DyadSystem::SOURCE.to_string(),
            term_characters: DyadSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: DyadSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: DyadSystem::NODES.to_vec(),
            edges: DyadSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: DyadSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: DyadSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }

    fn build_triad(&self) -> System {
        use crate::data::by_system::triad::TriadSystem;
        System {
            name: TriadSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: TriadSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: TriadSystem::TERM_DESIGNATION.to_string(),
            connective_designation: TriadSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: TriadSystem::SOURCE.to_string(),
            term_characters: TriadSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: TriadSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: TriadSystem::NODES.to_vec(),
            edges: TriadSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: TriadSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: TriadSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }

    fn build_tetrad(&self) -> System {
        use crate::data::by_system::tetrad::TetradSystem;
        System {
            name: TetradSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: TetradSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: TetradSystem::TERM_DESIGNATION.to_string(),
            connective_designation: TetradSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: TetradSystem::SOURCE.to_string(),
            term_characters: TetradSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: TetradSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: TetradSystem::NODES.to_vec(),
            edges: TetradSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: TetradSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: TetradSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }

    fn build_pentad(&self) -> System {
        use crate::data::by_system::pentad::PentadSystem;
        System {
            name: PentadSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: PentadSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: PentadSystem::TERM_DESIGNATION.to_string(),
            connective_designation: PentadSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: PentadSystem::SOURCE.to_string(),
            term_characters: PentadSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: PentadSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: PentadSystem::NODES.to_vec(),
            edges: PentadSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: PentadSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: PentadSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }

    fn build_hexad(&self) -> System {
        use crate::data::by_system::hexad::HexadSystem;
        System {
            name: HexadSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: HexadSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: HexadSystem::TERM_DESIGNATION.to_string(),
            connective_designation: HexadSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: HexadSystem::SOURCE.to_string(),
            term_characters: HexadSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: HexadSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: HexadSystem::NODES.to_vec(),
            edges: HexadSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: HexadSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: HexadSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }

    fn build_heptad(&self) -> System {
        use crate::data::by_system::heptad::HeptadSystem;
        System {
            name: HeptadSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: HeptadSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: HeptadSystem::TERM_DESIGNATION.to_string(),
            connective_designation: HeptadSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: HeptadSystem::SOURCE.to_string(),
            term_characters: HeptadSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: HeptadSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: HeptadSystem::NODES.to_vec(),
            edges: HeptadSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: HeptadSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: HeptadSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }

    fn build_octad(&self) -> System {
        use crate::data::by_system::octad::OctadSystem;
        System {
            name: OctadSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: OctadSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: OctadSystem::TERM_DESIGNATION.to_string(),
            connective_designation: OctadSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: OctadSystem::SOURCE.to_string(),
            term_characters: OctadSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: OctadSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: OctadSystem::NODES.to_vec(),
            edges: OctadSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: OctadSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: OctadSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }

    fn build_ennead(&self) -> System {
        use crate::data::by_system::ennead::EnneadSystem;
        System {
            name: EnneadSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: EnneadSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: EnneadSystem::TERM_DESIGNATION.to_string(),
            connective_designation: EnneadSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: EnneadSystem::SOURCE.to_string(),
            term_characters: EnneadSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: EnneadSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: EnneadSystem::NODES.to_vec(),
            edges: EnneadSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: EnneadSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: EnneadSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }

    fn build_decad(&self) -> System {
        use crate::data::by_system::decad::DecadSystem;
        System {
            name: DecadSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: DecadSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: DecadSystem::TERM_DESIGNATION.to_string(),
            connective_designation: DecadSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: DecadSystem::SOURCE.to_string(),
            term_characters: DecadSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: DecadSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: DecadSystem::NODES.to_vec(),
            edges: DecadSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: DecadSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: DecadSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }

    fn build_undecad(&self) -> System {
        use crate::data::by_system::undecad::UndecadSystem;
        System {
            name: UndecadSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: UndecadSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: UndecadSystem::TERM_DESIGNATION.to_string(),
            connective_designation: UndecadSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: UndecadSystem::SOURCE.to_string(),
            term_characters: UndecadSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: UndecadSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: UndecadSystem::NODES.to_vec(),
            edges: UndecadSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: UndecadSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: UndecadSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }

    fn build_dodecad(&self) -> System {
        use crate::data::by_system::dodecad::DodecadSystem;
        System {
            name: DodecadSystem::SYSTEM_NAME.to_string(),
            coherence_attribute: DodecadSystem::COHERENCE_ATTRIBUTE.to_string(),
            term_designation: DodecadSystem::TERM_DESIGNATION.to_string(),
            connective_designation: DodecadSystem::CONNECTIVE_DESIGNATION.to_string(),
            source: DodecadSystem::SOURCE.to_string(),
            term_characters: DodecadSystem::TERM_CHARACTERS.iter().map(|s| s.to_string()).collect(),
            connective_characters: DodecadSystem::CONNECTIVE_CHARACTERS
                .iter()
                .map(|(name, from, to)| Connector {
                    name: name.to_string(),
                    from_term: from.to_string(),
                    to_term: to.to_string(),
                })
                .collect(),
            nodes: DodecadSystem::NODES.to_vec(),
            edges: DodecadSystem::EDGES
                .iter()
                .map(|&(from, to)| Edge { from, to })
                .collect(),
            points: DodecadSystem::POINTS.iter().map(|&c| c.into()).collect(),
            lines: DodecadSystem::LINES
                .iter()
                .map(|&(start, end)| Line {
                    start: start.into(),
                    end: end.into(),
                })
                .collect(),
        }
    }
}

pub type SystematicsSchema = async_graphql::Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub fn create_schema() -> SystematicsSchema {
    async_graphql::Schema::build(QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription)
        .data(QueryRoot)
        .finish()
}
