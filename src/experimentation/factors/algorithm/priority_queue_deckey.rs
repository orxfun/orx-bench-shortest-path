use super::dary::Dary;
use crate::utils::cli;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PriorityQueueDecKey {
    OrxDaryHeapOfIndices(Dary),
    OrxDaryHeapWithMap(Dary),
    PriorityQueuePriorityQueue,
}
impl PriorityQueueDecKey {
    pub fn level_from_cli() -> Vec<Self> {
        let plain = PlainPriorityQueueDecKey::level_from_cli();
        let dary = Dary::level_from_cli();

        plain
            .into_iter()
            .flat_map(|queue| {
                dary.iter().map(move |&dary| match queue {
                    PlainPriorityQueueDecKey::OrxDaryHeapOfIndices => {
                        Self::OrxDaryHeapOfIndices(dary)
                    }
                    PlainPriorityQueueDecKey::OrxDaryHeapWithMap => Self::OrxDaryHeapWithMap(dary),
                    PlainPriorityQueueDecKey::PriorityQueuePriorityQueue => {
                        Self::PriorityQueuePriorityQueue
                    }
                })
            })
            .collect()
    }
    pub fn to_cell_string(self, algorithm_name: &str) -> String {
        match self {
            Self::OrxDaryHeapOfIndices(x) => {
                format!(
                    "{}\n* orx_priority_queue::{:?}HeapOfIndices (d={})",
                    algorithm_name,
                    x,
                    x.d()
                )
            }
            Self::OrxDaryHeapWithMap(x) => {
                format!(
                    "{}\n* orx_priority_queue::{:?}HeapWithMap (d={})",
                    algorithm_name,
                    x,
                    x.d()
                )
            }
            Self::PriorityQueuePriorityQueue => {
                format!("{}\n* priority_queue::PriorityQueue", algorithm_name)
            }
        }
    }
}

// helpers
#[derive(Clone, Debug, EnumIter)]
enum PlainPriorityQueueDecKey {
    OrxDaryHeapOfIndices,
    OrxDaryHeapWithMap,
    PriorityQueuePriorityQueue,
}
impl PlainPriorityQueueDecKey {
    fn level_from_cli() -> Vec<Self> {
        let available_levels: Vec<_> = PlainPriorityQueueDecKey::iter().collect();
        let definitions = &[
            "Basic priority queue with orx_priority_queue::DaryHeapOfIndices\n\
            * heap paired up with a positions array.",
            "Basic priority queue with orx_priority_queue::BinaryHeapWithMap\n\
            * heap paired up with a hash map.",
            "Priority queue with priority_queue::PriorityQueue\n\
            * heap paired up with a index map.",
        ];

        cli::print_subheader(2, "DecreaseKey Priority Queues");
        cli::print_table_get_choices("PriorityQueueDecKey", &available_levels, definitions, 0)
    }
}
