mod config;
use std::sync::Arc;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::extract::State;
#[derive(Clone)]
pub struct AppState {
    pub settings: std::sync::Arc<config::Settings>
}


#[tokio::main]
async fn main() {
    let settings = config::load().expect("Failed to load configuration");
    
    let state = AppState {
        settings: Arc::new(settings)
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
        .route("/port", get(port))
        .route("/health", get(health_check))
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
