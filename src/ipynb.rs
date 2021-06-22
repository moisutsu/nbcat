use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ipynb {
    pub cells: Vec<Cell>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cell {
    pub cell_type: String,
    pub execution_count: Option<i32>,
    pub outputs: Option<Vec<Output>>,
    pub source: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub output_type: String,
    pub data: Option<Data>,
    pub text: Option<Vec<String>>,
    pub traceback: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    #[serde(rename(serialize = "text/plain", deserialize = "text/plain"))]
    pub text_plain: Option<Vec<String>>,
    #[serde(rename(serialize = "image/png", deserialize = "image/png"))]
    pub image_png: Option<String>,
}
