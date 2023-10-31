use super::{
    out_edges_std_vec::{OutEdge, OutEdgesStdVec},
    sp_graph::SpGraph,
    sp_graph_builder::SpGraphBuilder,
};
use crate::Weight;

pub type AdjListJaggedVec = Vec<Vec<OutEdge>>;

impl SpGraphBuilder for AdjListJaggedVec {
    type G = AdjListJaggedVec;

    fn new(nodes_capacity: Option<usize>, _edges_capacity: Option<usize>) -> Self {
        match nodes_capacity {
            Some(c) => Self::with_capacity(c),
            None => Self::new(),
        }
    }
    fn add_node(&mut self, node: usize, out_degree_capacity: Option<usize>) {
        assert_eq!(node, self.len());
        self.push(match out_degree_capacity {
            Some(c) => Vec::with_capacity(c),
            None => Vec::new(),
        });
    }
    fn add_edge(&mut self, tail: usize, head: usize, weight: Weight) {
        self[tail].push(OutEdge::new(head, weight));
    }
    fn build(self) -> Self::G {
        self
    }
}

impl SpGraph for AdjListJaggedVec {
    type OutEdges<'a> = OutEdgesStdVec<'a>;

    // type
    fn name() -> &'static str {
        "AdjListJaggedVec"
    }

    // sp
    fn num_nodes(&self) -> usize {
        self.len()
    }
    fn out_edges(&self, node: usize) -> Self::OutEdges<'_> {
        self[node].iter()
    }
}
