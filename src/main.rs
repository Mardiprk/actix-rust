mod hello;

use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
use web::{get, ServiceConfig, Path};
use hello::hello;

async fn greet(name: Path<String>) -> impl Responder{
    let response = format!("{}, wants to learn about actix", name);
    HttpResponse::Ok().body(response)
}

fn config(cfg: &mut ServiceConfig){
    cfg
    .route("/", get().to(hello))
    .route("/about/{name}", get().to(greet));
}

#[actix_web::main]
async  fn main() -> std::io::Result<()>{
    println!("🚀 Server running @ http://localhost:8080");

    HttpServer::new(||{
        App::new().configure(config)
    }).bind(("127.0.0.1", 8080))?.run().await
}