use crate::request;
use crate::clubs::all_clubs::model_clubs::Clubs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// クラブ全件取得
pub async fn http_get_clubs() -> Result<Clubs> {
    let url = String::from("https://futdb.app/api/clubs?page=1&limit=20");
    let body = request::http_request(url)
      .await?
      .json::<Clubs>()// responseをjsonに変換
      .await?;

    Ok(body)
}