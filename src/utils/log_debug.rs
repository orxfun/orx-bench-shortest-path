use crate::graph::{
    out_edges::{OutEdgeData, OutEdges},
    sp_graph::SpGraph,
};

pub trait LogDebug {
    #[allow(dead_code)]
    fn debug_log(&self);
}

impl<G: SpGraph> LogDebug for G {
    fn debug_log(&self) {
        println!(
            "{}: N x A = {} x {}",
            G::name(),
            self.num_nodes(),
            self.num_edges()
        );
        for i in 0..self.num_nodes() {
            print!("{} =>", i);
            let mut edges = self.out_edges(i);
            while let Some(edge) = edges.next_edge() {
                print!(" {}({})", edge.head(), edge.weight());
            }
            println!();
        }
    }
}
