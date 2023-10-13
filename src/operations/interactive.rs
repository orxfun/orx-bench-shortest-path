use super::operation::Operation;
use crate::{
    algorithm::measure::{
        memory::Memory,
        time::Time,
        treatment_measure::{
            memory::TreatmentMemory, time::TreatmentTime, validation::TreatmentValidation,
        },
        validation::Validation,
    },
    experimentation::{experiment::Experiment, measurement::Measurement},
    utils::cli,
};

pub struct Interactive;

impl Operation for Interactive {
    fn name() -> &'static str {
        "interactive"
    }
    fn run(_command: &str) {
        let experiment = Experiment::from_cli();
        cli::print_experiment(&experiment);
        match experiment.measurement {
            Measurement::Time => experiment.cli_run::<TreatmentTime, Time>(),
            Measurement::Memory => experiment.cli_run::<TreatmentMemory, Memory>(),
            Measurement::Validation => experiment.cli_run::<TreatmentValidation, Validation>(),
        }
    }
}
