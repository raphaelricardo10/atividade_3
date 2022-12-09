use std::collections::HashSet;

type Edge = (u32, u32);
type EdgeSet = HashSet<Edge>;

pub(crate) struct Graph {
    pub num_edges: u32,
    pub num_vertex: u32,
    pub edges: EdgeSet,
}

impl Graph {
    pub(crate) fn new(num_edges: u32, num_vertex: u32, edges: EdgeSet) -> Self {
        Self {
            edges,
            num_edges,
            num_vertex,
        }
    }
}
