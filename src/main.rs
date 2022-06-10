mod errors;
mod handlers;
mod helpers;
mod models;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

struct DbState {
    db_pool: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "full");
    env::set_var("TEST", "1");
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("TESTING");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec![
                "GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "PATCH",
            ])
            .allowed_headers(vec![
                "authorization",
                "Origin",
                "X-Requested-With",
                "Content-Type",
                "Accept",
                "X-Access-Token",
                "x-api-key",
            ])
            .send_wildcard()
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(DbState {
                db_pool: pool.clone(),
            }))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(handlers::get_pizzas)
            .service(handlers::put_pizza)
            .service(handlers::get_order)
            .service(handlers::put_order)
            .service(handlers::get_price)
    })
    .bind(("127.0.0.1", 8095))?
    .run()
    .await
}
