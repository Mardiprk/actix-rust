use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct EchoRequest{
    name: String,
    message: String
}

#[derive(Serialize)]
pub struct EchoResponse{
    received: String,
}

pub async fn echo(body: web::Json<EchoRequest>) -> impl Responder{
    let message = body.into_inner();
    let reply = format!("{} says {}", message.name, message.message);
    HttpResponse::Ok().json(EchoResponse{
        received: reply
    })
}