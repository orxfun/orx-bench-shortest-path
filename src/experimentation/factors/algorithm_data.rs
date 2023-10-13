use super::factor::Factor;
use crate::utils::cli;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, Serialize, Deserialize)]
pub enum FactorAlgorithmData {
    Cached,
    Pure,
}

impl Factor for FactorAlgorithmData {
    fn query_header_index() -> usize {
        5
    }
    fn name_factor() -> &'static str {
        "Caching"
    }
    fn definition_factor() -> &'static str {
        "Usage pattern of algorithm's internal temporary data."
    }

    fn query_levels_from_cli() -> Vec<Self> {
        let available_levels: Vec<_> = FactorAlgorithmData::iter().collect();
        let definitions = &[
            "Reuses once allocated internal data for all (s,t) pairs\n\
            Leads to an impure run method requiring `&mut self`.",
            "Recreates internal data for each (s,t) pair\n\
            Leads to a pure run method only requiring `&self`.",
        ];
        cli::print_table_get_choices(Self::name_factor(), &available_levels, definitions, 0)
    }
}
impl Default for FactorAlgorithmData {
    fn default() -> Self {
        Self::Cached
    }
}
