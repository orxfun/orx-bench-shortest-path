use super::{
    experiment::Experiment,
    factors::{
        algorithm::factor_algorithm::FactorAlgorithm, algorithm_data::FactorAlgorithmData,
        factor::Factor, graph_data::factor_graph_data::FactorGraphData,
        graph_representation::FactorGraphRepresentation, pairs::FactorPairs,
    },
};
use crate::{
    algorithm::{
        measure::{run_measure::Measure, treatment_measure::core::TreatmentMeasure},
        orx_pq_dijkstra::OrxPqDijkstra,
        orx_pqdeckey_dijkstra::OrxPqDecKeyDijkstra,
        petgraph_dijkstra::PetgraphDijsktra,
        run_attempt::RunAttempt,
        sd_algorithm::ShortestDistanceAlgorithm,
        solution::Solution,
        std_pq_dijkstra::StdPqDijkstra,
    },
    experimentation::factors::algorithm::{
        dary::Dary, dijkstra::Dijkstra, priority_queue::PriorityQueue,
        priority_queue_deckey::PriorityQueueDecKey,
    },
    graph::{
        adjlist_flat_vec::AdjListFlatVecBuilder, adjlist_jagged_vec::AdjListJaggedVec,
        adjlist_petgraph::AdjListPetgraph, sp_graph::SpGraph, sp_graph_builder::SpGraphBuilder,
    },
    Weight,
};
use itertools::Itertools;
use orx_priority_queue::{DaryHeap, DaryHeapOfIndices, DaryHeapWithMap};
use prettytable::{Cell, Row, Table};

#[derive(Clone)]
pub struct Treatment {
    pub graph_data: FactorGraphData,
    pub graph_representation: FactorGraphRepresentation,
    pub algorithm: FactorAlgorithm,
    pub algorithm_data: FactorAlgorithmData,
    pub pairs: FactorPairs,
}

impl Treatment {
    pub(crate) fn add_table_header(table: &mut Table, experiment: &Experiment) {
        fn add_cell<F: Factor>(cells: &mut Vec<&str>, num_exp_levels: usize) {
            if num_exp_levels > 1 {
                cells.push(F::name_factor());
            }
        }

        let mut cells = vec!["i"];
        let xp = experiment;
        add_cell::<FactorGraphData>(&mut cells, xp.graph_data.len());
        add_cell::<FactorGraphRepresentation>(&mut cells, xp.graph_representation.len());
        add_cell::<FactorAlgorithm>(&mut cells, xp.algorithm.len());
        add_cell::<FactorAlgorithmData>(&mut cells, xp.algorithm_data.len());
        add_cell::<FactorPairs>(&mut cells, xp.pairs.len());

        let pretty_cells = cells.iter().map(|x| Cell::new(x)).collect_vec();
        table.add_row(Row::new(pretty_cells));
    }
    pub(crate) fn add_table_row(&self, table: &mut Table, experiment: &Experiment, i: usize) {
        fn add_cell<F: Factor>(cells: &mut Vec<String>, num_exp_levels: usize, level: &F) {
            if num_exp_levels > 1 {
                cells.push(level.to_cell_string());
            }
        }

        let mut cells = vec![i.to_string()];
        let xp = experiment;
        add_cell(&mut cells, xp.graph_data.len(), &self.graph_data);
        add_cell(
            &mut cells,
            xp.graph_representation.len(),
            &self.graph_representation,
        );
        add_cell(&mut cells, xp.algorithm.len(), &self.algorithm);
        add_cell(&mut cells, xp.algorithm_data.len(), &self.algorithm_data);
        add_cell(&mut cells, xp.pairs.len(), &self.pairs);

        let pretty_cells = cells.iter().map(|x| Cell::new(x.as_str())).collect_vec();
        table.add_row(Row::new(pretty_cells));
    }
    pub(crate) fn short_definition(&self, experiment: &Experiment) -> String {
        fn add<F: Factor>(cells: &mut Vec<String>, num_exp_levels: usize, level: &F) {
            if num_exp_levels > 1 {
                cells.push(level.name_level());
            }
        }
        let mut def = vec![];
        let xp = experiment;
        add(&mut def, xp.graph_data.len(), &self.graph_data);
        add(
            &mut def,
            xp.graph_representation.len(),
            &self.graph_representation,
        );
        add(&mut def, xp.algorithm.len(), &self.algorithm);
        add(&mut def, xp.algorithm_data.len(), &self.algorithm_data);
        add(&mut def, xp.pairs.len(), &self.pairs);

        def.join(" | ")
    }

    // RUN
    pub fn run<E, M>(&self) -> RunAttempt<M, E>
    where
        M: Measure,
        E: TreatmentMeasure<M = M>,
    {
        let special = self.run_special::<E, M>();
        if let Some(s) = special {
            return s;
        }

        match self.graph_representation {
            FactorGraphRepresentation::AdjListJaggedVec => self
                .run_with_graph_builder(self.graph_data.create_graph_builder::<AdjListJaggedVec>()),
            FactorGraphRepresentation::AdjListFlatVec => self.run_with_graph_builder(
                self.graph_data
                    .create_graph_builder::<AdjListFlatVecBuilder>(),
            ),
            FactorGraphRepresentation::AdjListPetgraph => self
                .run_with_graph_builder(self.graph_data.create_graph_builder::<AdjListPetgraph>()),
        }
    }

    // handle special matches
    fn run_special<E, M>(&self) -> Option<RunAttempt<M, E>>
    where
        M: Measure,
        E: TreatmentMeasure<M = M>,
    {
        #[allow(irrefutable_let_patterns)]
        if let FactorAlgorithm::Dijkstra(dijkstra) = &self.algorithm {
            if matches!(dijkstra, Dijkstra::Petgraph) {
                let attempt = match self.graph_representation {
                    FactorGraphRepresentation::AdjListPetgraph => {
                        let graph = self.graph_data.create_graph_builder::<AdjListPetgraph>();

                        #[cfg(feature = "dhat-heap")]
                        let _profiler = dhat::Profiler::new_heap();

                        let graph = graph.build();
                        let algorithm = PetgraphDijsktra::new(&graph);
                        self.run_with_graph_alg(graph, algorithm)
                    }
                    _ => {
                        let err = String::from("(graph, algorithm) mismatch:\npetgraph::algo::Dijkstra can only be run with petgraph::graph::Graph");
                        RunAttempt::NotCompleted(err)
                    }
                };
                return Some(attempt);
            }
        }
        None
    }
    // all graphs
    fn run_with_graph_builder<E, M, G, B>(&self, graph_builder: B) -> RunAttempt<M, E>
    where
        M: Measure,
        G: SpGraph,
        B: SpGraphBuilder<G = G>,
        E: TreatmentMeasure<M = M>,
    {
        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();

        let graph = graph_builder.build();
        match &self.algorithm {
            FactorAlgorithm::Dijkstra(dijkstra) => match dijkstra {
                Dijkstra::PriorityQueue(pq) => self.run_with_graph_pq(pq, graph),
                Dijkstra::PriorityQueueDecKey(pqdk) => self.run_with_graph_pqdk(pqdk, graph),
                Dijkstra::Petgraph => panic!("must've been handled with run_special"),
            },
        }
    }
    fn run_with_graph_pq<E, M, G>(&self, pq: &PriorityQueue, graph: G) -> RunAttempt<M, E>
    where
        M: Measure,
        G: SpGraph,
        E: TreatmentMeasure<M = M>,
    {
        type OrxBinary = DaryHeap<usize, Weight, 2>;
        type OrxQuaternary = DaryHeap<usize, Weight, 4>;
        type OrxOctonary = DaryHeap<usize, Weight, 8>;
        type OrxD16 = DaryHeap<usize, Weight, 16>;
        type OrxD32 = DaryHeap<usize, Weight, 32>;
        type OrxD64 = DaryHeap<usize, Weight, 64>;

        match pq {
            PriorityQueue::StdBinaryHeap => {
                let algorithm = StdPqDijkstra::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
            PriorityQueue::OrxDaryHeap(dary) => match dary {
                Dary::Binary => {
                    let algorithm = OrxPqDijkstra::<OrxBinary>::new(&graph);
                    self.run_with_graph_alg(graph, algorithm)
                }
                Dary::Quaternary => {
                    let algorithm = OrxPqDijkstra::<OrxQuaternary>::new(&graph);
                    self.run_with_graph_alg(graph, algorithm)
                }
                Dary::Octonary => {
                    let algorithm = OrxPqDijkstra::<OrxOctonary>::new(&graph);
                    self.run_with_graph_alg(graph, algorithm)
                }
                Dary::D16 => {
                    let algorithm = OrxPqDijkstra::<OrxD16>::new(&graph);
                    self.run_with_graph_alg(graph, algorithm)
                }
                Dary::D32 => {
                    let algorithm = OrxPqDijkstra::<OrxD32>::new(&graph);
                    self.run_with_graph_alg(graph, algorithm)
                }
                Dary::D64 => {
                    let algorithm = OrxPqDijkstra::<OrxD64>::new(&graph);
                    self.run_with_graph_alg(graph, algorithm)
                }
            },
        }
    }
    fn run_with_graph_pqdk<E, M, G>(&self, pqdk: &PriorityQueueDecKey, graph: G) -> RunAttempt<M, E>
    where
        M: Measure,
        G: SpGraph,
        E: TreatmentMeasure<M = M>,
    {
        match pqdk {
            PriorityQueueDecKey::OrxDaryHeapOfIndices(dary) => {
                self.run_with_graph_pqdk_idx(dary, graph)
            }
            PriorityQueueDecKey::OrxDaryHeapWithMap(dary) => {
                self.run_with_graph_pqdk_map(dary, graph)
            }
        }
    }
    fn run_with_graph_pqdk_idx<E, M, G>(&self, dary: &Dary, graph: G) -> RunAttempt<M, E>
    where
        M: Measure,
        G: SpGraph,
        E: TreatmentMeasure<M = M>,
    {
        type OrxIdxBinary = DaryHeapOfIndices<usize, Weight, 2>;
        type OrxIdxQuaternary = DaryHeapOfIndices<usize, Weight, 4>;
        type OrxIdxOctonary = DaryHeapOfIndices<usize, Weight, 8>;
        type OrxIdxD16 = DaryHeapOfIndices<usize, Weight, 16>;
        type OrxIdxD32 = DaryHeapOfIndices<usize, Weight, 32>;
        type OrxIdxD64 = DaryHeapOfIndices<usize, Weight, 64>;

        match dary {
            Dary::Binary => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxIdxBinary>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
            Dary::Quaternary => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxIdxQuaternary>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
            Dary::Octonary => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxIdxOctonary>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
            Dary::D16 => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxIdxD16>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
            Dary::D32 => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxIdxD32>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
            Dary::D64 => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxIdxD64>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
        }
    }
    fn run_with_graph_pqdk_map<E, M, G>(&self, dary: &Dary, graph: G) -> RunAttempt<M, E>
    where
        M: Measure,
        G: SpGraph,
        E: TreatmentMeasure<M = M>,
    {
        type OrxMapBinary = DaryHeapWithMap<usize, Weight, 2>;
        type OrxMapQuaternary = DaryHeapWithMap<usize, Weight, 4>;
        type OrxMapOctonary = DaryHeapWithMap<usize, Weight, 8>;
        type OrxMapD16 = DaryHeapWithMap<usize, Weight, 16>;
        type OrxMapD32 = DaryHeapWithMap<usize, Weight, 32>;
        type OrxMapD64 = DaryHeapWithMap<usize, Weight, 64>;

        match dary {
            Dary::Binary => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxMapBinary>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
            Dary::Quaternary => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxMapQuaternary>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
            Dary::Octonary => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxMapOctonary>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
            Dary::D16 => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxMapD16>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
            Dary::D32 => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxMapD32>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
            Dary::D64 => {
                let algorithm = OrxPqDecKeyDijkstra::<OrxMapD64>::new(&graph);
                self.run_with_graph_alg(graph, algorithm)
            }
        }
    }
    fn run_with_graph_alg<E, M, G, A>(&self, graph: G, mut algorithm: A) -> RunAttempt<M, E>
    where
        M: Measure,
        G: SpGraph,
        A: ShortestDistanceAlgorithm<G>,
        E: TreatmentMeasure<M = M>,
    {
        let pairs = self.pairs.create_pairs(graph.num_nodes());
        let mut exp_measure = E::default();
        for (source, sink) in pairs {
            let solution: Solution<M> = match self.algorithm_data {
                FactorAlgorithmData::Cached => algorithm.run_cached(&graph, source, sink),
                FactorAlgorithmData::Pure => A::run_pure(&graph, source, sink),
            };
            exp_measure.aggregate(solution.measure);
        }

        exp_measure.into()
    }
}
