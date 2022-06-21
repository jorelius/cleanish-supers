mod api;
mod settings;

use std::time::Duration;
use std::sync::RwLock;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    http::StatusCode, Router,
    error_handling::HandleErrorLayer,
    Extension,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::{
    ServiceBuilderExt,
    trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse}
};
use tower_http::request_id::MakeRequestUuid;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::settings::Settings;

use supers_core::{
    drivers::db::memory::InMemoryDB, 
    entities::supers::Super, 
    repositories::{supers::SupersRepository, Repository}
};

use crate::api::get_all_supers;
use crate::api::create_super;
use crate::api::get_super;
use crate::api::delete_super;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // load configuration
    let settings = Settings::new();

    // setup the database
    let db = Arc::new(RwLock::new(SupersRepository::<InMemoryDB>::new(InMemoryDB::new())));

    // load default supers
    load_bulk_super_data(&db, settings.unwrap().supers);

    // build our application with a set of routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/supers", post(create_super::execute).get(get_all_supers::execute))
        .route("/supers/:id", get(get_super::execute).delete(delete_super::execute))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                // make sure to set request ids before the request reaches `TraceLayer`
                .set_x_request_id(MakeRequestUuid{})
                // log requests and responses
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_response(DefaultOnResponse::new().include_headers(true))
                )
                // propagate the header to the response before the response reaches `TraceLayer`
                .propagate_x_request_id()
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

fn load_bulk_super_data(db: &Arc<RwLock<SupersRepository<InMemoryDB>>>, supers: Vec<Super>) {
    for supr in supers {
        db.write().unwrap().create(&supr).unwrap();
    }
}
