use actix_web::{post, web, HttpResponse, Responder};

#[post("/verify")]
pub async fn verify() -> impl Responder {
    HttpResponse::Ok().body("User verified")
}
