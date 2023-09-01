use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ranking {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub week: String,
    pub ranking: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rankings {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub year: i32,
    pub rankings: Vec<Ranking>,
}