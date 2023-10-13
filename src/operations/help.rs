use colored::Colorize;

use super::operation::Operation;
use crate::utils::cli;

pub struct Help;

impl Operation for Help {
    fn name() -> &'static str {
        "help"
    }
    fn run(_command: &str) {
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
        println!("* help        : view the help content.");
        println!("* interactive : allows to define and run an experiment interactively.");
        println!(
            "* fromfile    : allows to run an experiment from a json experiment file:\n\
            \t\texperiment definition files can be created manually;\n\
            \t\thowever, each interactive run does also create the experiment file."
        );

        cli::print_subheader(2, "Examples");
        println!("{}", ">_ cargo run --release help".green());
        println!("{}", "to view this content".italic());
        println!();
        println!("{}", ">_ cargo run --release interactive".green());
        println!(
            "{}",
            "to define and run the experimentation interactively".italic()
        );
        println!();
        println!(
            "{}",
            r">_ cargo run --release fromfile=results\my-experiment.json".green()
        );
        println!(
            "{}",
            "to directly run the experiment defined in the given file".italic()
        );
    }
}
