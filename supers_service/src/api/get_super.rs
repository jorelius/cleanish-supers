use std::sync::RwLock;
use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
    Extension,
};

use supers_core::drivers::db::memory::InMemoryDB;
use supers_core::repositories::supers::SupersRepository;
use supers_core::usecases;
use crate::api::models;

pub(crate) async fn execute(
    Path(id): Path<String>,
    Extension(db): Extension<Arc<RwLock<SupersRepository<InMemoryDB>>>>,
) -> impl IntoResponse {
    let supr = usecases::get_super::execute(id, db).await;

    // this will be converted into a JSON response
    // with a status code of `200 Ok`
    (StatusCode::OK, Json(models::GetSuperResponse { id: supr.id, name: supr.name, powers: supr.powers }))
}