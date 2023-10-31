use crate::{graph::sp_graph_builder::SpGraphBuilder, utils::cli, Weight};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct GraphRandom {
    pub seed: u64,
    pub num_nodes: usize,
    pub density: f32,
}

impl GraphRandom {
    pub fn level_from_cli() -> Vec<Self> {
        let seed = cli::print_scalar_query_get_answers("random seed", DEFAULT_SEED, |_| true);
        let num_nodes = cli::print_scalar_query_get_answers(
            "random graph number of nodes",
            DEFAULT_NUM_NODES,
            |n| n >= &4,
        );
        let density = cli::print_scalar_query_get_answers(
            "random graph density between 0 (no arcs) and 1 (fully connected)",
            DEFAULT_SPARSITY,
            |&n| (1e-5..=1.0).contains(&n),
        );

        let mut combinations = vec![];
        for &seed in &seed {
            for &num_nodes in &num_nodes {
                for &density in &density {
                    combinations.push(Self {
                        seed,
                        density,
                        num_nodes,
                    });
                }
            }
        }
        combinations
    }
    pub fn to_cell_string(self) -> String {
        format!(
            "Random:\n* seed      : {}\n* num_nodes : {}\n* density   : {:.4}",
            self.seed, self.num_nodes, self.density
        )
    }

    // graph ctor
    pub fn create_graph_builder<B: SpGraphBuilder>(&self) -> B {
        let (seed, num_nodes, density) = (self.seed, self.num_nodes, self.density);
        assert!((0.0..=1.0).contains(&density));

        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let nodes: Vec<_> = (0..num_nodes).collect();
        let out_degrees: Vec<_> = (0..num_nodes)
            .map(|_| {
                let r: f32 = rng.gen();
                let s = r * 2.0 * density;
                let out_degree = num_nodes as f32 * s;
                let out_degree = out_degree as usize;
                if out_degree == num_nodes {
                    out_degree - 1
                } else {
                    out_degree
                }
            })
            .collect();
        let num_edges: usize = out_degrees.iter().sum();

        let mut builder = B::new(Some(num_nodes), Some(num_edges));

        for (i, out_degree) in (0..num_nodes).zip(&out_degrees) {
            builder.add_node(i, Some(*out_degree));
        }
        for (i, out_degree) in (0..num_nodes).zip(out_degrees) {
            let heads = nodes
                .choose_multiple(&mut rng, out_degree + 1)
                .filter(|j| i != **j)
                .take(out_degree);
            for head in heads {
                let weight = rng.gen_range(1..2 * num_nodes) as Weight;
                builder.add_edge(i, *head, weight);
            }
        }
        builder
    }
}
impl Default for GraphRandom {
    fn default() -> Self {
        Self {
            seed: DEFAULT_SEED,
            num_nodes: DEFAULT_NUM_NODES,
            density: DEFAULT_SPARSITY,
        }
    }
}

// defaults
const DEFAULT_SEED: u64 = 9864;
const DEFAULT_SPARSITY: f32 = 0.25;
const DEFAULT_NUM_NODES: usize = 1000;
