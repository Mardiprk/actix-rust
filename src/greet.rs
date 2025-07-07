use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GreetParams {
    name: String,
}

pub async fn greet(query: web::Query<GreetParams>) -> impl Responder{
    let name = &query.name;
    let response = format!("Hi {}, Welcome to Actix Class.", name);
    HttpResponse::Ok().body(response)
}