use polars::{prelude::*};
use itertools::Itertools;
use time;
use travelling_salesman;
use std::str;

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
        let dm = CsvReader::from_path("backend/data/osrm-distance-matrix.csv").unwrap().finish().unwrap();
        let out= df.select(["lat","lng"]).unwrap();
        let binding = df.select(["POINT"]).unwrap();
        let labels= binding.select_at_idx(0);
        let mut v = Vec::<(f64,f64)>::new();
        
        let ndarray = out.to_ndarray::<Float64Type>().unwrap();
        let dmarray = dm.to_ndarray::<Float64Type>().unwrap();
        

        let ln = ndarray.rows().into_iter().len() -1;
        
        for i in 0..ln{
            let x = ndarray.row(i);
            let dat :(f64,f64)=(x[0],x[1]);
            v.push(dat);
            let s = labels.unwrap().0.get(i).to_string();
            println!("{}",s);
            
            
        }
        
        let tour = travelling_salesman::simulated_annealing::solve(&v,time::Duration::seconds(5));
        let mut res_dist:f64= 0.0;
        let mut route = tour.route.clone();
        route.pop();
        let idx=route.iter().find_position(|&&x| x == 0).unwrap().0;
        let mut vec = route.split_off(idx);

        vec.append(&mut route);
        
        for i in 1..ln{
            let j = tour.route[i-1];
            let k = tour.route[i];
            let a =*dmarray.row(j).to_vec().get(k).unwrap();
            res_dist+=a;
            
        }

        let j = tour.route[ln];
        let k = tour.route[0];
        let a =*dmarray.row(j).to_vec().get(k).unwrap();
        
        res_dist+=a;

        println!("route: {:?} distance {}", vec, res_dist);
        
    }
}

