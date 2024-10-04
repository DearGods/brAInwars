use crate::database::game::get_game::get_game;
use crate::database::participant_action::get_all_players_status;
use crate::database::participant_action::get_all_players_status::GamePlayersStatus;
use crate::database::participant_action::get_player_status::get_player_status;
use crate::domain::date::Date;
use crate::domain::game::game_obj::Game;
use crate::domain::game::game_status::GameStatus;
use crate::domain::id::Id;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::routes::basic_response::ResponseStatus;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_login::AuthSession;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::fmt::Display;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetGameRequest {
    game_id: Id,
}

#[typeshare]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum PLAYER_STATUS {
    NOT_IN_GAME,
    JOINED_GAME,
    LEFT_GAME,
}

impl Display for PLAYER_STATUS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PLAYER_STATUS::NOT_IN_GAME => "NOT_IN_GAME".to_string(),
            PLAYER_STATUS::JOINED_GAME => "JOINED_GAME".to_string(),
            PLAYER_STATUS::LEFT_GAME => "LEFT_GAME".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetGameResponse {
    pub id: Id,
    pub game_id: f64,
    pub entry_fee: f64,
    pub mint: String,
    pub created_at: Date,
    pub game_status: GameStatus,
    pub num_participants: f64,
    pub winner: Option<String>,
    pub reveled_limit: Option<f64>,
    pub player_status: Option<PLAYER_STATUS>,
    pub start_time: Option<f64>,
    pub counter_end_time: Option<f64>,
    pub current_time: Date,
    pub players_statuses: Vec<GamePlayersStatus>,
    pub status: ResponseStatus,
    pub message: Option<String>,
}

impl GetGameResponse {
    pub fn new(
        game: Game,
        player_status: Option<PLAYER_STATUS>,
        players_statuses: Vec<GamePlayersStatus>,
        status: ResponseStatus,
        message: Option<String>,
    ) -> Self {
        let reveled_limit = match game.game_status {
            GameStatus::Finished => Some(game.reveled_limit.as_ref().clone().get() as f64),
            GameStatus::Settled => Some(game.reveled_limit.as_ref().clone().get() as f64),
            _ => None,
        };

        let game_status = game.game_status.clone();
        let winner = game.winner.clone();
        let counter_end_time: Option<f64> =
            if game.num_participants.get() >= 2 && game.waiting_for_players_start_time.is_some() {
                Option::from(
                    game.waiting_for_players_start_time
                        .unwrap_or_default()
                        .get() as f64
                        + game.wait_for_players_limit.get() as f64,
                )
            } else {
                None
            };
        Self {
            id: Id::from(game.id),
            game_id: game.game_id.get() as f64,
            mint: game.mint,
            entry_fee: game.entry_fee.get() as f64,
            created_at: Date::from(game.created_at),
            game_status: game.game_status,
            num_participants: game.num_participants.get() as f64,
            counter_end_time,
            winner: match (game_status, winner) {
                (GameStatus::Settled, None) => Some("No winner".to_string()),
                (_, Some(winner)) => Some(winner),
                (_, None) => None,
            },
            reveled_limit,
            player_status,
            start_time: Option::from(game.start_time.unwrap_or_default().get() as f64),
            current_time: Date::from(Utc::now()),
            players_statuses,
            status,
            message,
        }
    }
}

#[tracing::instrument(name = "Get game", skip(auth, pool), ret, err)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Query(parameters): Query<GetGameRequest>,
) -> Result<Json<GetGameResponse>, StatusCode> {
    let user = match auth.user {
        Some(user) => user,
        None => return Err(Error::AuthError("User not found".to_string()).into()),
    };
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let game = get_game(&mut transaction, parameters.game_id.as_uuid())
        .await
        .map_err(Error::from)?;
    let player_status = get_player_status(&mut transaction, user.id, game.id)
        .await
        .map_err(Error::from)?;
    let players_statuses =
        get_all_players_status::get_all_players_status(&mut transaction, game.id)
            .await
            .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(GetGameResponse::new(
        game,
        Option::from(player_status),
        players_statuses,
        ResponseStatus::Success,
        None,
    )))
}
