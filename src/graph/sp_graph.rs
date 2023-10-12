use super::out_edges::OutEdges;
use crate::{utils::log_debug::LogDebug, Weight};

pub trait SpGraph: LogDebug {
    type OutEdges<'a>: OutEdges where where Self:'a;

    // type
    fn name() -> &'static str;

    // sp
    fn num_nodes(&self) -> usize;
    fn out_edges(&self, node: usize) -> Self::OutEdges<'_>;

    // build
    fn new(nodes_capacity: Option<usize>, edges_capacity: Option<usize>) -> Self;
    fn add_node(&mut self, node: usize, out_degree_capacity: Option<usize>);
    fn add_edge(&mut self, tail: usize, head: usize, weight: Weight);

    // default impl
    fn num_edges(&self) -> usize {
        (0..self.num_nodes())
            .map(|i| self.out_edges(i).count_edges())
            .sum()
    }
}
