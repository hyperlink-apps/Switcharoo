use crate::models::user::User;

use super::structs::CreateUser;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/create")]
pub async fn create_user(
    user: web::Json<CreateUser>,
    pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<impl Responder, actix_web::Error> {
    let valid = user.validate(&pool).await.map_err(|e| {
        eprintln!("{}", e);
        actix_web::error::ErrorInternalServerError("Error validating user.")
    })?;

    if let Some(message) = valid {
        return Err(actix_web::error::ErrorBadRequest(message));
    }

    let _ = User::create(&user.email, &user.password, &pool).await;

    return Ok(HttpResponse::Ok().body("User created."));
}
