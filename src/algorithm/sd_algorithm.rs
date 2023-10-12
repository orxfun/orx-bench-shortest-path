use super::{measure::run_measure::Measure, solution::Solution};
use crate::graph::sp_graph::SpGraph;

pub trait ShortestDistanceAlgorithm<G: SpGraph> {
    fn new(graph: &G) -> Self;
    fn run_cached<M: Measure>(&mut self, graph: &G, source: usize, sink: usize) -> Solution<M>;
    fn run_pure<M: Measure>(graph: &G, source: usize, sink: usize) -> Solution<M>
    where
        Self: Sized,
    {
        Self::new(graph).run_cached(graph, source, sink)
    }
}
