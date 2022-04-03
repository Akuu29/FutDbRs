use reqwest::header;
use dotenv::dotenv;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn http_request(url: &str) -> Result<T> {
    dotenv().ok();

    let token = env::var("TOKEN")
        .expect("TOKEN is not found")
        .parse()
        .unwrap();
    
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());
    headers.insert("X-AUTH-TOKEN", token);

    let client = reqwest::Client::builder().build()?;
    let res = client
        .get(url)
        .headers(headers)
        .send()?
        .await?;
    
    let body = res.text().await?;

    Ok(body)
}