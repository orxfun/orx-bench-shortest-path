use super::out_edges::OutEdges;
use crate::utils::log_debug::LogDebug;

pub trait SpGraph: LogDebug {
    type OutEdges<'a>: OutEdges where where Self:'a;

    // type
    fn name() -> &'static str;

    // sp
    fn num_nodes(&self) -> usize;
    fn out_edges(&self, node: usize) -> Self::OutEdges<'_>;

    // default impl
    fn num_edges(&self) -> usize {
        (0..self.num_nodes())
            .map(|i| self.out_edges(i).count_edges())
            .sum()
    }
}
