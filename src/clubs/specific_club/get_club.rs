use crate::request;
use crate::clubs::specific_club::model_club::Clubs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn http_get_club(id: u32) -> Result<Clubs> {
    let url = format!("https://futdb.app/api/clubs/{}", id);
    let body = request::http_request(url)
      .await?
      .json::<Clubs>()
      .await?;

    Ok(body)
}