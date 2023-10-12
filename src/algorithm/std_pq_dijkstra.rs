use super::{
    measure::run_measure::Measure, sd_algorithm::ShortestDistanceAlgorithm, solution::Solution,
};
use crate::graph::{
    out_edges::{OutEdgeData, OutEdges},
    sp_graph::SpGraph,
};
use crate::Weight;
use std::cmp::Ordering;

#[derive(Default)]
pub struct StdPqDijkstra {
    heap: std::collections::BinaryHeap<State>,
    distances: Vec<Weight>,
}

impl<G: SpGraph> ShortestDistanceAlgorithm<G> for StdPqDijkstra {
    fn new(_: &G) -> Self {
        Self::default()
    }
    fn run_cached<M: Measure>(&mut self, graph: &G, source: usize, sink: usize) -> Solution<M> {
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
        self.heap.clear();

        // init
        self.distances[source] = 0;
        self.heap.push(State {
            cost: Weight::default(),
            position: source,
        });

        // iterate
        while let Some(State { cost, position }) = self.heap.pop() {
            if position == sink {
                return solution.reached(cost);
            }

            if cost > self.distances[position] {
                continue;
            }

            let mut out_edges = graph.out_edges(position);
            while let Some(edge) = out_edges.next_edge() {
                let (head, weight) = (edge.head(), edge.weight());
                let next = State {
                    cost: cost + weight,
                    position: head,
                };

                if next.cost < self.distances[next.position] {
                    self.heap.push(next);
                    self.distances[next.position] = next.cost;
                }
            }

            solution.iterate(self.heap.len(), self.heap.capacity());
        }

        solution.not_connected()
    }
}

/// State required for the std::collections::BinaryHeap.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub cost: Weight,
    pub position: usize,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
