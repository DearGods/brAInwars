use crate::startup::application::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;

pub(crate) async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    tracing::info!("websocket_handler game_ws_notifier connected.");
    ws.on_upgrade(move |socket| websocket(socket, state))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, _receiver) = stream.split();
    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.game_tx.subscribe();
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
    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => tracing::error!("game_ws_notifier send_task completed"),
    };
}
