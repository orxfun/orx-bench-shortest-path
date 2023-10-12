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
use orx_priority_queue::{DaryHeap, PriorityQueue};

pub struct OrxPqDijkstra<Pq>
where
    Pq: PriorityQueue<usize, Weight>,
{
    queue: Pq,
    distances: Vec<Weight>,
}
impl<Pq> OrxPqDijkstra<Pq>
where
    Pq: PriorityQueue<usize, Weight>,
{
    fn new(queue: Pq) -> Self {
        Self {
            queue,
            distances: Default::default(),
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
        self.distances
            .iter_mut()
            .take(graph.num_nodes())
            .for_each(|x| *x = Weight::MAX);
        if self.distances.len() < graph.num_nodes() {
            self.distances
                .extend((0..graph.num_nodes() - self.distances.len()).map(|_| Weight::MAX));
        }
        self.queue.clear();

        // init
        self.distances[source] = 0;
        self.queue.push(source, Weight::default());

        // iterate
        while let Some((position, cost)) = self.queue.pop() {
            if position == sink {
                return solution.reached(cost);
            }

            if cost > self.distances[position] {
                continue;
            }

            let mut out_edges = graph.out_edges(position);
            while let Some(edge) = out_edges.next_edge() {
                let (head, weight) = (edge.head(), edge.weight());
                let next_cost = cost + weight;
                if next_cost < self.distances[head] {
                    self.queue.push(head, next_cost);
                    self.distances[head] = next_cost;
                }
            }

            solution.iterate(self.queue.len(), self.queue.capacity());
        }

        solution.not_connected()
    }
}

// impl
impl<G, const D: usize> ShortestDistanceAlgorithm<G> for OrxPqDijkstra<DaryHeap<usize, Weight, D>>
where
    G: SpGraph,
{
    fn new(_: &G) -> Self {
        Self::new(Default::default())
    }
    fn run_cached<M: Measure>(&mut self, graph: &G, source: usize, sink: usize) -> Solution<M> {
        self.run_cached_core(graph, source, sink)
    }
}
