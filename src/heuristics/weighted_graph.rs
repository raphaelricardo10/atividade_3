use std::collections::{HashMap};

use crate::domain::graph::{Graph, Vertex};

pub(crate) type Color = u32;
pub(crate) type Weight = f32;
pub(crate) type WeightsMap = HashMap<Vertex, Weight>;
pub(crate) type VerticesWeights = Vec<(Vertex, Weight)>;
pub(crate) type ColorWeights = HashMap<Color, VerticesWeights>;

pub(crate) struct ResidualGraph<'a> {
    pub(crate) graph: &'a Graph,
    pub(crate) weights: WeightsMap,
}

pub(crate) struct WeightedGraph {
    graph: Graph,
    color_weights: ColorWeights,
}

impl<'a> WeightedGraph {
    pub(crate) fn new(graph: Graph, color_weights: ColorWeights) -> Self {
        Self {
            graph,
            color_weights,
        }
    }

    pub(crate) fn get_residual(&'a self, color: Color) -> Option<ResidualGraph<'a>> {
        let weights: WeightsMap = self.color_weights.get(&color)?.iter().cloned().collect();

        Some(ResidualGraph {
            weights,
            graph: &self.graph,
        })
    }
}
