#[macro_use]
extern crate rocket;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

pub mod base;
pub mod components;
pub mod error_handler;
pub mod schema;
pub mod utils;

pub use base::*;
pub use components::*;
pub use error_handler::*;
pub use schema::*;
pub use utils::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
