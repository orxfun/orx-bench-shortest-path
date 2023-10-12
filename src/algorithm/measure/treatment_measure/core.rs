use crate::{algorithm::measure::run_measure::Measure, experimentation::treatment::Treatment};
use std::io::Write;

pub trait TreatmentMeasure: Default {
    type M: Measure;

    fn aggregate(&mut self, run_measure: Self::M);

    // cli
    fn log(&self);

    // io
    fn write_result_header<W: Write>(w: &mut W);
    fn write_result_row<W: Write>(
        &self,
        treatment: &Treatment,
        current_treatment_idx: usize,
        w: &mut W,
    );
}
