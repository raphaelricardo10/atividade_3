use pyo3::prelude::*;
use pyo3::types::{PyList, PySet};
use std::env;

use crate::domain::graph::{Graph, Vertex};

pub fn solve_lp_graph_problem(graph: Graph) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<()> {
        let edges: Vec<(Vertex, Vertex)> = graph.edges.into_iter().collect();
        let edges_set = PySet::new(py, edges.as_slice())?;
        let args = (graph.num_vertex, graph.num_edges, edges_set);

        let sys_path: &PyList =
            pyo3::PyTryInto::try_into(py.import("sys").unwrap().getattr("path").unwrap()).unwrap();

        let path_buf = env::current_dir().unwrap();
        let current_dir = path_buf.to_str().unwrap();

        sys_path
            .insert(0, format!("{}/src/lp_solver/py_solver", current_dir))
            .unwrap();

        let solver = PyModule::import(py, "solver")?;
        solver.getattr("solve_lp_graph_problem")?.call1(args)?;
        print!("OK!");

        Ok(())
    })
}
