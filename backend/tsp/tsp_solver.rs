


use polars::{prelude::*};
use itertools::Itertools;
use time;
use travelling_salesman;
use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};
use ndarray::Array2;
use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use snailquote::unescape;
#[cfg(test)]
#[path = "tsp_solver_tests.rs"]
mod tsp_solver_tests;
pub type DistanceMatrix = Vec<Vec<f64>>;

pub type Route = Vec<usize>;


#[derive(Debug, Serialize, Deserialize)]
#[tsync::tsync]
pub struct GeoPoint{
    pub lat: f64,
    pub lng: f64,
    pub label: String
}

pub struct OSRMNodes{
    pub points: Vec<GeoPoint>,
    pub distance_matrix: DistanceMatrix
}
#[derive(Debug)]
pub struct Data {
    pub label: String,
    pub v: Vec<(f64,f64)>,
    pub points: Vec<GeoPoint>,
    pub dmarray: Array2::<f64>
}
pub fn solve_tsp()->(Vec<usize>,f64){
    let data = get_dataset().lock().unwrap();
    let ln = data.v.len()-1;
    let tour = travelling_salesman::simulated_annealing::solve(&data.v,time::Duration::seconds(5));
        let mut res_dist:f64= 0.0;
        let mut route = tour.route.clone();
        route.pop();
        let idx=route.iter().find_position(|&&x| x == 0).unwrap().0;
        let mut vec = route.split_off(idx);

        vec.append(&mut route);
        
        for i in 1..ln{
            let j = tour.route[i-1];
            let k = tour.route[i];
            let a =*data.dmarray.row(j).to_vec().get(k).unwrap();
            res_dist+=a;
            
        }

        let j = tour.route[ln];
        let k = tour.route[0];
        let a =*data.dmarray.row(j).to_vec().get(k).unwrap();
        
        res_dist+=a;
        (vec,res_dist)
}
pub fn get_dataset() -> &'static Mutex<Data> {
    static mut DATA: MaybeUninit<Mutex<Data>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        let df = CsvReader::from_path("backend/data/data-1.csv").unwrap().finish().unwrap();
        let dm = CsvReader::from_path("backend/data/osrm-distance-matrix.csv").unwrap().finish().unwrap();
        let out= df.select(["lat","lng"]).unwrap();
        let binding = df.select(["POINT"]).unwrap();
        let labels= binding.select_at_idx(0);
        let mut v = Vec::<(f64,f64)>::new();
        let mut points: Vec<GeoPoint> = Vec::<GeoPoint>::new();
        
        let ndarray = out.to_ndarray::<Float64Type>().unwrap();
        let dmarray = dm.to_ndarray::<Float64Type>().unwrap();
        

        let ln = ndarray.rows().into_iter().len() -1;
        
        for i in 0..ln{
            let x = ndarray.row(i);
            let dat :(f64,f64)=(x[0],x[1]);
            v.push(dat);
            let s = labels.unwrap().0.get(i).to_string();
            points.push(GeoPoint{lat:x[0],lng:x[1], label:unescape(&s).unwrap()});
            
            
        }
        DATA.as_mut_ptr().write(Mutex::new(Data {
            label: "test data".to_string(),
            v:v,
            points:points,
            dmarray:dmarray
        }));

    });

    unsafe { &*DATA.as_ptr() }
}



