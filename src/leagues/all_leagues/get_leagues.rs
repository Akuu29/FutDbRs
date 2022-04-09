use crate::mime_type::MimeType;
use crate::request;
use crate::leagues::all_leagues::model_leagues::Leagues;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn http_get_leagues() -> Result<Leagues> {
    let url = format!("https://futdb.app/api/leagues?page=1&limit=20");
    let res_type = MimeType::Json;
    let body = request::http_request(url, res_type)
      .await?
      .json::<Leagues>()
      .await?;
    
    Ok(body)
}