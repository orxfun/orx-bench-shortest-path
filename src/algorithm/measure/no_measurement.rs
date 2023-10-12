use crate::Weight;

use super::run_measure::Measure;
pub struct NoMeasurement;

impl Measure for NoMeasurement {
    #[inline(always)]
    fn initialize<G: crate::graph::sp_graph::SpGraph>(_: &G, _: usize, _: usize) -> Self {
        Self
    }
    #[inline(always)]
    fn iterate(&mut self, _: usize, _: usize) {}
    #[inline(always)]
    fn finalize(&mut self, _: Option<Weight>) {}
}
