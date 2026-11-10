use std::{path::Path, time::SystemTime};

use serde::Deserialize;

use crate::event::{EventSource, TaskEvent, UnifiedEvent};

const AGENT_TURN_COMPLETE: &str = "agent-turn-complete";
const MAX_SESSION_ID_BYTES: usize = 256;
const MAX_WORKSPACE_BYTES: usize = 4096;

#[derive(Debug, Deserialize)]
pub struct CodexNotifyPayload {
    #[serde(rename = "type")]
    event_type: String,
    #[serde(rename = "thread-id")]
    thread_id: String,
    #[serde(rename = "turn-id")]
    _turn_id: Option<String>,
    cwd: String,
}

pub fn adapt(payload: CodexNotifyPayload) -> Result<UnifiedEvent, &'static str> {
    if payload.event_type != AGENT_TURN_COMPLETE {
        return Err("unsupported Codex event");
    }
    if payload.thread_id.is_empty() || payload.thread_id.len() > MAX_SESSION_ID_BYTES {
        return Err("invalid thread id");
    }
    if payload.cwd.is_empty()
        || payload.cwd.len() > MAX_WORKSPACE_BYTES
        || !Path::new(&payload.cwd).is_absolute()
    {
        return Err("invalid workspace");
    }

    let timestamp = SystemTime::UNIX_EPOCH
        .elapsed()
        .map_err(|_| "system clock is before Unix epoch")?
        .as_secs();

    Ok(UnifiedEvent {
        version: 1,
        source: EventSource::Codex,
        session_id: payload.thread_id,
        workspace: payload.cwd,
        event: TaskEvent::Completed,
        message: "Codex agent turn completed".to_owned(),
        timestamp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOCAL_CODEX_FIXTURE: &str = r#"{
        "type": "agent-turn-complete",
        "thread-id": "thread-123",
        "turn-id": "turn-456",
        "cwd": "/repo/coding-pet",
        "input-messages": ["do not retain this"],
        "last-assistant-message": "also do not retain this"
    }"#;

    #[test]
    fn maps_local_codex_notify_shape_to_completed_event() {
        let payload = serde_json::from_str(LOCAL_CODEX_FIXTURE).unwrap();
        let event = adapt(payload).unwrap();

        assert_eq!(event.version, 1);
        assert_eq!(event.source, EventSource::Codex);
        assert_eq!(event.session_id, "thread-123");
        assert_eq!(event.workspace, "/repo/coding-pet");
        assert_eq!(event.event, TaskEvent::Completed);
        assert_eq!(event.message, "Codex agent turn completed");
    }

    #[test]
    fn rejects_non_completion_events() {
        let payload = serde_json::from_str::<CodexNotifyPayload>(
            r#"{"type":"other","thread-id":"thread-123","cwd":"/repo"}"#,
        )
        .unwrap();

        assert_eq!(adapt(payload), Err("unsupported Codex event"));
    }

    #[test]
    fn requires_an_absolute_workspace() {
        let payload = serde_json::from_str::<CodexNotifyPayload>(
            r#"{"type":"agent-turn-complete","thread-id":"thread-123","cwd":"relative"}"#,
        )
        .unwrap();

        assert_eq!(adapt(payload), Err("invalid workspace"));
    }
}
