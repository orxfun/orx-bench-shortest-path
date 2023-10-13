use super::{dary::Dary, dijkstra::Dijkstra, priority_queue::PriorityQueue};
use crate::{experimentation::factors::factor::Factor, utils::cli};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum FactorAlgorithm {
    Dijkstra(Dijkstra),
}
impl Factor for FactorAlgorithm {
    fn query_header_index() -> usize {
        4
    }
    fn name_factor() -> &'static str {
        "Algorithm"
    }
    fn definition_factor() -> &'static str {
        "Single (s,t) shortest path algorithm."
    }

    fn query_levels_from_cli() -> Vec<Self> {
        PlainAlgorithm::level_from_cli()
            .into_iter()
            .flat_map(|algorithm| match algorithm {
                PlainAlgorithm::Dijkstra => Dijkstra::level_from_cli()
                    .into_iter()
                    .map(Self::Dijkstra)
                    .collect_vec(),
            })
            .collect()
    }
    fn to_cell_string(&self) -> String {
        match self {
            Self::Dijkstra(d) => d.to_cell_string(),
        }
    }
}
impl Default for FactorAlgorithm {
    fn default() -> Self {
        Self::Dijkstra(Dijkstra::PriorityQueue(PriorityQueue::OrxDaryHeap(
            Dary::Binary,
        )))
    }
}

// plain
#[derive(Clone, Debug, EnumIter)]
enum PlainAlgorithm {
    Dijkstra,
}
impl PlainAlgorithm {
    fn level_from_cli() -> Vec<Self> {
        let available_levels: Vec<_> = PlainAlgorithm::iter().collect();
        let definitions = &["Dijsktra's shortest path algorithm"];
        cli::print_table_get_choices(
            FactorAlgorithm::name_factor(),
            &available_levels,
            definitions,
            0,
        )
    }
}
