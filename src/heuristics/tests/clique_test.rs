use rstest::rstest;

use crate::heuristics::clique::Clique;

use super::fixtures::weighted_graph_test::{weighted_graph_factory, WeightedGraphFactory};

#[rstest]
fn test_generate_a_clique(weighted_graph_factory: WeightedGraphFactory) {
    let weighted_graph = weighted_graph_factory();
    let residual_graph = weighted_graph.get_residual(1).unwrap();

    let mut clique = Clique::new(residual_graph.graph.edges.clone());
    clique.vertices.insert(1);
    clique.vertices.insert(2);
    clique.vertices.insert(3);
    clique.vertices.insert(4);

    assert!(!clique.can_add_vertex(9));
}
