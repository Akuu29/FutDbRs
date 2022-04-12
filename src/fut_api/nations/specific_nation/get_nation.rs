use crate::fut_api::nations::specific_nation::model_nation::Nations;
use crate::mime_type::MimeType;
use crate::request;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn http_get_nation(id: u32) -> Result<Nations> {
    let url = format!("https://futdb.app/api/nations/{}", id);
    let res_type = MimeType::Json;
    let body = request::http_request(url, res_type)
        .await?
        .json::<Nations>()
        .await?;

    Ok(body)
}