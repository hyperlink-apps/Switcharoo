use actix_web::{web, App, HttpServer};
use moka::future::Cache;

pub mod models;
pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename("switcharoo.env").ok();
    let config = crate::models::config::Config::get();

    let cache: Cache<String, i64> = Cache::new(1000);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(cache.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(routes::user::get_scope())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
