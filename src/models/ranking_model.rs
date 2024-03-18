use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ranking {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub week: String,
    pub ranking: Vec<Rank>,
    pub newbies: Vec<Rank>,
    pub absent: Vec<Rank>,
    pub cover: Figure,
    pub color_pages: Vec<Figure>,
    pub preview_pages: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rank {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub chapter: i16
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Figure {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub imgs: Vec<String>,
    pub rank: Rank
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rankings {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub year: i32,
    pub rankings: Vec<Ranking>,
}