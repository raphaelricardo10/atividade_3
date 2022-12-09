use std::collections::HashSet;

pub(crate) struct Graph {
    pub num_edges: u32,
    pub num_vertex: u32,
    pub edges: HashSet<(u32, u32)>,
}
