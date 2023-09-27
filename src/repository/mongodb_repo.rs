use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::doc,
    results::UpdateResult,
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

    pub fn get_ranking_for_a_title(&self, year: &i32, week: &String, title: &String) -> Result<String, &'static str> {
        let filter = doc! {"year": year};
        let mut ranking_result: Option<usize> = None;
        let rankings = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting ranking's detail").unwrap();
        for ranking in rankings.rankings {
            if &ranking.week == week {
                match ranking.ranking.iter().position(|manga| &manga.name == title) {
                    Some(m) => ranking_result = Some(m + 1),
                    None => break
                }
            }
        }
        match ranking_result {
            Some(r) => Ok(r.to_string()),
            None => Err("no ranking for given parameters")
        }
    }

    pub fn create_rankings(&self, new_ranking: Rankings) -> Result<UpdateResult, mongodb::error::Error> {
        let new_doc = Rankings {
            id: None,
            year: new_ranking.year,
            rankings: new_ranking.rankings,
        };
        let mut options = mongodb::options::ReplaceOptions::default();
        options.upsert = Some(true);

        self
            .col
            .replace_one(doc!{"year":new_ranking.year}, new_doc, options)
    }

    pub fn get_rankings(&self, year: &i32) -> Result<Option<Rankings>, mongodb::error::Error> {
        let filter = doc! {"year": year};
        self
            .col
            .find_one(filter, None)
    }
}
