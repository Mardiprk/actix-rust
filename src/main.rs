use actix_web::{web::{self, get}, App, HttpResponse, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct GreetRequest{
    name: String
}

async fn hello() -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().body("Hello, Actix"))
}

async fn about(name: web::Path<String>) -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().body(format!("{} wants to know about Actix", name)))
}

async fn greet(req: web::Json<GreetRequest>) -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().body(format!("Hello! {}", req.name)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Running server @ http://localhost:8080");

    HttpServer::new(||{
        App::new()
        .route("/", get().to(hello))
        .route("/about/{name}", get().to(about))
        .route("/greet", web::post().to(greet))
    }).bind("127.0.0.1:8080")?
    .run()
    .await
}