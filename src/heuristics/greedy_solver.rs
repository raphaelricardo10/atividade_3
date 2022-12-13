use std::collections::HashMap;

use super::weighted_graph::{Color, Weight, WeightedGraph};
use crate::domain::graph::Vertex;

pub struct GreedySolver {
    graph: WeightedGraph,
}

impl GreedySolver {
    pub(crate) fn new(graph: WeightedGraph) -> Self {
        Self { graph }
    }

    pub fn solve(&self) {
        todo!()
    }

    pub fn solve_for_color(&self, color: Color) -> Option<()> {
        let residual_graph = self.graph.get_residual(color)?;

        let starting_point = residual_graph
            .weights
            .iter()
            .max_by(|(_, w1), (_, w2)| w1.partial_cmp(w2).unwrap());

        let mut weights: Vec<(Vertex, Weight)> = residual_graph
            .weights
            .iter()
            // .filter(|(vertex, _)| residual_graph.graph.edges.contains((starting_point.0, )))
            .map(|(vertex, weight)| (*vertex, *weight))
            .collect();

        weights.sort_by(|(_, w1), (_, w2)| w1.partial_cmp(w2).unwrap());

        let a = residual_graph;

        Some(())
    }
}
