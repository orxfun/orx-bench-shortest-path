use super::core::TreatmentMeasure;
use crate::{
    algorithm::measure::no_measurement::NoMeasurement, experimentation::treatment::Treatment,
};
use std::io::Write;

#[derive(Default)]
pub struct TreatmentNoMeasurement;

impl TreatmentMeasure for TreatmentNoMeasurement {
    type M = NoMeasurement;

    fn aggregate(&mut self, _: Self::M) {}
    fn log(&self) {}
    fn write_result_header<W: Write>(_: &mut W) {}
    fn write_result_row<W: Write>(&self, _: &Treatment, _: usize, _: &mut W) {}
}
