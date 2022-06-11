use std::sync::RwLock;
use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
    Extension,
};

use crate::api::models;
use crate::drivers::db::memory::InMemoryDB;
use crate::repositories::supers::SupersRepository;
use crate::repositories::Repository;

pub(crate) async fn execute(
    Path(id): Path<String>,
    Extension(db): Extension<Arc<RwLock<SupersRepository<InMemoryDB>>>>,
) -> impl IntoResponse {
    let supr = db.read().unwrap().find_by_id(&id).unwrap();

    // this will be converted into a JSON response
    // with a status code of `200 Ok`
    (StatusCode::OK, Json(models::GetSuperResponse { id: supr.id, name: supr.name, powers: supr.powers }))
}