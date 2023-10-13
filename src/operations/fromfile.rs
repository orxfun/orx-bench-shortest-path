use std::path::PathBuf;

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
    experimentation::measurement::Measurement,
    utils,
};

pub struct FromFile;

impl Operation for FromFile {
    fn name() -> &'static str {
        "fromfile"
    }
    fn matches(command: &str) -> bool {
        command.len() > Self::name().len() && command.starts_with(Self::name())
    }
    fn run(command: &str) {
        let second_part = &command[Self::name().len()..];
        if !second_part.starts_with('=') {
            panic!(
                r"fromfile must be followed by equals sign and the fiel path as in the example below:\n\
            >_ cargo run --release fromfile=results\my-experiment.json"
            );
        }
        let filepath = PathBuf::from(&second_part[1..]);
        let name = filepath
            .file_stem()
            .expect("failed to get experiment name from file path")
            .to_str()
            .unwrap();

        let experiment = utils::fs::read_json(&filepath);
        utils::cli::print_experiment(&experiment);
        match experiment.measurement {
            Measurement::Time => experiment.run_fromfile::<TreatmentTime, Time>(name),
            Measurement::Memory => experiment.run_fromfile::<TreatmentMemory, Memory>(name),
            Measurement::Validation => {
                experiment.run_fromfile::<TreatmentValidation, Validation>(name)
            }
        }
    }
}
