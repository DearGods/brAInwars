use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    Extension,
};
use axum_login::AuthSession;
use futures::{sink::SinkExt, stream::StreamExt};
use http::StatusCode;
use std::sync::Arc;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(auth): Extension<AuthSession<Backend>>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user = match auth.user {
        Some(user) => user,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    tracing::info!("auth chat websocket_handler: user: {:#?}", user);
    ws.on_upgrade(|socket| websocket(socket, state))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.chat_tx.subscribe();

    // Now send the "joined" message to all subscribers.
    // let msg = format!("{} joined.", username);
    // tracing::debug!("{}", msg);
    // let _ = state.chat_tx.send(msg);

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let tx = state.chat_tx.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            if !text.trim().is_empty() {
                let _ = tx.send(text);
            }
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Send "user left" message (similar to "joined" above).
    // let msg = format!("{} left.", username);
    // tracing::debug!("{}", msg);
    // let _ = state.chat_tx.send(msg);

    // Remove username from map so new clients can take it again.
}
