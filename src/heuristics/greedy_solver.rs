use std::collections::{HashMap, HashSet};

use super::{
    clique::Clique,
    solution::Solution,
    weighted_graph::{Color, Weight, WeightedGraph},
};
use crate::domain::graph::Vertex;

pub struct GreedySolver {
    graph: WeightedGraph,
}

impl GreedySolver {
    pub(crate) fn new(graph: WeightedGraph) -> Self {
        Self { graph }
    }

    pub fn solve(&self) {
        let mut solutions: HashMap<Color, Solution> = HashMap::new();

        for color in self.graph.colors() {
            let sol = self.solve_for_color(color).unwrap();

            solutions.insert(color, sol);
        }
    }

    pub fn solve_for_color(&self, color: Color) -> Option<Solution> {
        let mut residual_graph = self.graph.get_residual(color)?;

        let mut clique = Clique::new(residual_graph.graph.edges.clone());

        let mut sorted_weights: Vec<(Vertex, Weight)> = residual_graph
            .weights
            .iter()
            .map(|(vertex, weight)| (*vertex, *weight))
            .collect();

        sorted_weights.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap());

        loop {
            let candidate = sorted_weights
                .iter()
                .filter(|(vertex, _)| residual_graph.weights.contains_key(vertex))
                .find(|(vertex, _)| clique.can_add_vertex(*vertex))
                .map(|(vertex, _)| vertex);

            if candidate.is_none() {
                break;
            }

            residual_graph.weights.remove(candidate.unwrap());
            clique.vertices.insert(*candidate.unwrap());
        }

        let clique_weights = residual_graph
            .weights
            .into_iter()
            .filter(|(vertex, _)| !clique.vertices.contains(vertex))
            .collect();

        Some(Solution::new(color, clique_weights))
    }
}
