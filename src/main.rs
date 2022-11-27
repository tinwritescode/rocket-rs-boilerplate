mod base;
mod components;
mod error_handler;
mod schema;
mod services;

#[macro_use]
extern crate rocket;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1/auth", components::auth::routes())
        .mount("/admin", components::admin::routes())
        .register(
            "/",
            catchers![error_handler::not_found, error_handler::internal_error],
        );

    rocket
}
