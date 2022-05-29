mod drivers;
mod repositories;
mod entities;
mod api;

use std::sync::RwLock;
use std::sync::Arc;
use crate::repositories::Repository;
use axum::extract::Path;
use crate::drivers::db::memory::InMemoryDB;
use crate::repositories::supers::SupersRepository;
use axum::{
    routing::{get, post, delete},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    error_handling::HandleErrorLayer,
    Extension,
};

use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use std::time::Duration;
use crate::entities::supers::Super;
use api::models;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // setup the database
    let db = Arc::new(RwLock::new(SupersRepository::<InMemoryDB>::new(InMemoryDB::new())));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/supers", get(get_all_supers).post(create_super))
        .route("/supers/:id", get(get_super).delete(|id: String| async { id }))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(Extension(db))
                .into_inner(),
        );



    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_all_supers(
    Extension(db): Extension<Arc<RwLock<SupersRepository<InMemoryDB>>>>,
) -> impl IntoResponse {
    let suprs = db.write().unwrap().find_all().unwrap();

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(suprs))
}

async fn get_super(
    Path(id): Path<String>,
    Extension(db): Extension<Arc<RwLock<SupersRepository<InMemoryDB>>>>,
) -> impl IntoResponse {
    let supr = db.read().unwrap().find_by_id(&id).unwrap();

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(models::GetSuperResponse { id: supr.id, name: supr.name, powers: supr.powers }))
}

async fn create_super(
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
