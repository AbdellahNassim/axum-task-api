mod config;
mod database;
mod models;
mod repositories;
mod services;
mod handlers;
use std::sync::Arc;
use axum::{Router, routing::{get,post}};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::PgPool;

use axum::{extract::State};

use crate::{handlers::auth_handler};

#[derive(Clone)]
pub struct AppState {
    pub settings: std::sync::Arc<config::Settings>,
    pub db: PgPool,
}


#[tokio::main]
async fn main() {
    let settings = config::load().expect("Failed to load configuration");
    let db = database::create_pool(&settings.database_url)
        .await
        .expect("Failed to create database pool");
    let state = AppState {
        settings: Arc::new(settings),
        db: db
    };
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/register", post(auth_handler::register))
        .route("/port", get(port))
        .route("/health", get(health_check))
        .route("/health/db", get(db_health))
        .with_state(state.clone())
        .layer(TraceLayer::new_for_http());
    let port = state.settings.port;
    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap();

    tracing::info!("Server is running on {}", address);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Task API Running!"
}

async fn health_check() -> &'static str {
    "OK"
}

async fn port(State(state): State<AppState>) -> String {
    format!("Running on Port: {}", state.settings.port)
}


async fn db_health(
    State(state): State<AppState>
) -> String {

    match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => "Database OK".to_string(),
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            format!("Database health check failed: {}", e)
        }
    }


}