use super::dary::Dary;
use crate::utils::cli;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PriorityQueue {
    OrxDaryHeap(Dary),
    StdBinaryHeap,
}
impl PriorityQueue {
    pub fn level_from_cli() -> Vec<Self> {
        PlainPriorityQueue::level_from_cli()
            .into_iter()
            .flat_map(|queue| match queue {
                PlainPriorityQueue::OrxDaryHeap => Dary::level_from_cli()
                    .into_iter()
                    .map(Self::OrxDaryHeap)
                    .collect_vec(),
                PlainPriorityQueue::StdBinaryHeap => vec![Self::StdBinaryHeap],
            })
            .collect()
    }
    pub fn to_cell_string(self, algorithm_name: &str) -> String {
        match self {
            Self::OrxDaryHeap(x) => {
                format!(
                    "{}\n* orx_priority_queue::{:?}Heap (d={})",
                    algorithm_name,
                    x,
                    x.d()
                )
            }
            Self::StdBinaryHeap => {
                format!("{}\n* std::collections::BinaryHeap", algorithm_name)
            }
        }
    }
}

// helpers
#[derive(Clone, Debug, EnumIter)]
enum PlainPriorityQueue {
    StdBinaryHeap,
    OrxDaryHeap,
}
impl PlainPriorityQueue {
    fn level_from_cli() -> Vec<Self> {
        let available_levels: Vec<_> = PlainPriorityQueue::iter().collect();
        let definitions = &[
            "Basic priority queue with orx_priority_queue::DaryHeap",
            "Basic priority queue with std::collections::BinaryHeap",
        ];

        cli::print_subheader(2, "Basic Priority Queues");
        cli::print_table_get_choices("PriorityQueue", &available_levels, definitions, 0)
    }
}
