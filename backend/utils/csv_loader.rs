use polars::prelude::*;

#[cfg(test)]
#[path = "csv_loader_tests.rs"]
mod csv_loader_tests;

pub struct CsvLoader{
    file_name: String
}

impl CsvLoader{
    pub fn load_sample_file(){
        println!("Loading file");
        let df = CsvReader::from_path("backend/data/data-1.csv").unwrap().finish().unwrap();
    }
}

fn module_private_function() {

}