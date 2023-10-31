use crate::{
    graph::sp_graph_builder::SpGraphBuilder,
    utils::{self, cli},
    Weight,
};
use serde::{Deserialize, Serialize};
use std::{io::BufRead, path::PathBuf};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const DATA_FOLDER: &str = r"benchmark_data/dimacs_9th/";

#[derive(Clone, Copy, Debug, EnumIter, Default, Serialize, Deserialize)]
pub enum GraphDimacs9th {
    #[default]
    USARoaddNY,
    USARoaddBAY,
    USARoaddCOL,
    USARoaddFLA,
    USARoaddLKS,
    USARoaddW,
    USARoaddUSA,
}

impl GraphDimacs9th {
    pub fn level_from_cli() -> Vec<Self> {
        let available_levels: Vec<_> = GraphDimacs9th::iter().collect();
        let definitions = &[
            "(NxA)=(264346, 733846)\nhttp://www.diag.uniroma1.it/~challenge9/data/USA-road-d/USA-road-d.NY.gr.gz",
            "(NxA)=(321270, 800172)\nhttp://www.diag.uniroma1.it/~challenge9/data/USA-road-d/USA-road-d.BAY.gr.gz",
            "(NxA)=(435666, 1057066)\nhttp://www.diag.uniroma1.it/~challenge9/data/USA-road-d/USA-road-d.COL.gr.gz",
            "(NxA)=(1070376, 2712798)\nhttp://www.diag.uniroma1.it/~challenge9/data/USA-road-d/USA-road-d.FLA.gr.gz",
            "(NxA)=(2758119, 6885658)\nhttp://www.diag.uniroma1.it/~challenge9/data/USA-road-d/USA-road-d.LKS.gr.gz",
            "(NxA)=(6262104, 15248146)\nhttp://www.diag.uniroma1.it/~challenge9/data/USA-road-d/USA-road-d.W.gr.gz",
            "(NxA)=(23947347, 58333344)\nhttp://www.diag.uniroma1.it/~challenge9/data/USA-road-d/USA-road-d.USA.gr.gz",
        ];

        cli::print_subheader(1, "Dimacs-9th-Challenge Instances");
        cli::print_table_get_choices("Dimacs9th instance", &available_levels, definitions, 0)
    }
    pub fn to_cell_string(self) -> String {
        format!("Dimacs9th:\n* name: {:?}", self)
    }

    // fs
    fn data_folder() -> PathBuf {
        DATA_FOLDER.into()
    }
    fn data_folder_temp() -> PathBuf {
        let dir = Self::data_folder().join("temp");
        utils::fs::create_dir_if_absent(dir.as_path());
        dir
    }
    fn path_zip_file(&self) -> PathBuf {
        let folder = Self::data_folder();
        let zip_filename = format!("{}.gr.zip", self.filename());
        folder.join(zip_filename)
    }
    fn filename(&self) -> &'static str {
        match self {
            Self::USARoaddNY => "USA-road-d.NY",
            Self::USARoaddBAY => "USA-road-d.BAY",
            Self::USARoaddCOL => "USA-road-d.COL",
            Self::USARoaddFLA => "USA-road-d.FLA",
            Self::USARoaddLKS => "USA-road-d.LKS",
            Self::USARoaddW => "USA-road-d.W",
            Self::USARoaddUSA => "USA-road-d.USA",
        }
    }
    fn path_extracted_file(&self) -> PathBuf {
        Self::data_folder_temp().join(format!("{}.gr", self.filename()))
    }
    fn extract_and_read_lines(
        &self,
    ) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
        #[allow(clippy::unused_io_amount)]
        fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
            use std::io::Read;

            let mut f = std::fs::File::open(filename).expect("no file found");
            let metadata = std::fs::metadata(filename).expect("unable to read metadata");
            let mut buffer = vec![0; metadata.len() as usize];
            f.read(&mut buffer).expect("buffer overflow");
            buffer
        }

        if !self.path_extracted_file().exists() {
            let temp_dir = Self::data_folder_temp();
            let path_zip = self.path_zip_file();
            let bytes = get_file_as_byte_vec(path_zip.to_str().unwrap());
            zip_extract::extract(std::io::Cursor::new(bytes), temp_dir.as_path(), true)
                .expect("failed to extract the compressed data file");
        }

        let file = std::fs::File::open(self.path_extracted_file())?;
        Ok(std::io::BufReader::new(file).lines())
    }

    // graph ctor
    pub fn create_graph_builder<B: SpGraphBuilder>(&self) -> B {
        let mut maybe_g: Option<B> = None;

        let lines = self.extract_and_read_lines().expect("failed to lines");
        for line in lines.flatten() {
            let parts: Vec<_> = line.split(' ').collect();
            match parts.first() {
                Some(&"p") => {
                    let num_nodes: usize = parts[2].parse().expect("invalid num-nodes");
                    let num_edges: usize = parts[3].parse().expect("invalid num-edges");
                    let mut gr = B::new(Some(num_nodes), Some(num_edges));
                    for i in 0..num_nodes {
                        gr.add_node(i, None);
                    }
                    maybe_g = Some(gr);
                }
                Some(&"a") => {
                    let tail: usize = parts[1].parse().expect("invalid tail id");
                    let head: usize = parts[2].parse().expect("invalid head id");
                    let weight: Weight = parts[3].parse().expect("invalid weight");
                    let g = maybe_g.as_mut().expect("missing graph meta data");
                    g.add_edge(tail - 1, head - 1, weight);
                }
                _ => {}
            }
        }

        maybe_g.expect("missing graph meta data")
    }
}
