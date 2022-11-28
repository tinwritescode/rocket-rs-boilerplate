#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{mount_endpoints_and_merged_docs, openapi, openapi_get_routes_spec};
use rust_backend::{base::BaseResponse, components, error_handler};

#[openapi(tag = "home")]
#[get("/")]
fn index() -> BaseResponse<&'static str> {
    Ok(Json("Hello, world!"))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let mut rocket = rocket::build()
        .mount(
            "/swagger/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../api/v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("My special documentation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../api/v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .mount("/", rocket::fs::FileServer::from("public"))
        .register(
            "/",
            catchers![
                error_handler::not_found,
                error_handler::internal_error,
                error_handler::unprocessable_entity,
                error_handler::default
            ],
        );

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/api/v1".to_owned(), openapi_settings,
        "" => openapi_get_routes_spec![index],
        "/auth" => components::auth::routes(),
        "/posts" => components::posts::routes(),
        "/admin" => components::admin::routes(),
    };

    rocket
}
