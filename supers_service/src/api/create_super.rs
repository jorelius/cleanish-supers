use tokio::sync::RwLock;
use std::sync::Arc;

use axum::{
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
    Json(payload): Json<models::CreateSuper>,
    Extension(db): Extension<Arc<RwLock<SupersRepository<InMemoryDB>>>>,
) -> impl IntoResponse {
    let obj = usecases::create_super::CreateSuper {
        name: payload.name,
        powers: payload.powers,
    };

    let id = usecases::create_super::execute(obj, db).await;

    // use repo

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(models::CreateSuperResponse { id: id }))
}