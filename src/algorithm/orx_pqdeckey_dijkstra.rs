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
use orx_priority_queue::{DaryHeapOfIndices, DaryHeapWithMap, PriorityQueueDecKey};

pub struct OrxPqDecKeyDijkstra<Pq>
where
    Pq: PriorityQueueDecKey<usize, Weight>,
{
    queue: Pq,
    visited: Vec<bool>,
}
impl<Pq> OrxPqDecKeyDijkstra<Pq>
where
    Pq: PriorityQueueDecKey<usize, Weight>,
{
    fn new(queue: Pq) -> Self {
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
            } else if self.visited[position] {
                continue;
            }

            let mut out_edges = graph.out_edges(position);
            while let Some(edge) = out_edges.next_edge() {
                let (head, weight) = (edge.head(), edge.weight());
                if !self.visited[head] {
                    self.queue.try_decrease_key_or_push(&head, cost + weight);
                }
            }
            self.visited[position] = true;
            solution.iterate(self.queue.len(), self.queue.capacity());
        }

        solution.not_connected()
    }
}

// impl
impl<G, const D: usize> ShortestDistanceAlgorithm<G>
    for OrxPqDecKeyDijkstra<DaryHeapOfIndices<usize, Weight, D>>
where
    G: SpGraph,
{
    fn new(graph: &G) -> Self {
        Self::new(DaryHeapOfIndices::with_index_bound(graph.num_nodes()))
    }
    fn run_cached<M: Measure>(&mut self, graph: &G, source: usize, sink: usize) -> Solution<M> {
        self.run_cached_core(graph, source, sink)
    }
}
impl<G, const D: usize> ShortestDistanceAlgorithm<G>
    for OrxPqDecKeyDijkstra<DaryHeapWithMap<usize, Weight, D>>
where
    G: SpGraph,
{
    fn new(_: &G) -> Self {
        Self::new(DaryHeapWithMap::default())
    }
    fn run_cached<M: Measure>(&mut self, graph: &G, source: usize, sink: usize) -> Solution<M> {
        self.run_cached_core(graph, source, sink)
    }
}
