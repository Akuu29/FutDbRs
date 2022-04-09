use crate::request;
use crate::clubs::specific_club::model_club::Clubs;
use crate::mime_type::MimeType;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn http_get_club(id: u32) -> Result<Clubs> {
    let url = format!("https://futdb.app/api/clubs/{}", id);
    let res_type = MimeType::Json;
    let body = request::http_request(url, res_type)
      .await?
      .json::<Clubs>()
      .await?;

    Ok(body)
}