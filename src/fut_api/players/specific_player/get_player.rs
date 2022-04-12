use crate::request;
use crate::fut_api::players::specific_player::model_player::Players;
use crate::mime_type::MimeType;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn http_get_player(id: u32) -> Result<Players> {
    let url = format!("https://futdb.app/api/players/{}", id);
    let res_type = MimeType::Json;

    let body = request::http_request(url, res_type)
        .await?
        .json::<Players>()
        .await?;
    
    Ok(body)
}