use std::env;

use graph_col_solver::file_reader::graph_file_reader::GraphFileReader;
use graph_col_solver::lp_solver::solver;

fn main() {
    let filename = env::args().nth(1).expect("No filename was provided!");
    println!("Filename: {}", filename);

    let graph = GraphFileReader::new(filename).read_file().unwrap();

    solver::solve_lp_graph_problem(graph).unwrap();
}
