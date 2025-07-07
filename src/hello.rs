use actix_web::{ web, App, HttpResponse, HttpServer, Responder};

pub async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Learn Actix")
}