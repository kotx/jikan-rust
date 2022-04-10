use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JikanResponse<T> {
    pub data: T,
}

pub mod anime {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Anime {
        pub mal_id: i64,
        pub url: String,
        pub images: HashMap<String, Image>,
        pub trailer: Trailer,
        pub title: String,
        pub title_english: String,
        pub title_japanese: String,
        pub title_synonyms: Vec<Option<serde_json::Value>>,
        #[serde(rename = "type")]
        pub data_type: String,
        pub source: String,
        pub episodes: i64,
        pub status: String,
        pub airing: bool,
        pub aired: Aired,
        pub duration: String,
        pub rating: String,
        pub score: f64,
        pub scored_by: i64,
        pub rank: i64,
        pub popularity: i64,
        pub members: i64,
        pub favorites: i64,
        pub synopsis: String,
        pub background: String,
        pub season: String,
        pub year: i64,
        pub broadcast: Broadcast,
        pub producers: Vec<Genre>,
        pub licensors: Vec<Genre>,
        pub studios: Vec<Genre>,
        pub genres: Vec<Genre>,
        pub explicit_genres: Vec<Option<serde_json::Value>>,
        pub themes: Vec<Genre>,
        pub demographics: Vec<Option<serde_json::Value>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Aired {
        pub from: String,
        pub to: String,
        pub prop: Prop,
        pub string: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Prop {
        pub from: From,
        pub to: From,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct From {
        pub day: i64,
        pub month: i64,
        pub year: i64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Broadcast {
        pub day: String,
        pub time: String,
        pub timezone: String,
        pub string: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Genre {
        pub mal_id: i64,
        #[serde(rename = "type")]
        pub genre_type: String,
        pub name: String,
        pub url: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Image {
        pub image_url: String,
        pub small_image_url: String,
        pub large_image_url: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Trailer {
        pub youtube_id: String,
        pub url: String,
        pub embed_url: String,
        pub images: Images,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Images {
        pub image_url: String,
        pub small_image_url: String,
        pub medium_image_url: String,
        pub large_image_url: String,
        pub maximum_image_url: String,
    }
}
