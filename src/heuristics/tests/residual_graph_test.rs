use rstest::rstest;

use super::fixtures::weighted_graph::{weighted_graph_factory, WeightedGraphFactory};

#[rstest]
fn test_can_get_a_residual_graph(weighted_graph_factory: WeightedGraphFactory) {
    let weighted_graph = weighted_graph_factory();
    let residual_graph = weighted_graph.get_residual(0).unwrap();
    
    assert_eq!(residual_graph.weights.len(), 16);
}
