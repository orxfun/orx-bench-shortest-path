use super::{
    out_edges_petgraph::OutEdgesPetgraph, sp_graph::SpGraph, sp_graph_builder::SpGraphBuilder,
};
use crate::Weight;
use petgraph::graph::NodeIndex;

pub type AdjListPetgraph = petgraph::graph::Graph<(), Weight, petgraph::Directed>;

impl SpGraphBuilder for AdjListPetgraph {
    type G = AdjListPetgraph;

    fn new(nodes_capacity: Option<usize>, edges_capacity: Option<usize>) -> Self {
        Self::with_capacity(nodes_capacity.unwrap_or(0), edges_capacity.unwrap_or(0))
    }
    fn add_node(&mut self, node: usize, _: Option<usize>) {
        let idx = self.add_node(());
        assert_eq!(node, idx.index());
    }
    fn add_edge(&mut self, tail: usize, head: usize, weight: crate::Weight) {
        self.add_edge(NodeIndex::new(tail), NodeIndex::new(head), weight);
    }
    fn build(self) -> Self::G {
        self
    }
}

impl SpGraph for AdjListPetgraph {
    type OutEdges<'a> = OutEdgesPetgraph<'a>;

    // type
    fn name() -> &'static str {
        "AdjListPetgraph"
    }

    // sp
    fn num_nodes(&self) -> usize {
        self.node_count()
    }
    fn out_edges(&self, node: usize) -> Self::OutEdges<'_> {
        self.edges(NodeIndex::new(node))
    }

    // build
    // fn new(nodes_capacity: Option<usize>, edges_capacity: Option<usize>) -> Self {
    //     Self::with_capacity(nodes_capacity.unwrap_or(0), edges_capacity.unwrap_or(0))
    // }
    // fn add_node(&mut self, node: usize, _: Option<usize>) {
    //     let idx = self.add_node(());
    //     assert_eq!(node, idx.index());
    // }
    // fn add_edge(&mut self, tail: usize, head: usize, weight: crate::Weight) {
    //     self.add_edge(NodeIndex::new(tail), NodeIndex::new(head), weight);
    // }
}
