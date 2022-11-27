mod base;
mod components;
mod error_handler;
mod schema;
mod utils;

#[macro_use]
extern crate rocket;

use base::model::BaseResponse;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use rocket::serde::json::Json;

#[get("/")]
fn index() -> BaseResponse<&'static str> {
    Ok(Json("Hello, world!"))
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
        .mount("/", rocket::fs::FileServer::from("public"))
        .mount("/", routes![index])
        .mount("/api/v1/auth", components::auth::routes())
        .mount("/admin", components::admin::routes())
        .mount("/api/v1/posts", components::posts::routes())
        .register(
            "/",
            catchers![error_handler::not_found, error_handler::internal_error],
        );

    rocket
}
