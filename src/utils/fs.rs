use super::cli;
use std::path::{Path, PathBuf};

pub fn create_dir_if_absent(dir: &Path) {
    if !dir.exists() {
        std::fs::create_dir(dir).expect("failed to create temporary data directory");
    }
}

// RESULTS
const FOLDER_RESULTS: &str = r"results";
fn result_filepath(name: &str, counter: Option<usize>) -> PathBuf {
    let dir = PathBuf::from(FOLDER_RESULTS);
    match counter {
        None => dir.join(format!("{}.txt", name)),
        Some(i) => dir.join(format!("{}-{}.txt", name, i)),
    }
}
pub fn get_file_path(name: &str) -> PathBuf {
    create_dir_if_absent(PathBuf::from(FOLDER_RESULTS).as_path());

    let mut path = result_filepath(name, None);
    let mut counter = 2;
    while path.exists() {
        path = result_filepath(name, Some(counter));
        counter += 1;
    }
    if counter > 2 {
        cli::print_error(&format!(
            "Result file '{}' already existed; hence, results are being written to '{}'",
            name,
            path.to_str().unwrap()
        ))
    }
    path.to_path_buf()
}
