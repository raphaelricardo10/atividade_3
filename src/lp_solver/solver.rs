use pyo3::prelude::*;
use pyo3::types::PySet;

use crate::domain::graph::{Graph, Vertex};

pub fn solve_lp_graph_problem(graph: Graph) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<()> {
        let edges: Vec<(Vertex, Vertex)> = graph.edges.into_iter().collect();
        let edges_set = PySet::new(py, edges.as_slice())?;
        let args = (graph.num_vertex, graph.num_edges, edges_set);
        let code = include_str!("./py_solver/solver.py");

        let solver = PyModule::from_code(py, code, "", "")?;
        solver.getattr("solve_lp_graph_problem")?.call1(args)?;
        print!("OK!");

        Ok(())
    })
}
