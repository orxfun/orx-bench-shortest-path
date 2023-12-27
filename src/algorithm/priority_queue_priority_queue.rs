use super::{
    measure::run_measure::Measure, sd_algorithm::ShortestDistanceAlgorithm, solution::Solution,
};
use crate::{
    graph::{
        out_edges::{OutEdgeData, OutEdges},
        sp_graph::SpGraph,
    },
    Weight,
};
use priority_queue::PriorityQueue;

pub struct PriorityQueuePqDecKeyDijkstra {
    queue: PriorityQueue<usize, Weight>,
    visited: Vec<bool>,
}

impl PriorityQueuePqDecKeyDijkstra {
    fn new(queue: PriorityQueue<usize, u64>) -> Self {
        Self {
            queue,
            visited: Default::default(),
        }
    }
    fn run_cached_core<G: SpGraph, M: Measure>(
        &mut self,
        graph: &G,
        source: usize,
        sink: usize,
    ) -> Solution<M> {
        // reset
        let mut solution = Solution::new(graph, source, sink);
        self.visited
            .iter_mut()
            .take(graph.num_nodes())
            .for_each(|x| *x = false);
        if self.visited.len() < graph.num_nodes() {
            self.visited
                .extend((0..graph.num_nodes() - self.visited.len()).map(|_| false));
        }
        self.queue.clear();

        // init
        self.queue.push(source, Weight::default());

        // iterate
        while let Some((position, cost)) = self.queue.pop() {
            if position == sink {
                return solution.reached(cost);
            }
            // else if self.visited[position] {
            //     continue;
            // }

            let mut out_edges = graph.out_edges(position);
            while let Some(edge) = out_edges.next_edge() {
                let (head, weight) = (edge.head(), edge.weight());
                if !self.visited[head] {
                    self.queue.push_decrease(head, cost + weight);
                }
            }
            self.visited[position] = true;
            solution.iterate(self.queue.len(), self.queue.capacity());
        }

        solution.not_connected()
    }
}

// impl
impl<G> ShortestDistanceAlgorithm<G> for PriorityQueuePqDecKeyDijkstra
where
    G: SpGraph,
{
    fn new(graph: &G) -> Self {
        Self::new(PriorityQueue::with_capacity(graph.num_nodes()))
    }
    fn run_cached<M: Measure>(&mut self, graph: &G, source: usize, sink: usize) -> Solution<M> {
        self.run_cached_core(graph, source, sink)
    }
}
