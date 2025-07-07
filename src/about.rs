use actix_web::{ web, HttpResponse, Responder};

pub async fn about(name: web::Path<String>) -> impl Responder{
    let response = format!("{}, wants to learn actix", name);
    HttpResponse::Ok().body(response)
}