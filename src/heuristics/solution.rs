use super::weighted_graph::{VerticesWeights, Color, Weight};

pub struct Solution {
    pub color: Color,
    pub total_weight: Weight,
    pub weights: VerticesWeights,
}

impl Solution {
    pub fn new(color: Color, weights: VerticesWeights) -> Self {
        let total_weight = Self::calculate_total_weight(&weights);

        Self {
            color,
            weights,
            total_weight,
        }
    }

    fn calculate_total_weight(weights: &VerticesWeights) -> Weight {
        weights.values().sum()
    }
}
