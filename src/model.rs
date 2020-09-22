use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ipynb {
    pub cells: Vec<Cell>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cell {
    pub cell_type: String,
    pub execution_count: Option<i32>,
    pub outputs: Vec<Output>,
    pub source: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub output_type: String,
    pub data: Option<Data>,
    pub text: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    #[serde(rename(serialize = "text/plain"))]
    pub text_plain: Option<Vec<String>>,
}

#[derive(clap::Clap)]
pub struct Opts {
    pub input: String,
}
