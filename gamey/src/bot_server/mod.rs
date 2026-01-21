pub mod choose;
pub mod error;
pub mod state;
pub mod version;
use axum::response::IntoResponse;
use std::sync::Arc;
pub use version::*;

use crate::{RandomBot, YBotRegistry, state::AppState};

pub async fn run_bot_server(port: u16) {
    let bots = YBotRegistry::new().with_bot(Arc::new(RandomBot));
    let state = AppState::new(bots);
    let app = axum::Router::new()
        .route("/status", axum::routing::get(status))
        .route(
            "/{api_version}/ybot/choose/{bot_id}",
            axum::routing::post(choose::choose),
        )
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server mode: Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

pub async fn status() -> impl IntoResponse {
    "OK"
}
