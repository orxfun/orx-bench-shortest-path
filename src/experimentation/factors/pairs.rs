use super::factor::Factor;
use crate::utils::cli;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Clone, Copy, Debug)]
pub struct FactorPairs {
    pub seed: u64,
    pub num_pairs: usize,
}

impl FactorPairs {
    pub fn create_pairs(&self, num_nodes: usize) -> Vec<(usize, usize)> {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);
        (0..self.num_pairs)
            .map(|_| {
                let s = rng.gen_range(0..num_nodes);
                let t = rng.gen_range(0..num_nodes);
                (s, t)
            })
            .collect()
    }
}

impl Factor for FactorPairs {
    fn query_header_index() -> usize {
        6
    }
    fn name_factor() -> &'static str {
        "Source-Sink Pairs"
    }
    fn definition_factor() -> &'static str {
        "Source-sink pairs shortest distances of which \
        will be computed in the experiment."
    }

    fn query_levels_from_cli() -> Vec<Self> {
        let seed = cli::print_scalar_query_get_answers("random seed", DEFAULT_SEED, |_| true);
        let num_pairs =
            cli::print_scalar_query_get_answers("number of (s,t) pairs", DEFAULT_NUM_NODES, |n| {
                n >= &1
            });
        let mut combinations = vec![];
        for &seed in &seed {
            for &num_pairs in &num_pairs {
                combinations.push(Self { seed, num_pairs })
            }
        }
        combinations
    }

    fn to_cell_string(&self) -> String {
        format!(
            "* seed      : {}\n* num_pairs : {}",
            self.seed, self.num_pairs
        )
    }
}
impl Default for FactorPairs {
    fn default() -> Self {
        Self {
            seed: DEFAULT_SEED,
            num_pairs: DEFAULT_NUM_NODES,
        }
    }
}

// defaults
const DEFAULT_SEED: u64 = 465477;
const DEFAULT_NUM_NODES: usize = 100;
