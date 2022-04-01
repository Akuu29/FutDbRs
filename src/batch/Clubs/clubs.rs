use reqwest::header;
use dotenv::dotenv;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn http_get_clubs() -> Result<String> { // Result<(), Box<dyn std::error::Error>>
    dotenv().ok();

    let token = env::var("TOKEN")
      .expect("TOKEN is not found")
      .parse()
      .unwrap();

    // headers
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "applicaion/json".parse().unwrap());
    headers.insert("X-AUTH-TOKEN", token);

    let client = reqwest::Client::builder().build()?;
    let res = client
      .get("https://futdb.app/api/clubs?page=1&limit=20")
      .headers(headers)
      .send()
      .await?;

    let body = res.text().await?;

    Ok(body)
}

#[tokio::main]
async fn main() {
    let res = http_get_clubs().await.unwrap();
    println!("{}", res);
}