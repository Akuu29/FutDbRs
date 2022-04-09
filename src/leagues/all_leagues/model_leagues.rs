use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct League {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Leagues {
    pub items: Vec<League>,
}