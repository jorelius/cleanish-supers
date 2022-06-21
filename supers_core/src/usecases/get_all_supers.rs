use std::sync::RwLock;
use std::sync::Arc;

use crate::drivers::db::memory::InMemoryDB;
use crate::repositories::supers::SupersRepository;
use crate::repositories::Repository;
use crate::entities::supers::Super;

pub async fn execute(
    db: Arc<RwLock<SupersRepository<InMemoryDB>>>,
) -> Vec<Super> {
    let suprs = db.write().unwrap().find_all().unwrap();

    suprs
}