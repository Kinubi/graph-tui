use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct Edge {
    pub id: u64,
    pub from: u64,
    pub to: u64,
    pub label: String,
}
