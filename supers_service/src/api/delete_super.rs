use tokio::sync::RwLock;
use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Extension,
};

use supers_core::drivers::db::memory::InMemoryDB;
use supers_core::repositories::supers::SupersRepository;
use supers_core::usecases;

pub(crate) async fn execute(
    Path(id): Path<String>,
    Extension(db): Extension<Arc<RwLock<SupersRepository<InMemoryDB>>>>,
) -> impl IntoResponse {
    usecases::delete_super::execute(id, db).await;

    // this will be converted into a
    // 200 response OK
    StatusCode::OK
}