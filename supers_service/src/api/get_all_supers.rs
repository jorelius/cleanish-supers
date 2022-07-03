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

pub(crate) async fn execute(
    Extension(db): Extension<Arc<RwLock<SupersRepository<InMemoryDB>>>>,
) -> impl IntoResponse {
    let suprs = usecases::get_all_supers::execute(db).await;

    // this will be converted into a JSON response
    // with a status code of `200 Ok`
    (StatusCode::OK, Json(suprs))
}