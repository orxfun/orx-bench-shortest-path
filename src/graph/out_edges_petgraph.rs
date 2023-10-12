use super::out_edges::{OutEdgeData, OutEdges};
use crate::Weight;
use petgraph::{
    graph::{EdgeReference, Edges},
    visit::EdgeRef,
    Directed,
};

pub type OutEdgesPetgraph<'a> = Edges<'a, Weight, Directed>;

impl<'a> OutEdgeData for EdgeReference<'a, Weight> {
    fn head(&self) -> usize {
        self.target().index()
    }
    fn weight(&self) -> Weight {
        *self.weight()
    }
}

impl<'a> OutEdges for OutEdgesPetgraph<'a> {
    type Edge = EdgeReference<'a, Weight>;
    fn next_edge(&mut self) -> Option<Self::Edge> {
        self.next()
    }
    fn count_edges(&mut self) -> usize {
        self.count()
    }
}
