use super::run_measure::Measure;
use crate::{graph::sp_graph::SpGraph, Weight};
use std::time::SystemTime;

pub struct Time {
    pub start_time: SystemTime,
    pub elapsed_seconds: f32,
}
impl Time {
    pub fn start() -> Self {
        Self {
            start_time: SystemTime::now(),
            elapsed_seconds: f32::INFINITY,
        }
    }
    pub fn stop(&mut self) {
        let elapsed = self.start_time.elapsed().unwrap();
        self.elapsed_seconds = elapsed.as_secs_f32();
    }
}
impl Measure for Time {
    fn initialize<G: SpGraph>(_: &G, _: usize, _: usize) -> Self {
        Self::start()
    }
    #[inline(always)]
    fn iterate(&mut self, _: usize, _: usize) {}
    fn finalize(&mut self, _: Option<Weight>) {
        self.stop()
    }
}
