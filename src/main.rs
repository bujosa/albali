use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use reqwest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
    popularity: u32,
    #[serde(rename = "type")]
    type_: String,
    uri: String,
}

#[tokio::main]
async fn main() {
    get_access_token().await;
}

async fn get_access_token() {
    // Load dotenv
    dotenv::dotenv().ok();

    // Get the Spotify API token from the environment
    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();

    // Build the request
    let client = reqwest::Client::new();
    let res = client
        .post("https://accounts.spotify.com/api/token")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(
            AUTHORIZATION,
            format!(
                "Basic {}",
                BASE64.encode(&format!("{}:{}", client_id, client_secret))
            ),
        )
        .body("grant_type=client_credentials")
        .send()
        .await
        .unwrap();

    // Get the response
    let body = res.text().await.unwrap();
    println!("body = {:?}", body);
}
