use actix_web::{web, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use crate::tsp::tsp_solver;

#[get("")]
async fn index() -> HttpResponse {
    let rs = &tsp_solver::get_dataset().lock().unwrap().points;
    HttpResponse::Ok().json(web::Json(rs))
}


pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
}