use actix_web::{web::{self, get}, App, HttpResponse, HttpServer, Result};

async fn hello() -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().body("Learning Actix"))
}

async  fn about(name: web::Path<String>) -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().body(format!("{} wants to know about Actix", name)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Starting Running in http://localhost:8080");

    HttpServer::new(||{
        App::new()
        .route("/", get().to(hello))
        .route("/about/{name}", get().to(about))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}