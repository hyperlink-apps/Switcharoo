use actix_web::web;

pub mod create;
pub mod login;
pub mod structs;
pub mod verify;

pub fn get_scope() -> actix_web::Scope {
    web::scope("/user").service(
        web::scope("/auth")
            .service(create::create_user)
            .service(login::log_user_in),
    )
}
