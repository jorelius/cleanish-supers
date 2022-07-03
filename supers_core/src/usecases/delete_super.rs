use tokio::sync::RwLock;
use std::sync::Arc;

use crate::drivers::db::memory::InMemoryDB;
use crate::repositories::supers::SupersRepository;
use crate::repositories::Repository;

pub async fn execute(
    id: String,
    db: Arc<RwLock<SupersRepository<InMemoryDB>>>,
) {
    let _ = db.write().await.delete(&id).await.unwrap();
}