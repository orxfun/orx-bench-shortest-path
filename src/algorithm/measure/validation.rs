use super::{no_measurement::NoMeasurement, run_measure::Measure};
use crate::{
    algorithm::{sd_algorithm::ShortestDistanceAlgorithm, std_pq_dijkstra::StdPqDijkstra},
    graph::sp_graph::SpGraph,
    Weight,
};

#[derive(Debug)]
pub struct Validation {
    pub source: usize,
    pub sink: usize,
    pub correct_shortest_distance: Option<Weight>,
    pub found_shortest_distance: Option<Weight>,
}
impl Validation {
    pub fn is_correct(&self) -> bool {
        self.correct_shortest_distance == self.found_shortest_distance
    }
}

impl Measure for Validation {
    fn initialize<G: SpGraph>(graph: &G, source: usize, sink: usize) -> Self {
        Self {
            source,
            sink,
            correct_shortest_distance: get_valid_solution(graph, source, sink),
            found_shortest_distance: Some(Weight::MAX),
        }
    }
    #[inline(always)]
    fn iterate(&mut self, _: usize, _: usize) {}
    fn finalize(&mut self, shortest_distance: Option<Weight>) {
        self.found_shortest_distance = shortest_distance;
    }
}

fn get_valid_solution<G: SpGraph>(graph: &G, source: usize, sink: usize) -> Option<Weight> {
    StdPqDijkstra::run_pure::<NoMeasurement>(graph, source, sink).shortest_distance
}
