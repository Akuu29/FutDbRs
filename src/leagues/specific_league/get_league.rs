use crate::mime_type::MimeType;
use crate::request;
use crate::leagues::specific_league::model_league::Leagues;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn http_get_league(id: u32) -> Result<Leagues> {
    let url = format!("https://futdb.app/api/leagues/{}", id);
    let res_type = MimeType::Json;
    let body = request::http_request(url, res_type)
      .await?
      .json::<Leagues>()
      .await?;
    
    Ok(body)
}