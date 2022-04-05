use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Club {
    pub id: u32,
    pub name: String,
    pub league: u32,
}

#[derive(Debug, Deserialize)]
pub struct Clubs {
    pub items: Vec<Club>,
}