use crate::{Coordinates, GameY, YEN, check_api_version, error::ErrorResponse, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ChooseParams {
    api_version: String,
    bot_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct MoveResponse {
    pub api_version: String,
    pub bot_id: String,
    pub coords: Coordinates,
}

#[axum::debug_handler]
pub async fn choose(
    State(state): State<AppState>,
    Path(params): Path<ChooseParams>,
    Json(yen): Json<YEN>,
) -> Result<Json<MoveResponse>, Json<ErrorResponse>> {
    check_api_version(&params.api_version)?;
    let game_y = match GameY::try_from(yen) {
        Ok(game) => game,
        Err(err) => {
            return Err(Json(ErrorResponse::error(
                &format!("Invalid YEN format: {}", err),
                Some(params.api_version),
                Some(params.bot_id),
            )));
        }
    };
    let bot = match state.bots().find(&params.bot_id) {
        Some(bot) => bot,
        None => {
            let available_bots = state.bots().names().join(", ");
            return Err(Json(ErrorResponse::error(
                &format!(
                    "Bot not found: {}, available bots: [{}]",
                    params.bot_id, available_bots
                ),
                Some(params.api_version),
                Some(params.bot_id),
            )));
        }
    };
    let coords = match bot.choose_move(&game_y) {
        Some(coords) => coords,
        None => {
            // Handle the case where the bot has no valid moves
            return Err(Json(ErrorResponse::error(
                "No valid moves available for the bot",
                Some(params.api_version),
                Some(params.bot_id),
            )));
        }
    };
    let response = MoveResponse {
        api_version: params.api_version,
        bot_id: params.bot_id,
        coords,
    };
    Ok(Json(response))
}
