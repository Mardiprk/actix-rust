mod hello;
mod about;
mod greet;
mod echo;
mod counter;

use std::sync::{Mutex, Arc};

use actix_web::{ web, App, HttpServer};
use web::{get, post, ServiceConfig};
use hello::hello;
use about::about;
use greet::greet;
use echo::echo;
use counter::{AppState, get_count, increment};

fn config(cfg: &mut ServiceConfig){
    cfg
    .route("/", get().to(hello))
    .route("/about/{name}", get().to(about))
    .route("/greet", get().to(greet))
    .route("/echo", post().to(echo))
    .route("/counter", get().to(get_count))
    .route("/counter", post().to(increment));
}

#[actix_web::main]
async  fn main() -> std::io::Result<()>{
    println!("🚀 Server running @ http://localhost:8080");

    let shared_data = web::Data::new(AppState{
        count: Arc::new(Mutex::new(0)),
    });

    HttpServer::new( move||{
        App::new()
        .app_data(shared_data.clone())
        .configure(config)
    }).bind(("127.0.0.1", 8080))?.run().await
}