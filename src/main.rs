use actix_web::{web, App, HttpResponse, HttpServer, Result};

async fn hello() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Learning Actix"))
}

async fn kkrh() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("How actix works"))
}

async fn about() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))        
            .route("/kkrh", web::get().to(kkrh))
            .route("/about", web::get().to(about))     
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}