use rigma::views;
use rigma::services::model_bl;
use std::fs;
use std::path;
use std::io::Read;

fn main() {
    let mut file = fs::File::open(path::Path::new("document.json")).expect("unable to open file 'document.json'");
    let mut rep = String::new();
    file.read_to_string(&mut rep).expect("unable to read json");
    let view: views::Document = serde_json::from_str(&rep).expect("unable to parse json document");
    model_bl::ModelSaver::from(&view);
}
