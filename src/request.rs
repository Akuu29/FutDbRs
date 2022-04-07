use reqwest::{Response, header, Client};
use dotenv::dotenv;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn http_request(url: String) -> Result<Response> {
    dotenv().ok();

    let token = env::var("TOKEN")
      .expect("TOKEN is not found")
      .parse()
      .unwrap();
    
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());
    headers.insert("X-AUTH-TOKEN", token);

    let client = Client::builder().build()?;
    let res = client
      .get(url)
      .headers(headers)
      .send()
      .await?;

    Ok(res)
}