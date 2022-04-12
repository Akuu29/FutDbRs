use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Nation {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Nations {
    pub item: Nation,
}