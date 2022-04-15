use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JikanResponse<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JikanAPIError {
    pub status: u32,
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
    pub error: String,
}

pub mod anime {
    use self::time::Aired;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Anime {
        pub mal_id: u64,
        pub url: String,
        pub images: ImageList,
        pub trailer: Trailer,
        pub title: String,
        pub title_english: Option<String>,
        pub title_japanese: Option<String>,
        pub title_synonyms: Vec<String>,
        #[serde(rename = "type")]
        pub data_type: String,
        pub source: Option<String>,
        pub episodes: Option<u64>,
        pub status: AirStatus,
        pub airing: bool,
        pub aired: Aired,
        pub duration: Option<String>,
        pub rating: Option<String>,
        pub score: Option<f64>,
        pub scored_by: Option<u64>,
        pub rank: Option<u64>,
        pub popularity: Option<u64>,
        pub members: Option<u64>,
        pub favorites: Option<u64>,
        pub synopsis: Option<String>,
        pub background: Option<String>,
        pub season: Option<Season>,
        pub year: Option<u64>,
        pub broadcast: Broadcast,
        pub producers: Vec<MalUrl>,
        pub licensors: Vec<MalUrl>,
        pub studios: Vec<MalUrl>,
        pub genres: Vec<MalUrl>,
        pub explicit_genres: Vec<MalUrl>,
        pub themes: Vec<MalUrl>,
        pub demographics: Vec<MalUrl>,
    }

    #[cfg(not(feature = "chrono"))]
    pub mod time {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Aired {
            pub from: Option<String>,
            pub to: Option<String>,
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
            pub day: Option<i64>,
            pub month: Option<i64>,
            pub year: Option<i64>,
        }
    }

    #[cfg(feature = "chrono")]
    pub mod time {
        use chrono::Utc;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Aired {
            from: Option<chrono::DateTime<Utc>>,
            to: Option<chrono::DateTime<Utc>>,
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Broadcast {
        pub day: Option<String>,
        pub time: Option<String>,
        pub timezone: Option<String>,
        pub string: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MalUrl {
        pub mal_id: i64,
        #[serde(rename = "type")]
        pub url_type: String,
        pub name: String,
        pub url: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Images {
        pub jpg: ImageList,
        pub webp: ImageList,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImageList {
        pub image_url: Option<String>,
        pub small_image_url: Option<String>,
        pub medium_image_url: Option<String>,
        pub large_image_url: Option<String>,
        pub maximum_image_url: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Trailer {
        pub youtube_id: Option<String>,
        pub url: Option<String>,
        pub embed_url: Option<String>,
        pub images: Option<ImageList>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum AirStatus {
        #[serde(rename = "Finished Airing")]
        FinishedAiring,
        #[serde(rename = "Currently Airing")]
        CurrentlyAiring,
        #[serde(rename = "Not Yet Aired")]
        NotYetAired,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Season {
        #[serde(rename = "summer")]
        Summer,
        #[serde(rename = "winter")]
        Winter,
        #[serde(rename = "spring")]
        Spring,
        #[serde(rename = "fall")]
        Fall,
    }

    pub type Characters = Vec<Character>;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Character {
        pub character: CharacterDetails,
        pub role: String,
        pub voice_actors: Vec<VoiceActor>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CharacterDetails {
        pub mal_id: u32,
        pub url: String,
        pub images: CharacterImages,
        pub name: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CharacterImages {
        pub jpg: CharacterImage,
        pub webp: CharacterImage,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CharacterImage {
        pub image_url: Option<String>,
        pub small_image_url: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct VoiceActor {
        person: Person,
        language: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Person {
        mal_id: u32,
        url: String,
        images: PersonImages,
        name: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PersonImages {
        jpg: PersonImage,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PersonImage {
        image_url: Option<String>,
    }
}
