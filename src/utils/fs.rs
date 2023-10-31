use super::cli;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

pub fn create_file(path: &Path) -> File {
    std::fs::File::create(path)
        .unwrap_or_else(|_| panic!("failed to create file '{}'", path.to_str().unwrap_or("?")))
}

pub fn create_dir_if_absent(dir: &Path) {
    if !dir.exists() {
        std::fs::create_dir(dir).expect("failed to create temporary data directory");
    }
}

// RESULTS
const FOLDER_RESULTS: &str = r"results";
fn experiment_and_results_filepath(name: &str, counter: Option<usize>) -> (PathBuf, PathBuf) {
    let dir = PathBuf::from(FOLDER_RESULTS);
    match counter {
        None => (
            dir.join(format!("{}.json", name)),
            dir.join(format!("result-{}.txt", name)),
        ),
        Some(i) => (
            dir.join(format!("{}-{}.json", name, i)),
            dir.join(format!("result-{}-{}.txt", name, i)),
        ),
    }
}

pub fn get_experiment_and_results_paths(name: &str, interactive: bool) -> (PathBuf, PathBuf) {
    create_dir_if_absent(PathBuf::from(FOLDER_RESULTS).as_path());

    let mut paths = experiment_and_results_filepath(name, None);
    let mut counter = 2;
    while (interactive && paths.0.exists()) || paths.1.exists() {
        paths = experiment_and_results_filepath(name, Some(counter));
        counter += 1;
    }
    if interactive && counter > 2 {
        cli::print_error(&format!(
            "Experiment or Result file for '{}' already existed; hence,\n* experiment is written to '{}'\n* results are being written to '{}'",
            name,
            paths.0.to_str().unwrap(),
            paths.1.to_str().unwrap()
        ))
    }
    (paths.0.to_path_buf(), paths.1.to_path_buf())
}

// Serialize
pub fn write_json<S: Serialize>(value: &S, path: &Path) {
    let file = File::create(path)
        .unwrap_or_else(|e| panic!("failed to create file '{}': {}", path.to_str().unwrap(), e));
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, value)
        .unwrap_or_else(|e| panic!("failed to save file '{}': {}", path.to_str().unwrap(), e));
}
pub fn read_json<D: DeserializeOwned>(path: &Path) -> D {
    let file = File::open(path)
        .unwrap_or_else(|e| panic!("failed to open file '{}': {}", path.to_str().unwrap(), e));
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|e| {
        panic!(
            "failed to read/deserialize file '{}': {}",
            path.to_str().unwrap(),
            e
        )
    })
}
