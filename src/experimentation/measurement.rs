use crate::utils::cli;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, EnumIter)]
pub enum Measurement {
    Time,
    Memory,
    Validation,
}

impl Measurement {
    pub fn query_from_cli() -> Self {
        let available_levels: Vec<_> = Measurement::iter().collect();
        let definitions = &[
            "Measures time.",
            "Measures memory; \n\
            time is also kept; however, not to be treated as accurate \n\
            as memory measurement might slow down the execution.",
            "Validates generated solutions;\n\
            using solution created by a reference algorithm.",
        ];

        cli::print_header(Some(1), "Measurement");
        cli::print_definition_under_header("Metric to be measured by the experiment.");
        let value = cli::print_table_get_choice(&available_levels, definitions, 2);
        cli::print_selection(&format!("{:?}", value));
        value
    }
}
