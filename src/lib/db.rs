use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;

const DEFAULT_DATABASE_URI: &'static str = "postgresql://127.0.0.1:5432/postgres";

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = match env::var("DATABASE_URI") {
    Ok(value) => value,
    Err(_) => DEFAULT_DATABASE_URI.to_string(),
  };

  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}
