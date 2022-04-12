use crate::fut_api::nations::all_nations::model_nations::Nations;
use crate::mime_type::MimeType;
use crate::request::http_request;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn http_get_nations() -> Result<Nations> {
    let url = format!("https://futdb.app/api/nations?page=1&limit=20");
    let res_type = MimeType::Json;
    let body = http_request(url, res_type)
        .await?
        .json::<Nations>()
        .await?;
    
    Ok(body)
}