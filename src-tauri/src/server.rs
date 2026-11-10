use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::StatusCode,
    routing::post,
};
use tauri::{AppHandle, Emitter};

use crate::{
    adapters::codex::{self, CodexNotifyPayload},
    notification,
    task_manager::TaskManager,
};

pub const LISTENER_ADDR: &str = "127.0.0.1:17891";
pub const MAX_PAYLOAD_BYTES: usize = 64 * 1024;
pub const TASK_STATUS_EVENT: &str = "task-status-changed";

#[derive(Clone)]
struct AppState {
    app: AppHandle,
    task_manager: TaskManager,
}

pub fn spawn(app: AppHandle, task_manager: TaskManager) {
    tauri::async_runtime::spawn(async move {
        let listener = match tokio::net::TcpListener::bind(LISTENER_ADDR).await {
            Ok(listener) => listener,
            Err(error) => {
                eprintln!("failed to bind Coding Pet listener on {LISTENER_ADDR}: {error}");
                return;
            }
        };

        let router = Router::new()
            .route("/events", post(receive_event))
            .layer(DefaultBodyLimit::max(MAX_PAYLOAD_BYTES))
            .with_state(AppState { app, task_manager });

        if let Err(error) = axum::serve(listener, router).await {
            eprintln!("Coding Pet listener stopped: {error}");
        }
    });
}

async fn receive_event(
    State(state): State<AppState>,
    Json(payload): Json<CodexNotifyPayload>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    let event =
        codex::adapt(payload).map_err(|message| (StatusCode::UNPROCESSABLE_ENTITY, message))?;

    state.task_manager.apply(event.clone());
    state.app.emit(TASK_STATUS_EVENT, &event).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to emit task event",
        )
    })?;
    notification::show_completion(&state.app);
    eprintln!("accepted Codex completion event");

    Ok(StatusCode::ACCEPTED)
}
