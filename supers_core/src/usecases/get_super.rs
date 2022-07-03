use tokio::sync::RwLock;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::drivers::db::memory::InMemoryDB;
use crate::repositories::supers::SupersRepository;
use crate::repositories::Repository;
use crate::entities::supers::Super;

pub async fn execute(
    id: String,
    db: Arc<RwLock<SupersRepository<InMemoryDB>>>,
) -> Super {
    let supr = db.write().await.find_by_id(&id).await.unwrap();
    
    supr
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
