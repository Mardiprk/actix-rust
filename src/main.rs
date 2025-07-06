use std::fmt::format;

use actix_web::{web::{self, get, post}, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GreetRequest{
    name: String
}

#[derive(Deserialize, Serialize)]
struct GreetResponse{
    message: String
}
async fn hello() -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().body("Hello, Actix"))
}

async fn about(name: web::Path<String>) -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().body(format!("{} wants to know about Actix", name)))
}

async  fn greet(info: web::Json<GreetRequest>) -> Result<HttpResponse>{
    let res = GreetResponse{
        message: format!("Hello, {}!", info.name),
    };
    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Running server @ http://localhost:8080");

    HttpServer::new(||{
        App::new()
        .route("/", get().to(hello))
        .route("/about/{name}", get().to(about))
        .route("/greet", post().to(greet))
    }).bind("127.0.0.1:8080")?
    .run()
    .await
}