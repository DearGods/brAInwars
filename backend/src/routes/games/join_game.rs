use crate::blockchain::solana::helpers::get_player_balance;
use crate::database::game::get_game::get_game;
use crate::database::game::update_num_participants::update_num_participants;
use crate::database::game::update_waiting_for_players_start_time::update_waiting_for_players_start_time;
use crate::database::participant::count_participants::count_participants;
use crate::database::participant::create_participant::create_participant;
use crate::database::participant_action::create_participant_action::create_participant_action;
use crate::database::user::get_user_opt_by_id::get_user_opt_by_id;
use crate::domain::game::game_status::GameStatus;
use crate::domain::id::Id;
use crate::domain::job::job_obj::Job;
use crate::domain::job::job_type::JobType;
use crate::domain::participant::participant_action_type::ParticipantActionType;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::routes::basic_response::ResponseStatus;
use crate::startup::application::AppState;
use crate::websockets::messages::{publish_msg, GameCounter, MsgName, PlayerJoined};
use anchor_lang::prelude::Pubkey;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_login::AuthSession;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;
use typeshare::typeshare;
use uuid::Uuid;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGameRequest {
    pub game_id: Id,
}

impl JoinGameRequest {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            game_id: Id::from(uuid),
        }
    }
}

impl Display for JoinGameRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGameResponse {
    pub game_id: Option<Id>,
    pub user_id: Option<Id>,
    pub status: ResponseStatus,
    pub message: Option<String>,
}

#[tracing::instrument(name = "Join a game", skip(auth, pool, state), ret, err)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<JoinGameRequest>,
) -> Result<Json<JoinGameResponse>, StatusCode> {
    let user = match auth.user {
        Some(user) => user,
        None => {
            return Ok(Json(JoinGameResponse {
                game_id: Option::from(body.game_id),
                user_id: None,
                status: ResponseStatus::Failure,
                message: Some("User not found".to_string()),
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
            return Ok(Json(JoinGameResponse {
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
    match game.game_status {
        GameStatus::OnGoing => {
            return Ok(Json(JoinGameResponse {
                game_id: Option::from(body.game_id),
                user_id: Option::from(Id::from(user.id)),
                status: ResponseStatus::Failure,
                message: Some("Game Already Started".to_string()),
            }));
        }
        GameStatus::WaitingForPlayers => {}
        GameStatus::Settled | GameStatus::Finished => {
            return Ok(Json(JoinGameResponse {
                game_id: Option::from(body.game_id),
                user_id: Option::from(Id::from(user.id)),
                status: ResponseStatus::Failure,
                message: Some("Game Ended".to_string()),
            }));
        }
    }
    if game.mev_lock {
        return Ok(Json(JoinGameResponse {
            game_id: Option::from(body.game_id),
            user_id: Option::from(Id::from(user.id)),
            status: ResponseStatus::Failure,
            message: Some("Game MEV Locked".to_string()),
        }));
    }
    let time = chrono::Utc::now();
    if game.waiting_for_players_start_time.is_some()
        && game.waiting_for_players_start_time.unwrap().get() + game.wait_for_players_limit.get()
            < time.timestamp() as u64
    {
        return Ok(Json(JoinGameResponse {
            game_id: Option::from(body.game_id),
            user_id: Option::from(Id::from(user.id)),
            status: ResponseStatus::Failure,
            message: Some("Game Already Started".to_string()),
        }));
    }
    let player = Pubkey::from_str(&user.wallet_address.clone()).map_err(Error::from)?;
    let mint = Pubkey::from_str(&game.mint).map_err(Error::from)?;
    let player_balance = get_player_balance(state.clone(), &player, &mint).await;
    if player_balance.is_err() {
        return Ok(Json(JoinGameResponse {
            game_id: Option::from(body.game_id),
            user_id: Option::from(Id::from(user.id)),
            status: ResponseStatus::Failure,
            message: Some("Player Balance Not Found, player need to deposit".to_string()),
        }));
    }
    if player_balance.unwrap() < game.entry_fee.get() {
        return Ok(Json(JoinGameResponse {
            game_id: Option::from(body.game_id),
            user_id: Option::from(Id::from(user.id)),
            status: ResponseStatus::Failure,
            message: Some("Player Balance Too Low, player need to deposit".to_string()),
        }));
    }
    create_participant(&mut transaction, user.id, game.id)
        .await
        .map_err(Error::from)?;
    create_participant_action(
        &mut transaction,
        user.id,
        game.id,
        ParticipantActionType::JOIN,
        "".to_string(),
    )
    .await
    .map_err(Error::from)?;
    let participants = count_participants(&mut transaction, game.id)
        .await
        .map_err(Error::from)?;
    update_num_participants(&mut transaction, game.id, participants)
        .await
        .map_err(Error::from)?;
    if participants == 2 {
        update_waiting_for_players_start_time(&mut transaction, game.id, Utc::now().timestamp())
            .await
            .map_err(Error::from)?;
        let msg = GameCounter {
            msg_name: MsgName::GAME_COUNTER,
            game_id: Id::from(game.id),
            msg_id: Id::new(),
            start_time_from_now_in_milliseconds: (game.wait_for_players_limit.get()
                + game.wait_for_players_limit.get())
                as f64,
        };
        publish_msg(state.clone(), msg.to_string());
    }
    Job::create_job(
        &state.pool,
        game.id,
        Some(user.wallet_address.clone()),
        JobType::JoinGame,
    )
    .await
    .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;
    let msg = PlayerJoined {
        msg_name: MsgName::PLAYER_JOINED,
        game_id: Id::from(game.id),
        wallet_address: user.wallet_address.clone(),
        name: db_user.name,
        msg_id: Id::new(),
    };
    publish_msg(state.clone(), msg.to_string());
    Ok(Json(JoinGameResponse {
        game_id: Option::from(Id::from(game.id)),
        user_id: Option::from(Id::from(user.id)),
        status: ResponseStatus::Success,
        message: None,
    }))
}
