// extern crate reqwest;
use reqwest::header;
use dotenv::dotenv;
use std::env;
// use std::collections::HashMap;

async fn http_get_clubs() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("TOKEN").expect("TOKEN is not found").parse().unwrap();
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "applicaion/json".parse().unwrap());
    headers.insert("X-AUTH-TOKEN", token);

    println!("{}", "aaaaaaaaaaaaaaaaaaaaa");

    let client = reqwest::Client::builder().build()?;
    let res = client
      .get("https://futdb.app/api/clubs?page=1&limit=20")
      .headers(headers)
      .send()
      .await?
      .text()
      .await?;

    println!("{:?}", res);

    // let context = res
    //   .json::<HashMap<String, String>>()
    //   .await?;

    // println!("{:?}", context);

    Ok(())
}

fn main() {
    http_get_clubs();
}