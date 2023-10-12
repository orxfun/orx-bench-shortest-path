use crate::utils::cli;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::dary::Dary;

#[derive(Clone, Copy, Debug)]
pub enum PriorityQueueDecKey {
    OrxDaryHeapOfIndices(Dary),
    OrxDaryHeapWithMap(Dary),
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
        }
    }
}

// helpers
#[derive(Clone, Debug, EnumIter)]
enum PlainPriorityQueueDecKey {
    OrxDaryHeapOfIndices,
    OrxDaryHeapWithMap,
}
impl PlainPriorityQueueDecKey {
    fn level_from_cli() -> Vec<Self> {
        let available_levels: Vec<_> = PlainPriorityQueueDecKey::iter().collect();
        let definitions = &[
            "Basic priority queue with orx_priority_queue::DaryHeapOfIndices\n\
            * heap paired up with a positions array.",
            "Basic priority queue with std::collections::BinaryHeapWithMap\n\
            * heap paired up with a hash map.",
        ];

        cli::print_subheader(2, "DecreaseKey Priority Queues");
        cli::print_table_get_choices("PriorityQueueDecKey", &available_levels, definitions, 0)
    }
}
