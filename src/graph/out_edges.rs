use crate::Weight;

pub trait OutEdgeData {
    fn head(&self) -> usize;
    fn weight(&self) -> Weight;
}

pub trait OutEdges {
    type Edge: OutEdgeData;
    fn next_edge(&mut self) -> Option<Self::Edge>;
    fn count_edges(&mut self) -> usize;
}
