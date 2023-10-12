use super::{core::TreatmentMeasure, time::TreatmentTime};
use crate::{
    algorithm::measure::memory::Memory,
    experimentation::{factors::factor::Factor, treatment::Treatment},
    utils::cli,
};
use std::io::Write;

#[derive(Default)]
pub struct TreatmentMemory {
    pub time: TreatmentTime,
    pub num_iterations: usize,
    pub max_num_items: usize,
    pub total_num_items_in_observations: usize,
    pub max_capacity: usize,
    pub total_capacity_in_observations: usize,
}
impl TreatmentMemory {
    pub fn average_heap_size(&self) -> f32 {
        if self.num_iterations == 0 {
            0.0
        } else {
            self.total_num_items_in_observations as f32 / self.num_iterations as f32
        }
    }
    pub fn average_heap_capacity(&self) -> f32 {
        if self.num_iterations == 0 {
            0.0
        } else {
            self.total_capacity_in_observations as f32 / self.num_iterations as f32
        }
    }
}

impl TreatmentMeasure for TreatmentMemory {
    type M = Memory;

    fn aggregate(&mut self, run_measure: Self::M) {
        self.num_iterations += run_measure.num_iterations;

        self.max_num_items = self.max_num_items.max(run_measure.max_num_items);
        self.total_num_items_in_observations += run_measure.total_num_items_in_observations;

        self.max_capacity = self.max_capacity.max(run_measure.max_capacity);
        self.total_capacity_in_observations += run_measure.total_capacity_in_observations;

        self.time.aggregate(run_measure.time);
    }
    fn log(&self) {
        let info = format!(
            "average | maximum heap length   : {:.2} | {}\n\
            average | maximum heap capacity : {:.2} | {}",
            self.average_heap_size(),
            self.max_num_items,
            self.average_heap_capacity(),
            self.max_capacity
        );

        cli::print_treatment_completion_log(&info);
    }

    fn write_result_header<W: Write>(w: &mut W) {
        use crate::experimentation::factors::*;
        writeln!(
            w,
            "i\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\tAverage Heap Size\tMaximum Number of Items\tAverage Heap Capacity\tMaximum Heap Capacity",
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
            "{}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{}\t{}\t{}\t{}",
            current_treatment_idx,
            treatment.graph_data,
            treatment.graph_representation,
            treatment.algorithm,
            treatment.algorithm_data,
            treatment.pairs,
            self.average_heap_size(),
            self.max_num_items,
            self.average_heap_capacity(),
            self.max_capacity
        )
        .expect("failed to write result");
    }
}
