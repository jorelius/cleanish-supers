use uuid::Uuid;
use std::sync::RwLock;
use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    Extension,
};

use crate::api::models;
use crate::drivers::db::memory::InMemoryDB;
use crate::repositories::supers::SupersRepository;
use crate::repositories::Repository;
use crate::entities::supers::Super;

pub(crate) async fn execute(
    Json(payload): Json<models::CreateSuper>,
    Extension(db): Extension<Arc<RwLock<SupersRepository<InMemoryDB>>>>,
) -> impl IntoResponse {
    // insert your application logic here
    let sup = Super {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        powers: payload.powers,
    };

    db.write().unwrap().create(&sup).unwrap();

    // use repo

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(models::CreateSuperResponse { id: sup.id }))
}