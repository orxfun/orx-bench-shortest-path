use super::out_edges::{OutEdgeData, OutEdges};
use crate::Weight;

pub type OutEdgesStdVec<'a> = std::slice::Iter<'a, OutEdge>;

#[derive(Clone, derive_new::new)]
pub struct OutEdge {
    head: usize,
    weight: Weight,
}
impl OutEdgeData for OutEdge {
    #[inline(always)]
    fn head(&self) -> usize {
        self.head
    }
    #[inline(always)]
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl<'a> OutEdges for OutEdgesStdVec<'a> {
    type Edge = OutEdge;
    fn next_edge(&mut self) -> Option<Self::Edge> {
        self.next().cloned()
    }
    fn count_edges(&mut self) -> usize {
        self.count()
    }
}
