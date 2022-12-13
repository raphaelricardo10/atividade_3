use crate::{
    domain::graph::{EdgeSet, Graph, Vertex},
    heuristics::weighted_graph::{Color, Solution, Weight, WeightedGraph},
};

#[repr(C)]
pub struct PartialSolutionEntry {
    vertex: Vertex,
    color: Color,
    weight: Weight,
}

#[repr(C)]
pub struct ABIEdge {
    src: Vertex,
    dest: Vertex,
}

#[repr(C)]
pub struct ABIGraph {
    num_vertex: u32,
    num_edges: usize,
    edges: *mut ABIEdge,
}

#[no_mangle]
pub extern "C" fn test_greedy_solver_callback(
    _: ABIGraph,
    _: *mut PartialSolutionEntry,
    _: usize,
) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn greedy_solver_callback(
    graph_ptr: ABIGraph,
    partial_solution_ptr: *mut PartialSolutionEntry,
    num_entries: usize,
) {
    let graph = unsafe {
        let edges: EdgeSet = std::slice::from_raw_parts(graph_ptr.edges, graph_ptr.num_edges)
            .iter()
            .map(|edge| (edge.src, edge.dest))
            .collect();

        Graph::new(
            graph_ptr.num_edges.try_into().unwrap(),
            graph_ptr.num_vertex,
            edges,
        )
    };

    let partial_solution: Solution = unsafe {
        std::slice::from_raw_parts(partial_solution_ptr, num_entries)
            .iter()
            .map(|entry| ((entry.vertex, entry.color), entry.weight))
            .collect()
    };

    let weighted_graph = WeightedGraph::from_solution(graph, partial_solution);
}
