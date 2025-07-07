mod hello;
mod about;
mod greet;

use actix_web::{ web, App, HttpServer};
use web::{get, ServiceConfig};
use hello::hello;
use about::about;
use greet::greet;

fn config(cfg: &mut ServiceConfig){
    cfg
    .route("/", get().to(hello))
    .route("/about/{name}", get().to(about))
    .route("/greet", get().to(greet));
}

#[actix_web::main]
async  fn main() -> std::io::Result<()>{
    println!("🚀 Server running @ http://localhost:8080");

    HttpServer::new(||{
        App::new().configure(config)
    }).bind(("127.0.0.1", 8080))?.run().await
}