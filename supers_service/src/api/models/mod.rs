use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateSuper {
    pub name: String,
    pub powers: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateSuperResponse {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetSuper {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetSuperResponse {
    pub id: String,
    pub name: String,
    pub powers: String,
}
