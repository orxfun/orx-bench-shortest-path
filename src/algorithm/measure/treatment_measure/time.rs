use super::core::TreatmentMeasure;
use crate::{
    algorithm::measure::time::Time,
    experimentation::{factors::factor::Factor, treatment::Treatment},
    utils::cli,
};
use std::io::Write;

#[derive(Default)]
pub struct TreatmentTime {
    pub count: usize,
    pub total_elapsed_secs: f32,
}
impl TreatmentTime {
    pub fn elapsed_secs_per_treatment(&self) -> f32 {
        if self.count == 0 {
            0.0
        } else {
            self.total_elapsed_secs / self.count as f32
        }
    }
}

impl TreatmentMeasure for TreatmentTime {
    type M = Time;

    fn aggregate(&mut self, run_measure: Self::M) {
        self.count += 1;
        self.total_elapsed_secs += run_measure.elapsed_seconds;
    }
    fn log(&self) {
        let info = format!(
            "total | average-per-problem duration : {:.4} | {:.4} seconds",
            self.total_elapsed_secs,
            self.elapsed_secs_per_treatment()
        );
        cli::print_treatment_completion_log(&info);
    }

    fn write_result_header<W: Write>(w: &mut W) {
        use crate::experimentation::factors::*;
        writeln!(
            w,
            "i\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\tTotal Elapsed Seconds\tElapsed Seconds per Problem",
            graph_data::factor_graph_data::FactorGraphData::name_factor(),
            graph_representation::FactorGraphRepresentation::name_factor(),
            algorithm::factor_algorithm::FactorAlgorithm::name_factor(),
            algorithm_data::FactorAlgorithmData::name_factor(),
            pairs::FactorPairs::name_factor(),
        )
        .expect("failed to write result");
    }
    fn write_result_row<W: Write>(
        &self,
        treatment: &Treatment,
        current_treatment_idx: usize,
        w: &mut W,
    ) {
        writeln!(
            w,
            "{}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{}\t{}",
            current_treatment_idx,
            treatment.graph_data,
            treatment.graph_representation,
            treatment.algorithm,
            treatment.algorithm_data,
            treatment.pairs,
            self.total_elapsed_secs,
            self.elapsed_secs_per_treatment()
        )
        .expect("failed to write result");
    }
}
