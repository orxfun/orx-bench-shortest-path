use super::{
    factors::{
        algorithm::factor_algorithm::FactorAlgorithm, factor::Factor,
        graph_representation::FactorGraphRepresentation,
    },
    measurement::Measurement,
    treatment::Treatment,
};
use crate::{
    algorithm::{
        measure::{run_measure::Measure, time::Time, treatment_measure::core::TreatmentMeasure},
        run_attempt::RunAttempt,
    },
    experimentation::factors::{
        algorithm_data::FactorAlgorithmData, graph_data::factor_graph_data::FactorGraphData,
        pairs::FactorPairs,
    },
    utils::{self, cli},
};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Experiment {
    pub measurement: Measurement,
    pub graph_data: Vec<FactorGraphData>,
    pub graph_representation: Vec<FactorGraphRepresentation>,
    pub algorithm: Vec<FactorAlgorithm>,
    pub algorithm_data: Vec<FactorAlgorithmData>,
    pub pairs: Vec<FactorPairs>,
}

impl Experiment {
    pub fn from_cli() -> Self {
        let measurement = Measurement::query_from_cli();
        let graph_data = FactorGraphData::query_levels_from_cli_with_headers();
        let graph_representation = FactorGraphRepresentation::query_levels_from_cli_with_headers();
        let algorithm = FactorAlgorithm::query_levels_from_cli_with_headers();
        let algorithm_data = FactorAlgorithmData::query_levels_from_cli_with_headers();
        let pairs = FactorPairs::query_levels_from_cli_with_headers();
        Self {
            measurement,
            graph_data,
            graph_representation,
            algorithm,
            algorithm_data,
            pairs,
        }
    }
    pub fn num_treatments(&self) -> usize {
        self.graph_data.len()
            * self.graph_representation.len()
            * self.algorithm.len()
            * self.algorithm_data.len()
            * self.pairs.len()
    }
    pub fn create_treatments(&self) -> Vec<Treatment> {
        let mut treatments = vec![];
        for &graph_data in &self.graph_data {
            for &graph_representation in &self.graph_representation {
                for &algorithm in &self.algorithm {
                    for &algorithm_data in &self.algorithm_data {
                        for &pairs in &self.pairs {
                            treatments.push(Treatment {
                                graph_data,
                                graph_representation,
                                algorithm,
                                algorithm_data,
                                pairs,
                            });
                        }
                    }
                }
            }
        }
        treatments
    }

    // run
    pub fn cli_run<E, M>(self)
    where
        M: Measure,
        E: TreatmentMeasure<M = M>,
    {
        let experiment_name = cli::print_scalar_query_get_answer(
            "experiment name",
            String::from("experiment"),
            // todo: must validate that experiment name is a valid filename (sanitize-filename crate can help)
            |_| true,
        );
        self.run_core::<E, M>(&experiment_name, true);
    }
    pub fn run_fromfile<E, M>(self, experiment_name: &str)
    where
        M: Measure,
        E: TreatmentMeasure<M = M>,
    {
        self.run_core::<E, M>(experiment_name, false);
    }
    fn run_core<E, M>(&self, experiment_name: &str, interactive: bool)
    where
        M: Measure,
        E: TreatmentMeasure<M = M>,
    {
        let (path_exp, path_res) =
            utils::fs::get_experiment_and_results_paths(experiment_name, interactive);
        utils::fs::write_json(self, &path_exp);
        cli::print_experiment_written_to(path_exp.to_str().unwrap());

        let mut file = utils::fs::create_file(&path_res);
        E::write_result_header(&mut file);

        let i = interactive;
        if self.num_treatments() > 1 && !cli::echo_continue(i, "create treatments table") {
            return;
        }

        let treatments = self.create_treatments();
        if treatments.len() > 1 {
            cli::echo(i, || cli::print_treatments(self, &treatments, usize::MAX));
        }

        if !cli::echo_continue(i, "run full-factorial experiment") {
            return;
        }

        cli::echo(i, cli::print_experiment_run_start);

        let mut exp_time = Time::start();
        let mut num_not_completed = 0;

        for (t, treatment) in treatments.iter().enumerate() {
            let attempt = treatment.run::<E, M>();
            cli::print_treatment_completion_progress_bar(
                t,
                treatments.len(),
                &treatment.short_definition(self),
            );
            match &attempt {
                RunAttempt::NotCompleted(why) => {
                    cli::print_error(&format!("[T-{}-NotCompleted] {}\n", t, why));
                    num_not_completed += 1;
                }
                RunAttempt::Succeeded(measure) => {
                    measure.write_result_row(treatment, t, &mut file);
                    measure.log();
                }
            };

            if t % 100 == 0 {
                file.flush().expect("failed to flush results writer");
            }
        }

        exp_time.stop();
        cli::print_experiment_run_end(
            &exp_time,
            treatments.len(),
            num_not_completed,
            path_res.to_str().unwrap(),
        );
    }
}
