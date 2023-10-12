#[macro_use]
extern crate prettytable;
extern crate progress;

mod algorithm;
mod experimentation;
mod graph;
mod operations;
mod utils;

pub type Weight = u64;

fn main() {
    let commands: Vec<String> = std::env::args().skip(1).map(|x| x.to_lowercase()).collect();
    operations::runner::run_commands(&commands);
}
