use std::sync::{Arc, Mutex};

use crate::event::UnifiedEvent;

#[derive(Clone, Default)]
pub struct TaskManager {
    latest: Arc<Mutex<Option<UnifiedEvent>>>,
}

impl TaskManager {
    pub fn apply(&self, event: UnifiedEvent) {
        *self.latest.lock().expect("task state lock poisoned") = Some(event);
    }

    #[cfg(test)]
    fn latest(&self) -> Option<UnifiedEvent> {
        self.latest
            .lock()
            .expect("task state lock poisoned")
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::event::{EventSource, TaskEvent};

    use super::*;

    #[test]
    fn stores_the_latest_unified_event() {
        let manager = TaskManager::default();
        let event = UnifiedEvent {
            version: 1,
            source: EventSource::Codex,
            session_id: "thread-123".to_owned(),
            workspace: "/repo".to_owned(),
            event: TaskEvent::Completed,
            message: "Codex agent turn completed".to_owned(),
            timestamp: 1,
        };

        manager.apply(event.clone());

        assert_eq!(manager.latest(), Some(event));
    }
}
