//add the modules
mod api; 
mod models;
mod repository;

#[macro_use] extern crate rocket;

//add imports below

use api::ranking_api::{browse_and_add_rankings, get_ranking, get_rankings, get_ranking_for_a_title};
use repository::mongodb_repo::MongoRepo;

use reqwest::Response;

use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![browse_and_add_rankings])
        .mount("/", routes![get_ranking])
        .mount("/", routes![get_ranking_for_a_title])
        .mount("/", routes![get_rankings])
}