use dotenv::dotenv;
use include_dir::{include_dir, Dir};
use sqlx_pg_migrate::migrate;
use std::env;
use std::error;

static MIGRATIONS: Dir = include_dir!("migrations");

#[actix_web::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    migrate(&database_url, &MIGRATIONS)
        .await
        .expect("DATABASE CONNECTION FAILED");
    println!("Finished migrating");
    Ok(())
}
