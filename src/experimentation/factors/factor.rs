use crate::utils::cli;
use std::fmt::Debug;

pub trait Factor: Clone + Debug + Default {
    fn query_header_index() -> usize;
    fn name_factor() -> &'static str;
    fn definition_factor() -> &'static str;

    fn definition_level(&self) -> String {
        format!("{:?}", self)
    }
    fn name_level(&self) -> String {
        format!("{:?}", self)
    }
    fn to_cell_string(&self) -> String {
        self.name_level()
    }

    // cli
    fn query_levels_from_cli() -> Vec<Self>;
    fn query_levels_from_cli_with_headers() -> Vec<Self> {
        cli::print_factor_header::<Self>();
        cli::print_factor_definition::<Self>();
        let levels = Self::query_levels_from_cli();
        cli::print_created_factor_levels(&levels);
        levels
    }
}
