use std::{collections::HashMap, ops::Range};

use crate::domain::graph::{Graph, Vertex};

pub type Color = u32;
pub type Weight = f32;
pub type Solution = HashMap<(Vertex, Color), Weight>;

pub(crate) type VerticesWeights = HashMap<Vertex, Weight>;
pub(crate) type ColorWeights = HashMap<Color, VerticesWeights>;

pub(crate) struct ResidualGraph<'a> {
    pub(crate) graph: &'a Graph,
    pub(crate) weights: VerticesWeights,
}

pub(crate) struct WeightedGraph {
    graph: Graph,
    color_weights: ColorWeights,
}

impl<'a> WeightedGraph {
    pub fn new(graph: Graph, color_weights: ColorWeights) -> Self {
        Self {
            graph,
            color_weights,
        }
    }

    pub fn from_solution(graph: Graph, solution: Solution) -> Self {
        let mut color_weights = ColorWeights::new();

        for ((vertex, color), weight) in solution {
            color_weights
                .entry(color)
                .or_insert_with(VerticesWeights::new);
            color_weights
                .get_mut(&color)
                .unwrap()
                .insert(vertex, weight);
        }

        Self {
            graph,
            color_weights,
        }
    }

    pub(crate) fn get_residual(&'a self, color: Color) -> Option<ResidualGraph<'a>> {
        let weights: VerticesWeights = self.color_weights.get(&color)?.clone();

        Some(ResidualGraph {
            weights,
            graph: &self.graph,
        })
    }

    pub(crate) fn colors(&self) -> Range<Color> {
        return 1..self.graph.num_edges;
    }
}
