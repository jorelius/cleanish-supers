use std::sync::RwLock;
use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    Extension,
};

use crate::drivers::db::memory::InMemoryDB;
use crate::repositories::supers::SupersRepository;
use crate::repositories::Repository;

pub(crate) async fn execute(
    Extension(db): Extension<Arc<RwLock<SupersRepository<InMemoryDB>>>>,
) -> impl IntoResponse {
    let suprs = db.write().unwrap().find_all().unwrap();

    // this will be converted into a JSON response
    // with a status code of `200 Ok`
    (StatusCode::OK, Json(suprs))
}