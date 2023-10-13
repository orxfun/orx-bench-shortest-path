use super::{dimacs9th::GraphDimacs9th, random::GraphRandom};
use crate::{experimentation::factors::factor::Factor, graph::sp_graph::SpGraph, utils::cli};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum FactorGraphData {
    Random(GraphRandom),
    Dimacs9th(GraphDimacs9th),
}

impl FactorGraphData {
    // graph ctor
    pub fn create_graph<G: SpGraph>(&self) -> G {
        match self {
            Self::Random(x) => x.create_graph(),
            Self::Dimacs9th(x) => x.create_graph(),
        }
    }
}

impl Factor for FactorGraphData {
    fn query_header_index() -> usize {
        2
    }
    fn name_factor() -> &'static str {
        "Graph Data"
    }
    fn definition_factor() -> &'static str {
        "Input graph of the problem."
    }

    fn query_levels_from_cli() -> Vec<Self> {
        PlainGraphData::level_from_cli()
            .iter()
            .flat_map(|data| match data {
                PlainGraphData::Random => GraphRandom::level_from_cli()
                    .into_iter()
                    .map(Self::Random)
                    .collect_vec(),
                PlainGraphData::Dimacs9th => GraphDimacs9th::level_from_cli()
                    .into_iter()
                    .map(Self::Dimacs9th)
                    .collect_vec(),
            })
            .collect()
    }

    fn to_cell_string(&self) -> String {
        match self {
            Self::Random(x) => x.to_cell_string(),
            Self::Dimacs9th(x) => x.to_cell_string(),
        }
    }
}
impl Default for FactorGraphData {
    fn default() -> Self {
        Self::Random(GraphRandom::default())
    }
}

// plain
#[derive(Clone, Debug, EnumIter)]
enum PlainGraphData {
    Random,
    Dimacs9th,
}
impl PlainGraphData {
    fn level_from_cli() -> Vec<Self> {
        let available_levels: Vec<_> = PlainGraphData::iter().collect();
        let definitions = &[
            "Randomly generated graph\n\
            with size and sparsity to be defined",
            "Graph from Dimacs-9th-Challenge\n\
            http://www.diag.uniroma1.it/~challenge9/download.shtml\n\
            Consists of USA road networks that are part of the challenge core instances.\n\
            These networks can be considered sparse having densities within 1e-7 and 1e-5.\n\
            (density = A / N^2; where density is 1 for a fully connected graph)",
        ];
        cli::print_table_get_choices(
            FactorGraphData::name_factor(),
            &available_levels,
            definitions,
            0,
        )
    }
}
