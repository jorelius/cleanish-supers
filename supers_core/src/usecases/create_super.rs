use uuid::Uuid;
use tokio::sync::RwLock;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::drivers::db::memory::InMemoryDB;
use crate::repositories::supers::SupersRepository;
use crate::repositories::Repository;
use crate::entities::supers::Super;

pub async fn execute(
    payload: CreateSuper,
    db: Arc<RwLock<SupersRepository<InMemoryDB>>>,
) -> String {
    // insert your application logic here
    let sup = Super {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        powers: payload.powers,
    };

    db.write().await.create(&sup).await.unwrap();

    sup.id // return id
}

#[derive(Deserialize, Serialize)]
pub struct CreateSuper {
    pub name: String,
    pub powers: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateSuperResponse {
    pub id: String,
}