use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Super {
    pub id: String,
    pub name: String,
    pub powers: String,
}