use crate::database::game::get_game::get_game;
use crate::database::game::update_num_participants_left::update_num_participants_left;
use crate::database::participant::get_participant::get_participant;
use crate::database::participant::update_exit_time::update_exit_time;
use crate::database::participant::update_participant_status::update_participant_status;
use crate::database::participant_action::create_participant_action::create_participant_action;
use crate::database::participant_action::get_participant_action::get_participant_action;
use crate::database::user::get_user_opt_by_id::get_user_opt_by_id;
use crate::domain::game::game_status::GameStatus;
use crate::domain::id::Id;
use crate::domain::job::job_obj::Job;
use crate::domain::job::job_type::JobType;
use crate::domain::participant::participant_action_type::ParticipantActionType;
use crate::domain::participant::participant_status::ParticipantStatus;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::routes::basic_response::ResponseStatus;
use crate::startup::application::AppState;
use crate::websockets::messages::{publish_msg, MsgName, PlayerLeft};
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_login::AuthSession;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::fmt::Display;
use std::sync::Arc;
use typeshare::typeshare;
use uuid::Uuid;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveGameRequest {
    pub game_id: Id,
}

impl LeaveGameRequest {
    pub fn new(id: Uuid) -> Self {
        Self {
            game_id: Id::from(id),
        }
    }
}

impl Display for LeaveGameRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveGameResponse {
    pub game_id: Option<Id>,
    pub user_id: Option<Id>,
    pub status: ResponseStatus,
    pub message: Option<String>,
}

#[tracing::instrument(name = "Leave a game", skip(auth, pool, state), ret, err)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<LeaveGameRequest>,
) -> Result<Json<LeaveGameResponse>, StatusCode> {
    let user = match auth.user {
        Some(user) => user,
        None => {
            return Ok(Json(LeaveGameResponse {
                game_id: Option::from(body.game_id),
                user_id: None,
                status: ResponseStatus::Failure,
                message: Option::from("User not found".to_string()),
            }));
        }
    };
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let db_user = match get_user_opt_by_id(&mut transaction, &user.id)
        .await
        .map_err(Error::from)?
    {
        Some(user) => user,
        None => {
            return Ok(Json(LeaveGameResponse {
                game_id: Option::from(body.game_id),
                user_id: None,
                status: ResponseStatus::Failure,
                message: Some("DB User not found".to_string()),
            }));
        }
    };
    let game = get_game(&mut transaction, body.game_id.as_uuid())
        .await
        .map_err(Error::from)?;
    if game.mev_lock {
        return Ok(Json(LeaveGameResponse {
            game_id: Option::from(Id::from(game.id)),
            user_id: Option::from(Id::from(user.id)),
            status: ResponseStatus::Failure,
            message: Option::from("Game is MEV locked".to_string()),
        }));
    }
    update_num_participants_left(
        &mut transaction,
        game.id,
        game.num_participants_left_game + 1u64,
    )
    .await
    .map_err(Error::from)?;
    match game.game_status {
        GameStatus::WaitingForPlayers => {
            return Ok(Json(LeaveGameResponse {
                game_id: Option::from(Id::from(game.id)),
                user_id: Option::from(Id::from(user.id)),
                status: ResponseStatus::Failure,
                message: Option::from("Game still waiting for others to start".to_string()),
            }));
        }
        GameStatus::Settled => {
            return Ok(Json(LeaveGameResponse {
                game_id: Option::from(Id::from(game.id)),
                user_id: Option::from(Id::from(user.id)),
                status: ResponseStatus::Failure,
                message: Option::from("Game Ended".to_string()),
            }));
        }
        GameStatus::Finished => {
            return Ok(Json(LeaveGameResponse {
                game_id: Option::from(Id::from(game.id)),
                user_id: Option::from(Id::from(user.id)),
                status: ResponseStatus::Failure,
                message: Option::from("Game Ended".to_string()),
            }));
        }
        GameStatus::OnGoing => (),
    }
    get_participant(
        &mut transaction,
        user.id,
        game.id,
        ParticipantStatus::JOINED,
    )
    .await
    .map_err(Error::from)?;
    get_participant_action(
        &mut transaction,
        user.id,
        game.id,
        ParticipantActionType::JOIN,
    )
    .await
    .map_err(Error::from)?;
    create_participant_action(
        &mut transaction,
        user.id,
        game.id,
        ParticipantActionType::LEAVE,
        "".to_string(),
    )
    .await
    .map_err(Error::from)?;
    update_participant_status(&mut transaction, user.id, game.id, ParticipantStatus::LEFT)
        .await
        .map_err(Error::from)?;
    update_exit_time(&mut transaction, user.id, game.id, Utc::now())
        .await
        .map_err(Error::from)?;
    Job::create_job(
        &state.pool,
        game.id,
        Some(user.wallet_address.clone()),
        JobType::LeaveGame,
    )
    .await
    .map_err(Error::from)?;
    if game.num_participants == game.num_participants_left_game {
        Job::create_job(&state.pool, game.id, None, JobType::FinishGame)
            .await
            .map_err(Error::from)?;
    }
    let msg = PlayerLeft {
        msg_name: MsgName::PLAYER_LEFT,
        game_id: Id::from(game.id),
        wallet_address: user.wallet_address.clone(),
        name: db_user.name,
        msg_id: Id::new(),
    };
    publish_msg(state.clone(), msg.to_string());

    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(LeaveGameResponse {
        game_id: Option::from(Id::from(game.id)),
        user_id: Option::from(Id::from(user.id)),
        status: ResponseStatus::Success,
        message: None,
    }))
}
