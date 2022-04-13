use std::{path::Path, time::Duration};

use jikan::{models::JikanAPIError, JikanClient, JikanError, JikanResult};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> JikanResult<()> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter("jikan=trace")
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut id = 1;

    let path = Path::new("anime");
    if path.is_dir() {
        for entry in path.read_dir().unwrap() {
            if let Ok(item) = entry {
                let item_id: u32 = item
                    .file_name()
                    .to_string_lossy()
                    .trim_end_matches(".json")
                    .parse()
                    .unwrap();

                if item_id > id {
                    id = item_id;
                }
            }
        }
    }

    if !path.exists() {
        std::fs::create_dir(path).unwrap();
    }

    loop {
        let url = std::env::var("JIKAN_API_URL").unwrap_or(jikan::DEFAULT_API_URL.into());

        let client = JikanClient::default().with_api_url(&url);

        let delay = if url.to_ascii_lowercase() == "https://api.jikan.moe/v4" {
            Duration::from_secs(4)
        } else {
            Duration::ZERO
        };

        println!("Fetching {id}");
        let anime = client.get_anime_by_id(id).await;
        match anime {
            Ok(anime) => {
                println!("{:#?}", anime);
                std::fs::write(
                    format!("anime/{}.json", anime.mal_id),
                    serde_json::to_vec(&anime)?,
                )
                .ok();
            }

            Err(JikanError::API {
                0: JikanAPIError { status: 404, .. },
            }) => {
                println!("{id}: 404");
            }
            Err(err) => Result::Err(err).unwrap(),
        }
        if !delay.is_zero() {
            println!("Waiting for {:?}...", delay);
            tokio::time::sleep(delay).await;
        }
        id += 1;
    }
}
