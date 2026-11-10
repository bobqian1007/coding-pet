use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EventSource {
    Codex,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskEvent {
    Completed,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UnifiedEvent {
    pub version: u8,
    pub source: EventSource,
    pub session_id: String,
    pub workspace: String,
    pub event: TaskEvent,
    pub message: String,
    pub timestamp: u64,
}
