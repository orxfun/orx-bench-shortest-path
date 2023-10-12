use super::measure::run_measure::Measure;
use crate::{graph::sp_graph::SpGraph, Weight};

pub struct Solution<M: Measure> {
    pub measure: M,
    pub shortest_distance: Option<Weight>,
}
impl<M: Measure> Solution<M> {
    pub fn new<G: SpGraph>(graph: &G, source: usize, sink: usize) -> Self {
        Self {
            measure: M::initialize(graph, source, sink),
            shortest_distance: None,
        }
    }
    pub fn reached(mut self, distance: Weight) -> Self {
        self.shortest_distance = Some(distance);
        self.measure.finalize(self.shortest_distance);
        self
    }
    pub fn not_connected(mut self) -> Self {
        self.measure.finalize(self.shortest_distance);
        self
    }
    pub fn iterate(&mut self, heap_length: usize, heap_capacity: usize) {
        self.measure.iterate(heap_length, heap_capacity)
    }
}
