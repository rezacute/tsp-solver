use polars::{prelude::*, export::arrow::array::{new_empty_array, new_null_array}};
use itertools::Itertools;
use time;
use travelling_salesman;
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
        let out= df.select(["lat","lng"]).unwrap();
        let mut v = Vec::<(f64,f64)>::new();
        
        let ndarray = out.to_ndarray::<Float64Type>().unwrap();
        let ln = ndarray.rows().into_iter().len() -1;
        

        for i in 0..ln{
            let x = ndarray.row(i);
            let dat :(f64,f64)=(x[0],x[1]);
            v.push(dat);
            
        }
        println!("{:?}",v);
        let tour = travelling_salesman::simulated_annealing::solve(&v,time::Duration::seconds(1));
        println!("Tour distance: {}, route: {:?}", tour.distance, tour.route);
        
    }
}

fn module_private_function() {

}