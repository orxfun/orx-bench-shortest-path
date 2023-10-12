use crate::{
    algorithm::measure::time::Time,
    experimentation::{experiment::Experiment, factors::factor::Factor, treatment::Treatment},
};
use colored::Colorize;
use itertools::Itertools;
use prettytable::Table;
use std::{fmt::Debug, str::FromStr};

// print
pub fn print_header(header_index: Option<usize>, text: &str) {
    println!("\n\n");
    if let Some(header_index) = header_index {
        print!("{}. ", header_index.to_string().yellow().bold());
    }
    println!("{}", text.yellow().bold());
}
pub fn print_subheader(level: usize, text: &str) {
    let header = format!("\n{} {}", "#".repeat(level), text);
    println!("{}", header.bright_yellow().dimmed());
}
pub fn print_definition_under_header(definition: &str) {
    println!("{}", definition.dimmed());
}
pub fn print_selection(selected: &str) {
    println!("=> {}", selected.bright_green().dimmed());
}
pub fn print_selections<T, F, S>(selections: &[T], define: F)
where
    F: Fn(&T) -> S,
    S: AsRef<str>,
{
    for selected in selections {
        print_selection(define(selected).as_ref());
    }
}

// print & query
pub fn query_continue(action: &str) -> bool {
    let query_txt = format!("n to cancel, any other key to {}", action);
    let query = print_scalar_query_get_answer(&query_txt, "y".to_string(), |_| true)
        .trim()
        .to_lowercase();

    if query == "n" {
        print_error("canceled");
        false
    } else {
        true
    }
}
pub fn print_table_get_choices<T>(
    choice: &str,
    available_levels: &[T],
    definitions: &[&str],
    default_index: usize,
) -> Vec<T>
where
    T: Clone + Debug,
{
    assert!(!available_levels.is_empty());
    assert!((0..available_levels.len()).contains(&default_index));
    assert_eq!(available_levels.len(), definitions.len());

    let mut levels_table = Table::new();
    for (i, level) in available_levels.iter().enumerate() {
        levels_table.add_row(row![
            i,
            format!(
                "{:?}{}",
                level,
                if i == default_index { " (default)" } else { "" }
            ),
            definitions[i]
        ]);
    }
    println!("{}", levels_table);

    if available_levels.len() == 1 {
        print_selection(&format!("{:?}", available_levels[0]));
        vec![available_levels[0].clone()]
    } else {
        print_scalar_query_get_answers(&format!("indices of {}", choice), default_index, |&i| {
            i < available_levels.len()
        })
        .iter()
        .unique()
        .map(|&i| available_levels[i].clone())
        .collect()
    }
}
pub fn print_scalar_query_get_answers<T, V>(factor: &str, default_level: T, validator: V) -> Vec<T>
where
    T: FromStr + Debug,
    V: Fn(&T) -> bool,
{
    let enter = format!("enter comma-separated '{}' levels", factor);
    let default = format!("default value = {:?}", default_level);
    println!("{} ({}):", enter.italic().bold(), default.dimmed());

    let mut std_input = String::new();
    let _ = std::io::stdin().read_line(&mut std_input).ok();
    let inputs = std_input.trim();
    if inputs.is_empty() {
        vec![default_level]
    } else {
        let mut valid_inputs: Vec<_> = inputs
            .split(',')
            .unique()
            .flat_map(|input| match input.parse::<T>() {
                Err(_) => {
                    print_error(&format!("skipping input '{}', failed to parse", input));
                    None
                }
                Ok(parsed) => {
                    if validator(&parsed) {
                        Some(parsed)
                    } else {
                        print_error(&format!("skipping input '{}', validation failed", input));
                        None
                    }
                }
            })
            .collect();
        if valid_inputs.is_empty() {
            print_error("no valid entries are received, choosing the default value");
            valid_inputs.push(default_level)
        }
        valid_inputs
    }
}

pub fn print_table_get_choice<T>(
    available_levels: &[T],
    definitions: &[&str],
    default_index: usize,
) -> T
where
    T: Clone + Debug,
{
    assert!(!available_levels.is_empty());
    assert!((0..available_levels.len()).contains(&default_index));
    assert_eq!(available_levels.len(), definitions.len());

    let mut levels_table = Table::new();
    for (i, level) in available_levels.iter().enumerate() {
        levels_table.add_row(row![
            i,
            format!(
                "{:?}{}",
                level,
                if i == default_index { " (default)" } else { "" }
            ),
            definitions[i]
        ]);
    }
    println!("{}", levels_table);

    let final_choice = if available_levels.len() == 1 {
        print_selection(&format!("{:?}", available_levels[0]));
        &available_levels[0]
    } else {
        let chosen = print_scalar_query_get_answer("index to select", default_index, |&i| {
            i < available_levels.len()
        });
        &available_levels[chosen]
    };
    final_choice.clone()
}
pub fn print_scalar_query_get_answer<T, V>(factor: &str, default_level: T, validator: V) -> T
where
    T: FromStr + Debug,
    V: Fn(&T) -> bool,
{
    let enter = format!("enter {}", factor);
    let default = format!("default value = {:?}", default_level);
    println!("{} ({}):", enter.italic().bold(), default.dimmed());

    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    let value = if input.is_empty() {
        default_level
    } else {
        match input.trim().parse::<T>() {
            Err(_) => {
                print_error("failed to parse input; using default");
                default_level
            }
            Ok(parsed) => {
                if validator(&parsed) {
                    parsed
                } else {
                    print_error("invalid value entered; using default");
                    default_level
                }
            }
        }
    };
    value
}
pub fn print_error(error: &str) {
    println!("{}", error.bright_red());
}

// factor wrappers
pub fn print_factor_header<F: Factor>() {
    print_header(Some(F::query_header_index()), F::name_factor())
}
pub fn print_factor_definition<F: Factor>() {
    print_definition_under_header(F::definition_factor())
}
pub fn print_created_factor_levels<F: Factor>(levels: &[F]) {
    print_selections(levels, |level| level.name_level())
}

// experiment
pub fn print_experiment(experiment: &Experiment) {
    type Col = (String, String);
    fn split<F: Factor>(constants: &mut Vec<String>, level_cols: &mut Vec<Col>, levels: &[F]) {
        assert!(!levels.is_empty());
        if levels.len() == 1 {
            constants.push(format!("* {} = {:?}", F::name_factor(), &levels[0]));
        } else {
            level_cols.push((
                format!("{} ({})", F::name_factor(), levels.len()),
                levels.iter().map(|level| format!("{:?}", level)).join("\n"),
            ));
        }
    }
    let mut constants = vec![];
    let mut level_cols = vec![];
    split(&mut constants, &mut level_cols, &experiment.graph_data);
    split(
        &mut constants,
        &mut level_cols,
        &experiment.graph_representation,
    );
    split(&mut constants, &mut level_cols, &experiment.algorithm);
    split(&mut constants, &mut level_cols, &experiment.algorithm_data);
    split(&mut constants, &mut level_cols, &experiment.pairs);

    print_header(None, &format!("EXPERIMENT - {:?}", experiment.measurement));
    print_definition_under_header(&format!(
        "number of treatments = {}",
        experiment.num_treatments()
    ));
    if !constants.is_empty() {
        print_subheader(1, "Factors with Fixed Levels");
        for constant in constants {
            println!("{}", constant);
        }
    }
    if !level_cols.is_empty() {
        print_subheader(1, "Factors with Muiltiple Levels");
        for (factor, levels) in level_cols {
            print_subheader(2, &factor);
            println!("{}", table![[levels]]);
        }
    }
}
pub fn print_treatments(
    experiment: &Experiment,
    treatments: &[Treatment],
    max_num_treatments: usize,
) {
    let mut levels = Table::new();
    Treatment::add_table_header(&mut levels, experiment);
    for (i, t) in treatments.iter().take(max_num_treatments).enumerate() {
        t.add_table_row(&mut levels, experiment, i);
    }
    let num_remaining = if max_num_treatments > treatments.len() {
        0
    } else {
        treatments.len() - max_num_treatments
    };
    if num_remaining > 0 {
        levels.add_row(row![format!("{} more ...", num_remaining)]);
    }

    print_subheader(1, "Treatments Table");
    println!("{}", levels);
}

// experiment run
pub fn print_experiment_run_start() {
    let starting_message = "running experiment ...";
    println!("{}\n", starting_message.italic().dimmed());
}
pub fn print_experiment_run_end(
    exp_time: &Time,
    num_treatments: usize,
    num_not_completed: usize,
    results_path: &str,
) {
    let completion_message = format!(
        "\nexperiment run completed in {:.2} seconds",
        exp_time.elapsed_seconds
    );
    println!("{}", completion_message.italic());

    if num_not_completed > 0 {
        let not_completed_msg = format!(
            "{} out of {} treatments were not run and not included in the output \
            see the reasons in the corresponding log messages",
            num_not_completed, num_treatments
        );
        println!("{}", not_completed_msg.red().dimmed().italic());
    }

    println!("Results are written to:\n{}\n", results_path.green());
}
pub fn print_treatment_completion_progress_bar(
    current_treatment_idx: usize,
    num_treatments: usize,
    treatment_definition: &str,
) {
    assert!(current_treatment_idx < num_treatments);

    const LINE_LEN: usize = 100;

    let idx = format!("[T-{}]  ", current_treatment_idx);
    let line_len = LINE_LEN - idx.len();

    let completed = current_treatment_idx + 1;
    let done = line_len * completed / num_treatments;
    let remaining = line_len - done;
    let text = format!("{}{}{}", idx, "#".repeat(done), "-".repeat(remaining));
    println!("{}", text.bright_green());

    if !treatment_definition.is_empty() {
        let def = treatment_definition;
        let start = 0;
        let mut end = (start + LINE_LEN).min(def.len());
        println!("{}", &def[start..end].white().dimmed());
        while end < def.len() {
            let start = end;
            end = (start + LINE_LEN).min(def.len());
            println!("{}", &def[start..end].white().dimmed());
        }
    }
}
pub fn print_treatment_completion_log(information: &str) {
    println!("{}\n", information.bright_white().dimmed());
}

// conditional
pub fn echo<A: Fn()>(interactive: bool, action: A) {
    if interactive {
        action();
    }
}
pub fn echo_continue(interactive: bool, action: &str) -> bool {
    !interactive || query_continue(action)
}
