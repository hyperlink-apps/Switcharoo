use super::structs::LoginUser;
use crate::models::{session::Session, user::User};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

#[post("/login")]
pub async fn log_user_in(
    user: web::Json<LoginUser>,
    request: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    cache: web::Data<moka::future::Cache<String, i64>>,
    config: web::Data<crate::models::config::Config>,
) -> Result<impl Responder, actix_web::Error> {
    let mut expiration = None;
    let mut token = None;

    let cookie = request
        .cookie("switcharoo_session")
        .map(|cookie| cookie.value().to_string());

    if let Some(existing_token) = cookie {
        if let Some(exp) = Session::get_and_increase_expiration(&cache, &existing_token).await {
            token = Some(existing_token);
            expiration = Some(exp);
        }
    }

    if token.is_none() {
        User::check_email_password(&user.email, &user.password, &pool)
            .await
            .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

        let (new_token, new_expiration) = Session::create(&cache).await;

        token = Some(new_token);
        expiration = Some(new_expiration);
    }

    if let (Some(token), Some(expiration)) = (token, expiration) {
        let exp = actix_web::cookie::time::OffsetDateTime::from_unix_timestamp(expiration as i64)
            .map_err(|e| {
            eprintln!("Error creating expiration: {:?}", e);
            actix_web::error::ErrorInternalServerError("Error creating expiration.")
        })?;

        let cookie = actix_web::cookie::Cookie::build("switcharoo_session", token)
            .http_only(true)
            .secure(config.secure)
            .domain(config.domain.clone())
            .expires(exp)
            .finish();

        Ok(HttpResponse::Ok().cookie(cookie).body("Logged user in."))
    } else {
        return Ok(HttpResponse::Unauthorized().body("Invalid email or password."));
    }
}
