use crate::request;
use crate::mime_type::MimeType;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// pub async fn http_get_club_image(id: u32) -> Result<Bytes> {
//     // let mut file = std::fs::File::create("image.png").unwrap();
//     let url = format!("https://futdb.app/api/clubs/{}/image", id);
//     let res_type = MimeType::Image;
//     let body = request::http_request(url, res_type)
//       .await?
//       .bytes()
//       .await?;

//     let image = image::load_from_memory(&body)?;
    
//     Ok(body)
// }