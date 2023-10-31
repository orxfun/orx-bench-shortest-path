use super::sp_graph::SpGraph;
use crate::Weight;

pub trait SpGraphBuilder {
    type G: SpGraph;

    fn new(nodes_capacity: Option<usize>, edges_capacity: Option<usize>) -> Self;
    fn add_node(&mut self, node: usize, out_degree_capacity: Option<usize>);
    fn add_edge(&mut self, tail: usize, head: usize, weight: Weight);

    fn build(self) -> Self::G;
}
