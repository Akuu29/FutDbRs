use crate::request;
use crate::clubs::model_club::Clubs;

pub async fn http_get_clubs() -> Clubs {
    let base_url = "https://futdb.app/api/clubs?page=1&limit=20";
    let result = request::http_request(base_url).await;
    result.unwrap()
}