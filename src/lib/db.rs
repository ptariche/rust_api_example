use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URI")
    .expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}
