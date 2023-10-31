use crate::utils::cli;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, Serialize, Deserialize)]
pub enum Dary {
    Binary,
    Quaternary,
    Octonary,
    D16,
    D32,
    D64,
}
impl Dary {
    pub fn d(&self) -> usize {
        match self {
            Self::Binary => 2,
            Self::Quaternary => 4,
            Self::Octonary => 8,
            Self::D16 => 16,
            Self::D32 => 32,
            Self::D64 => 64,
        }
    }
    pub fn level_from_cli() -> Vec<Self> {
        let available_levels: Vec<_> = Dary::iter().collect();
        let definitions = &[
            "d = 2^1 = 2",
            "d = 2^2 = 4",
            "d = 2^3 = 8",
            "d = 2^4 = 16",
            "d = 2^5 = 32",
            "d = 2^6 = 64",
        ];

        cli::print_subheader(3, "d Values of the d-ary Heaps");
        cli::print_table_get_choices("d", &available_levels, definitions, 0)
    }
}
