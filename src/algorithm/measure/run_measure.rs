use crate::{graph::sp_graph::SpGraph, Weight};

pub trait Measure {
    fn initialize<G: SpGraph>(graph: &G, source: usize, sink: usize) -> Self;
    fn iterate(&mut self, heap_length: usize, heap_capacity: usize);
    fn finalize(&mut self, shortest_distance: Option<Weight>);
}
