use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};

pub struct AppState{
    pub count: Arc<Mutex<u32>>
}

pub async fn get_count(state: web::Data<AppState>) -> impl Responder{
    let count = state.count.lock().unwrap();
    HttpResponse::Ok().body(format!("Current count: {}", count))
}

pub async  fn increment(state: web::Data<AppState>) -> impl Responder{
    let mut count = state.count.lock().unwrap();
    *count += 1;
    HttpResponse::Ok().body(format!("Counter Incremented: {}", count))
}