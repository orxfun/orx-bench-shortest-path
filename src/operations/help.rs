use colored::Colorize;

use super::operation::Operation;
use crate::utils::cli;

pub struct Help;

impl Operation for Help {
    fn name() -> &'static str {
        "help"
    }
    fn run() {
        cli::print_header(None, "orx-bench-shortest-path");
        cli::print_definition_under_header("https://github.com/orxfun/orx-bench-shortest-path");

        println!("A command line tool aiming to benchmark solution approaches for the shoretst path algorithm.");

        cli::print_subheader(1, "Measures");
        cli::print_definition_under_header("Currently, the following can be measured.");
        println!("* Time       : execution time");
        println!("* Memory     : memory requirement");
        println!("* Validation : results of experimental algorithms can be validated");

        cli::print_subheader(1, "Factors");
        cli::print_definition_under_header(
            "Different levels of the following factors can be experimented.",
        );
        println!("* GraphData           : various random or testbed graph data can be used");
        println!("* GraphRepresentation : representation of the graph data structure");
        println!("* Algorithm           : shortest path algorithms and underlying queues");
        println!("* Algorithm Data      : impact of caching temporary data");

        cli::print_subheader(1, "How to use?");
        println!("Enter the following command in release mode to run the tool\n");
        println!("{}", ">_ cargo run --release c1 c2 c3".green());
        println!("\nwhere 'c1 c2 c3 ...' is an arbitrary sequence of operations / commands.");

        cli::print_subheader(2, "Available Commands");
        println!("* help        : view the help content");
        println!("* interactive : allows to define and run an experiment interactively");
    }
}
