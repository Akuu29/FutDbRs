use reqwest::{Response, header, Client};
use dotenv::dotenv;
use std::env;
use crate::mime_type::MimeType;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn http_request(url: String, res_type: MimeType) -> Result<Response> {
    dotenv().ok();

    let mime_type = match res_type {
        MimeType::Json => "application/json",
        MimeType::Image => "image/png",
    };

    let token = env::var("TOKEN")
      .expect("TOKEN is not found")
      .parse()
      .unwrap();
    
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", mime_type.parse().unwrap());
    headers.insert("X-AUTH-TOKEN", token);

    let client = Client::builder().build()?;
    let res = client
      .get(url)
      .headers(headers)
      .send()
      .await?;

    Ok(res)
}