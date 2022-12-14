use rstest::rstest;

use crate::heuristics::greedy_solver::GreedySolver;

use super::fixtures::weighted_graph_test::{weighted_graph_factory, WeightedGraphFactory};

#[rstest]
fn test_can_generate_a_greedy_solution_for_color(weighted_graph_factory: WeightedGraphFactory) {
    let weighted_graph = weighted_graph_factory();

    let greedy_solver = GreedySolver::new(weighted_graph);

    let solution = greedy_solver.solve_for_color(1).unwrap();

    assert_ne!(solution.weights.len(), 0);
    assert_ne!(solution.total_weight, 0.0);
}
