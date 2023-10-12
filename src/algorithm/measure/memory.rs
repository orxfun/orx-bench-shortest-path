use super::{run_measure::Measure, time::Time};
use crate::{graph::sp_graph::SpGraph, Weight};

pub struct Memory {
    pub time: Time,
    pub num_iterations: usize,
    pub max_num_items: usize,
    pub total_num_items_in_observations: usize,
    pub max_capacity: usize,
    pub total_capacity_in_observations: usize,
}
impl Measure for Memory {
    fn initialize<G: SpGraph>(_: &G, _: usize, _: usize) -> Self {
        Self {
            time: Time::start(),
            num_iterations: 0,
            max_num_items: 0,
            total_num_items_in_observations: 0,
            max_capacity: 0,
            total_capacity_in_observations: 0,
        }
    }
    fn iterate(&mut self, heap_length: usize, heap_capacity: usize) {
        self.time.iterate(heap_length, heap_capacity);
        self.num_iterations += 1;

        self.max_num_items = self.max_num_items.max(heap_length);
        self.total_num_items_in_observations += heap_length;

        self.max_capacity = self.max_capacity.max(heap_capacity);
        self.total_capacity_in_observations += heap_capacity;
    }
    fn finalize(&mut self, _: Option<Weight>) {
        self.time.stop()
    }
}
