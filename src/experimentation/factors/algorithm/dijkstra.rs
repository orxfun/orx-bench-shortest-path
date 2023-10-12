use super::{priority_queue::PriorityQueue, priority_queue_deckey::PriorityQueueDecKey};
use crate::utils::cli;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug)]
pub enum Dijkstra {
    PriorityQueue(PriorityQueue),
    PriorityQueueDecKey(PriorityQueueDecKey),
    Petgraph,
}

impl Dijkstra {
    pub fn level_from_cli() -> Vec<Self> {
        PlainDijkstra::level_from_cli()
            .into_iter()
            .flat_map(|dijkstra| match dijkstra {
                PlainDijkstra::PriorityQueue => PriorityQueue::level_from_cli()
                    .into_iter()
                    .map(Dijkstra::PriorityQueue)
                    .collect_vec(),
                PlainDijkstra::PriorityQueueDecKey => PriorityQueueDecKey::level_from_cli()
                    .into_iter()
                    .map(Dijkstra::PriorityQueueDecKey)
                    .collect_vec(),
                PlainDijkstra::Petgraph => vec![Dijkstra::Petgraph],
            })
            .collect()
    }
    pub fn to_cell_string(self) -> String {
        match self {
            Self::PriorityQueue(x) => x.to_cell_string("Dijkstra"),
            Self::PriorityQueueDecKey(x) => x.to_cell_string("Dijkstra"),
            Self::Petgraph => "Dijkstra\n* impl: petgraph::algo::dijkstra".to_string(),
        }
    }
}

// plain
#[derive(Clone, Debug, EnumIter)]
enum PlainDijkstra {
    PriorityQueue,
    PriorityQueueDecKey,
    Petgraph,
}
impl PlainDijkstra {
    fn level_from_cli() -> Vec<Self> {
        let available_levels: Vec<_> = PlainDijkstra::iter().collect();
        let definitions = &[
            "impl using a basic priority queue",
            "impl using a priority queue with decrease key",
            "petgraph::algo::dijkstra",
        ];
        cli::print_subheader(1, "Dijsktra Implementations");
        cli::print_table_get_choices("Dijsktra", &available_levels, definitions, 0)
    }
}
