use std::sync::RwLock;
use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Extension,
};

use crate::drivers::db::memory::InMemoryDB;
use crate::repositories::supers::SupersRepository;
use crate::repositories::Repository;

pub(crate) async fn execute(
    Path(id): Path<String>,
    Extension(db): Extension<Arc<RwLock<SupersRepository<InMemoryDB>>>>,
) -> impl IntoResponse {
    let _ = db.write().unwrap().delete(&id).unwrap();

    // this will be converted into a
    // 200 response OK
    StatusCode::OK
}