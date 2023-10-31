use super::{
    adjlist_jagged_vec::AdjListJaggedVec,
    out_edges_std_vec::{OutEdge, OutEdgesStdVec},
    sp_graph::SpGraph,
    sp_graph_builder::SpGraphBuilder,
};
use crate::Weight;

pub struct AdjListFlatVec {
    edges: Vec<OutEdge>,
    out_edges_begin_index: Vec<usize>,
}

impl SpGraph for AdjListFlatVec {
    type OutEdges<'a> = OutEdgesStdVec<'a>;
    fn name() -> &'static str {
        "AdjListFlatVec"
    }
    fn num_nodes(&self) -> usize {
        self.out_edges_begin_index.len() - 1
    }
    fn out_edges(&self, node: usize) -> Self::OutEdges<'_> {
        let begin = self.out_edges_begin_index[node];
        let end = self.out_edges_begin_index[node + 1];
        self.edges[begin..end].iter()
    }
}

pub struct AdjListFlatVecBuilder(AdjListJaggedVec);
impl SpGraphBuilder for AdjListFlatVecBuilder {
    type G = AdjListFlatVec;

    fn new(nodes_capacity: Option<usize>, edges_capacity: Option<usize>) -> Self {
        Self(<AdjListJaggedVec as SpGraphBuilder>::new(
            nodes_capacity,
            edges_capacity,
        ))
    }
    fn add_node(&mut self, node: usize, out_degree_capacity: Option<usize>) {
        self.0.add_node(node, out_degree_capacity)
    }
    fn add_edge(&mut self, tail: usize, head: usize, weight: Weight) {
        self.0.add_edge(tail, head, weight)
    }
    fn build(self) -> Self::G {
        let mut cumulative = 0;
        let mut out_edges_begin_index = vec![cumulative];
        for out_edges in &self.0 {
            cumulative += out_edges.len();
            out_edges_begin_index.push(cumulative);
        }
        let edges = self.0.into_iter().flatten().collect();
        Self::G {
            edges,
            out_edges_begin_index,
        }
    }
}
