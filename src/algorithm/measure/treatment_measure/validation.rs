use super::core::TreatmentMeasure;
use crate::{
    algorithm::measure::validation::Validation,
    experimentation::{factors::factor::Factor, treatment::Treatment},
    utils::cli,
};
use std::io::Write;

#[derive(Default)]
pub struct TreatmentValidation {
    pub wrong_results: Vec<Validation>,
}

impl TreatmentMeasure for TreatmentValidation {
    type M = Validation;

    fn aggregate(&mut self, run_measure: Self::M) {
        if !run_measure.is_correct() {
            self.wrong_results.push(run_measure);
        }
    }
    fn log(&self) {
        if !self.wrong_results.is_empty() {
            cli::print_error(&format!("* {} wrong results\n", self.wrong_results.len()));
        }
    }

    fn write_result_header<W: Write>(w: &mut W) {
        use crate::experimentation::factors::*;
        writeln!(
            w,
            "i\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\tNumber of Wrong Results\tFirst Wrong Source Sink Pair",
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
            "{}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{}\t{:?}",
            current_treatment_idx,
            treatment.graph_data,
            treatment.graph_representation,
            treatment.algorithm,
            treatment.algorithm_data,
            treatment.pairs,
            self.wrong_results.len(),
            self.wrong_results
                .first()
                .map(|f| format!("{}-{}", f.source, f.sink))
        )
        .expect("failed to write result");
    }
}
