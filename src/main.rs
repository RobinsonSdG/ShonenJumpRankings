//add the modules
mod api; 
mod models;
mod repository;

#[macro_use] extern crate rocket;

//add imports below

use api::ranking_api::{browse_and_add_rankings, get_ranking, get_rankings, get_ranking_for_a_title};
use repository::mongodb_repo::MongoRepo;


#[get("/")]
pub fn doc() -> &'static str {
    "Hello here is the jump ranking api!\nTo request for rankings of a year go to /rankings/<year>\nIf you want a ranking of a week go to /ranking/<year>/<week>"
}

#[get("/test")]
pub fn test() -> &'static str {
    "Test pour voir si c'est mongo le problème"
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    // Chemin vers les fichiers du certificat et de la clé privée
    let cert_path = "chemin/vers/cert.pem";
    let key_path = "chemin/vers/key.pem";

    // Configuration TLS/SSL pour Rocket avec figment
    let figment = rocket::Config::figment()
        .merge(("port", 8080))
        .merge(("address", "0.0.0.0"))
        .merge(("tls.certificate", cert_path))
        .merge(("tls.key", key_path));
        
    rocket::custom(figment)
        .manage(db)
        .mount("/", routes![doc])
        .mount("/", routes![test])
        .mount("/", routes![browse_and_add_rankings])
        .mount("/", routes![get_ranking])
        .mount("/", routes![get_ranking_for_a_title])
        .mount("/", routes![get_rankings])
}