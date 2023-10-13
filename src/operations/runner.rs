use super::{fromfile::FromFile, help::Help, interactive::Interactive, operation::Operation};

pub fn run_command(command: &str) {
    Help::run_if_matches(command);
    Interactive::run_if_matches(command);
    FromFile::run_if_matches(command);
}

pub fn run_commands(commands: &[String]) {
    for command in commands {
        run_command(command);
    }
}
