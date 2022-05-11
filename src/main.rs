use actix_web::{
    get, post,
    web::{self, Json},
    App, HttpServer, Responder,
};
use chrono::NaiveDateTime;
use dotenv::dotenv;
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use sqlx_pg_migrate::migrate;
use std::env;

static MIGRATIONS: Dir = include_dir!("migrations");

struct DbState {
    db_pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pizza {
    id: i32,
    name: Option<String>,
    description: Option<String>,
    price_inr: Option<i32>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PizzaRequest {
    name: String,
    description: String,
    price_inr: i32,
}

#[get("/pizza")]
async fn get_pizzas(db_conn: web::Data<DbState>) -> impl Responder {
    let pool = db_conn.db_pool.clone();
    let pizzas = sqlx::query_as!(
        Pizza,
        "
    SELECT id, name, description, price_inr,created_at,updated_at,deleted_at from pizza"
    )
    .fetch_all(&pool)
    .await
    .expect("HELLO");
    Json(pizzas)
}

#[post("/pizza")]
async fn put_pizza(
    db_conn: web::Data<DbState>,
    pizza_req: web::Json<PizzaRequest>,
) -> impl Responder {
    let pool = db_conn.db_pool.clone();
    let pizza = sqlx::query_as!(
        Pizza,
        "INSERT INTO pizza (name,description,price_inr,created_at,updated_at) VALUES 
        ($1,$2,$3,now(),now()) returning id,name,description,price_inr,created_at,updated_at,deleted_at
        ",pizza_req.name,pizza_req.description,pizza_req.price_inr)
        .fetch_one(&pool)
        .await
        .expect("Please don't fail :(");
    Json(pizza)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "full");
    env::set_var("TEST", "1");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("TESTING");

    migrate(&database_url, &MIGRATIONS)
        .await
        .expect("DATABASE CONNECTION FAILED");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(DbState {
                db_pool: pool.clone(),
            }))
            .service(get_pizzas)
            .service(put_pizza)
    })
    .bind(("127.0.0.1", 8095))?
    .run()
    .await
}
