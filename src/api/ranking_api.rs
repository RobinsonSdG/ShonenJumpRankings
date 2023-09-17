use crate::{models::ranking_model::{Ranking, Rankings, Rank}, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use reqwest::Response;
use rocket::{http::Status, serde::json::Json, State};

// #[post("/ranking", data = "<new_ranking>")]
// pub fn create_ranking(
//     db: &State<MongoRepo>,
//     new_ranking: Json<Ranking>,
// ) -> Result<Json<InsertOneResult>, Status> {
//     let data = Ranking {
//         id: None,
//         week: new_ranking.week.to_owned(),
//         ranking: new_ranking.ranking.to_owned(),
//     };
//     let ranking_detail = db.create_ranking(data);
//     match ranking_detail {
//         Ok(ranking) => Ok(Json(ranking)),
//         Err(_) => Err(Status::InternalServerError),
//     }
// }

#[get("/ranking/<year>/<week>")]
pub fn get_ranking(db: &State<MongoRepo>, year: i32, week: String) -> Result<Json<Ranking>, Status> {
    let ranking_detail = db.get_ranking(&year, &week);
    match ranking_detail {
        Ok(ranking) => Ok(Json(ranking)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/ranking/<year>/<week>/<title>")]
pub fn get_ranking_for_a_title(db: &State<MongoRepo>, year: i32, week: String, title: String) -> Result<String, Status> {
    let ranking_detail = db.get_ranking_for_a_title(&year, &week, &title);
    match ranking_detail {
        Ok(rank) => Ok(rank),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/rankings/<year>")]
pub fn get_rankings(db: &State<MongoRepo>, year: i32) -> Result<Json<Rankings>, Status> {
    let ranking_detail = db.get_rankings(&year);
    match ranking_detail {
        Ok(rankings) => Ok(Json(rankings)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/rankings/<year>")]
pub async fn browse_and_add_rankings(db: &State<MongoRepo>, year: i32) -> Result<Json<InsertOneResult>, Status> {
    let mut rankings: Vec<Ranking> = vec![];
    let mut weeks = 1..53;
    while let Some(week) = weeks.next() {
        let mut week_string = week.to_string();
        let mut cover: String = "".to_string();

        // let img_resp = get_img_resp().await;

        // match img_resp.status().is_success() {
        //     true => {
        //         let img_bytes = img_resp.bytes().await.unwrap();
        //         let img = image::load_from_memory(&img_bytes).unwrap();
        //         img.save("test.png");
        //         panic!("")
        //     }

        //     false => {
        //         println!("error");
        //     }
        // }

        // get all weeks of the year
        let mut resp = get_resp(&week_string, year).await;
        let body = match resp.status().is_success() {
            true => resp.text().await.unwrap(),
            false => {
                // try a double week
                week_string = format!("{}-{}", &week_string, (week + 1).to_string());
                resp = get_resp(&week_string, year).await;
                match resp.status().is_success() {
                    true => {
                        weeks.next();
                        resp.text().await.unwrap()
                    }

                    false => {
                        println!("error while getting url on year {} and week {}, status code: {}", year, &week_string, resp.status());
                        "".to_string()
                    }
                }
            }
        };
        if body == "" {
            continue;
        }

        let document = scraper::Html::parse_document(&body);

        let figure_selector = scraper::Selector::parse("figure>a").unwrap();
        let li_selector = scraper::Selector::parse("ol>li").unwrap();
        let font_color_selector = scraper::Selector::parse("font").unwrap();
        let a_selector = scraper::Selector::parse("a").unwrap();

        for figure in document.select(&figure_selector) {
            cover = match figure.value().attr("href") {
                Some(url) => {
                    if let Some(pos) = url.find(".png") {
                        // Extraire la sous-chaîne jusqu'à ".png" (inclus)
                        let result = &url[..=pos + 3];
                        result.to_string()
                    } else {
                        url.to_string()
                    }
                },
                None => panic!("no cover found")
            };
        }

        let mut placements: Vec<Rank> = vec![];
        for li in document.select(&li_selector) {
            let mut is_color = false;
            for _ in li.select(&font_color_selector).map(|x| x.inner_html()) {
                is_color = true;
                break
            }
            if !is_color {
                let chapters: Vec<&str> = li.children().filter_map(|node| match node.value() {
                    scraper::Node::Text(text) => Some(&text[..]),
                    _ => None,
                })
                .collect();
                let chapter_string: String = chapters[0].chars().filter(|c| c.is_digit(10)).collect();
                let chapter = match chapter_string.parse::<i16>() {
                    Ok(c) => c,
                    Err(e) => {
                        println!("error on {}: {}", chapters[0], e);
                        0
                    }
                };
                for element in li.select(&a_selector).map(|x| x.inner_html()) {
                    let rank = Rank {
                        id: None,
                        name: element,
                        chapter: chapter
                    };
                    placements.push(rank);
                }
            }
        }
        let ranking = Ranking {
            id: None,
            week: week_string,
            ranking: placements,
            cover: cover,
        };
        rankings.push(ranking);
    }
    let data = Rankings {
        id: None,
        year: year,
        rankings: rankings,
    };
    let rankings_detail = db.create_rankings(data);
    match rankings_detail {
        Ok(rankings) => Ok(Json(rankings)),
        Err(_) => Err(Status::InternalServerError),
    }


    // Some(Json(Rankings {
    //     id: None,
    //     year: year,
    //     rankings: rankings,
    // }))
}

async fn get_resp(week: &String, year: i32) -> Response {
    let url = format!("https://jump.fandom.com/wiki/Weekly_Shonen_Jump_Issue_{},_{}", week, year);
    let resp = match reqwest::get(url).await {
        Ok(v) => v,
        Err(e) => panic!("error while getting url on year {} and week {}: {}", year, week, e)
    };
    resp
}

async fn get_img_resp() -> Response {
    let img_resp = match reqwest::get("https://static.wikia.nocookie.net/weeky-shonen-jump/images/9/92/WSJ_Issue_2022_52_Cover.png").await {
        Ok(v) => v,
        Err(e) => panic!("")
    };
    img_resp
}