use std::collections::{HashMap, HashSet};

use crate::domain::graph::{EdgeSet, Vertex};

type EdgeMap = HashMap<Vertex, HashSet<Vertex>>;

pub(crate) struct Clique {
    pub(crate) edges: EdgeMap,
    pub(crate) vertices: HashSet<Vertex>,
}

impl Clique {
    pub fn new(edges: EdgeSet) -> Self {
        Self {
            vertices: Default::default(),
            edges: Self::map_edges(edges),
        }
    }

    fn add_edge(edge_map: &mut EdgeMap, src: Vertex, dest: Vertex) {
        edge_map.entry(src).or_default().insert(dest);
    }

    fn map_edges(edges: EdgeSet) -> EdgeMap {
        let mut edge_map = EdgeMap::new();

        for (src, dest) in edges {
            Self::add_edge(&mut edge_map, src, dest);
            Self::add_edge(&mut edge_map, dest, src);
        }

        edge_map
    }

    pub fn can_add_vertex(&self, new_vertex: Vertex) -> bool {
        self.vertices
            .iter()
            .map(|vertex| self.edges.get(vertex).unwrap())
            .all(|neighbors| neighbors.contains(&new_vertex))
    }
}
