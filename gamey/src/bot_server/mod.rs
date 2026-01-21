pub mod choose;
pub mod error;
pub mod state;
pub mod version;
use axum::response::IntoResponse;
use std::sync::Arc;
pub use choose::MoveResponse;
pub use error::ErrorResponse;
pub use version::*;

use crate::{RandomBot, YBotRegistry, state::AppState};

/// Creates the Axum router with the given state.
/// This is useful for testing without binding to a port.
pub fn create_router(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/status", axum::routing::get(status))
        .route(
            "/{api_version}/ybot/choose/{bot_id}",
            axum::routing::post(choose::choose),
        )
        .with_state(state)
}

/// Creates the default application state with the standard bot registry.
pub fn create_default_state() -> AppState {
    let bots = YBotRegistry::new().with_bot(Arc::new(RandomBot));
    AppState::new(bots)
}

pub async fn run_bot_server(port: u16) {
    let state = create_default_state();
    let app = create_router(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server mode: Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

pub async fn status() -> impl IntoResponse {
    "OK"
}
