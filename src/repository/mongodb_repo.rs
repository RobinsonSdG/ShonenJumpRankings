use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult},
    sync::{Client, Collection},
};
use crate::models::ranking_model::{Ranking, Rankings};

pub struct MongoRepo {
    col: Collection<Rankings>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => panic!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Rankings> = db.collection("Rankings");
        MongoRepo { col }
    }

    pub fn get_ranking(&self, year: &i32, week: &String) -> Result<Ranking, &'static str> {
        let filter = doc! {"year": year};
        let mut ranking_result: Option<Ranking> = None;
        let rankings = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting ranking's detail").unwrap();
        for ranking in rankings.rankings {
            if &ranking.week == week {
                ranking_result = Some(ranking)
            }
        }
        match ranking_result {
            Some(r) => Ok(r),
            None => Err("no ranking for given parameters")
        }
    }

    pub fn create_rankings(&self, new_ranking: Rankings) -> Result<InsertOneResult, Error> {
        let new_doc = Rankings {
            id: None,
            year: new_ranking.year,
            rankings: new_ranking.rankings,
        };
        let rankings = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating ranking");
        Ok(rankings)
    }

    pub fn get_rankings(&self, year: &i32) -> Result<Rankings, Error> {
        let filter = doc! {"year": year};
        let rankings = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting ranking's detail");
        Ok(rankings.unwrap())
    }
}
