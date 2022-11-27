#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{
    mount_endpoints_and_merged_docs, openapi, openapi_get_routes, openapi_get_routes_spec,
};
use rust_backend::{base::model::BaseResponse, components, error_handler};

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
                url: "../v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("My special documentation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
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
            catchers![error_handler::not_found, error_handler::internal_error],
        );

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/v1".to_owned(), openapi_settings,
        "" => openapi_get_routes_spec![index],
        "/api/v1/auth" => components::auth::routes(),
        "/api/v1/posts" => components::posts::routes(),
        "/admin" => components::admin::routes(),
    };

    rocket
}
