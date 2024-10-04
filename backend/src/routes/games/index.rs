use crate::database::game::all_games_by_excluded_status::all_games_by_excluded_status;
use crate::domain::game::game_obj::Game;
use crate::domain::game::game_status::GameStatus;
use crate::errors::error::Error;
use crate::routes::basic_response::ResponseStatus;
use crate::routes::games::get::GetGameResponse;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexGameResponse {
    pub games: Vec<GetGameResponse>,
    pub status: ResponseStatus,
    pub message: Option<String>,
}

impl IndexGameResponse {
    fn from(games: Vec<Game>, status: ResponseStatus, message: Option<String>) -> Self {
        Self {
            games: games
                .into_iter()
                .map(|game| GetGameResponse::new(game, None, vec![], status, message.clone()))
                .collect(),
            status,
            message,
        }
    }
}

#[tracing::instrument(name = "Get all games", skip(pool), ret, err)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<IndexGameResponse>, StatusCode> {
    let games = all_games_by_excluded_status(&pool, 10, GameStatus::Settled)
        .await
        .map_err(Error::from)?;
    Ok(Json(IndexGameResponse::from(
        games,
        ResponseStatus::Success,
        None,
    )))
}
