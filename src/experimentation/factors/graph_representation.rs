use super::factor::Factor;
use crate::utils::cli;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum FactorGraphRepresentation {
    AdjListStdVec,
    AdjListPetgraph,
}

impl Factor for FactorGraphRepresentation {
    fn query_header_index() -> usize {
        3
    }
    fn name_factor() -> &'static str {
        "Graph Representation"
    }
    fn definition_factor() -> &'static str {
        "Representation of the graph in memory."
    }

    fn query_levels_from_cli() -> Vec<Self> {
        let available_levels: Vec<_> = FactorGraphRepresentation::iter().collect();
        let definitions = &[
            "Adjacency list represented by std::vec::Vec<Vec<OutEdge>>",
            "Adjacency list represented by petgraph::graph::Graph",
        ];
        cli::print_table_get_choices(Self::name_factor(), &available_levels, definitions, 0)
    }
}
impl Default for FactorGraphRepresentation {
    fn default() -> Self {
        Self::AdjListStdVec
    }
}
