use super::{Hero, NewHero};
use crate::schema::heroes;
use diesel::RunQueryDsl;
use rocket::{
    response::{Flash, Redirect},
    serde::json::Json,
};
use serde::Serialize;

#[derive(Serialize)]
struct Employee {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
}

#[get("/")]
fn find_all() -> Result<Json<Vec<Hero>>, Flash<Redirect>> {
    let new_hero = NewHero {
        fantasy_name: &"test".to_string(),
        real_name: Some("test".clone()),
        spotted_photo: "test".to_string(),
        strength_level: 20,
    };

    let insert = diesel::insert_into(heroes::table)
        .values(&new_hero)
        .execute(&mut crate::establish_connection());

    if let Err(err_msg) = insert {
        return Err(Flash::error(Redirect::to("/"), err_msg.to_string()));
    }

    let results = heroes::table
        .load::<Hero>(&mut crate::establish_connection())
        .expect("Error loading heroes");

    Ok(Json(results))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![find_all]
}
