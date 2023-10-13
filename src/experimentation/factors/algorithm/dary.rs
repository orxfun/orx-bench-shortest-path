use crate::utils::cli;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, Serialize, Deserialize)]
pub enum Dary {
    Binary,
    Quaternary,
    Octonary,
    Hexadecimal,
}
impl Dary {
    pub fn d(&self) -> usize {
        match self {
            Self::Binary => 2,
            Self::Quaternary => 4,
            Self::Octonary => 8,
            Self::Hexadecimal => 16,
        }
    }
    pub fn level_from_cli() -> Vec<Self> {
        let available_levels: Vec<_> = Dary::iter().collect();
        let definitions = &["d = 2", "d = 4", "d = 8", "d = 16"];

        cli::print_subheader(3, "d Values of the d-ary Heaps");
        cli::print_table_get_choices("d", &available_levels, definitions, 0)
    }
}
