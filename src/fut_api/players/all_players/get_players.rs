use crate::request;
use crate::fut_api::players::all_players::model_players::Players;
use crate::mime_type::MimeType;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn http_get_players() -> Result<Players> {
    let url = "https://futdb.app/api/players?page=1&limit=20".to_string();
    let res_type = MimeType::Json;
    let body = request::http_request(url, res_type)
        .await?
        .json::<Players>()
        .await?;
    
    Ok(body)
}
