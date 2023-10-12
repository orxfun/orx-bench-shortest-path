use super::{
    measure::run_measure::Measure, sd_algorithm::ShortestDistanceAlgorithm, solution::Solution,
};
use crate::graph::adjlist_petgraph::AdjListPetgraph;
use petgraph::graph::NodeIndex;

pub struct PetgraphDijsktra;

impl ShortestDistanceAlgorithm<AdjListPetgraph> for PetgraphDijsktra {
    fn new(_: &AdjListPetgraph) -> Self {
        Self
    }
    fn run_cached<M: Measure>(
        &mut self,
        graph: &AdjListPetgraph,
        source: usize,
        sink: usize,
    ) -> Solution<M> {
        let solution = Solution::new(graph, source, sink);

        let sink = NodeIndex::new(sink);
        let result =
            petgraph::algo::dijkstra(&graph, NodeIndex::new(source), Some(sink), |e| *e.weight());
        let distance = result.get(&sink).cloned();

        match distance {
            Some(d) => solution.reached(d),
            None => solution.not_connected(),
        }
    }
}
