use rstest::fixture;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    domain::graph::Graph,
    file_reader::graph_file_reader::GraphFileReader,
    heuristics::weighted_graph::{ColorWeights, WeightedGraph, VerticesWeights},
};

pub(crate) type WeightedGraphFactory = Box<dyn FnOnce() -> WeightedGraph>;

#[fixture]
pub(crate) fn graph() -> Graph {
    let filename = format!("{}/TPI_BC_COL_0.txt", env!("INSTANCES_PATH"));
    GraphFileReader::new(filename)
        .read_file()
        .unwrap()
}

#[fixture]
pub(crate) fn weighted_graph_factory(graph: Graph) -> WeightedGraphFactory {
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    let wrapper = move || -> WeightedGraph {
        let mut color_weights = ColorWeights::new();

        for color in 0..graph.num_vertex {
            let mut acc: f32 = 0.0;
            let mut weights_by_color = VerticesWeights::new();

            for vertex in 0..graph.num_vertex {
                if acc == 1.0 {
                    break
                }

                let weight = rng.gen_range(0.0..(1.0 - acc));
                acc += weight;

                weights_by_color.insert(vertex, weight);
            }

            color_weights.insert(color, weights_by_color);
        }

        WeightedGraph::new(graph, color_weights)
    };

    Box::new(wrapper)
}
