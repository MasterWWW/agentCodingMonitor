use crate::event::{phase_for_event, session_key};
use crate::types::{
    NormalizedEvent, Session, SourceHealth, StatusSnapshot, VibePhase, VibeSource,
};
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

const IDLE_AFTER_SECS: i64 = 30;

#[derive(Clone)]
pub struct SessionStore {
    inner: Arc<RwLock<StoreInner>>,
    tx: broadcast::Sender<StatusSnapshot>,
    port: u16,
}

struct StoreInner {
    sessions: HashMap<String, Session>,
    source_health: HashMap<VibeSource, SourceHealth>,
    lite_mode: bool,
}

impl SessionStore {
    pub fn new(port: u16) -> Self {
        let mut source_health = HashMap::new();
        for s in VibeSource::all() {
            source_health.insert(s, SourceHealth::default());
        }
        let (tx, _) = broadcast::channel(64);
        Self {
            inner: Arc::new(RwLock::new(StoreInner {
                sessions: HashMap::new(),
                source_health,
                lite_mode: false,
            })),
            tx,
            port,
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn subscribe(&self) -> broadcast::Receiver<StatusSnapshot> {
        self.tx.subscribe()
    }

    pub async fn set_lite_mode(&self, enabled: bool) {
        let mut g = self.inner.write().await;
        g.lite_mode = enabled;
        drop(g);
        let _ = crate::state::write_lite_mode(enabled);
        self.broadcast().await;
    }

    pub async fn get_lite_mode(&self) -> bool {
        self.inner.read().await.lite_mode
    }

    pub async fn mark_hook_installed(&self, source: VibeSource, installed: bool) {
        let mut g = self.inner.write().await;
        let h = g.source_health.entry(source).or_default();
        h.hook_installed = installed;
        drop(g);
        self.broadcast().await;
    }

    pub async fn apply_event(&self, ev: NormalizedEvent) {
        let now = Utc::now();
        let phase = phase_for_event(&ev.event_name);
        let sid = ev
            .session_id
            .clone()
            .unwrap_or_else(|| "default".to_string());
        let key = session_key(ev.source, &sid);

        let mut g = self.inner.write().await;
        let h = g.source_health.entry(ev.source).or_default();
        h.last_seen = Some(now);
        if let Some(p) = phase {
            h.phase = p;
        }

        let session = g.sessions.entry(key).or_insert_with(|| Session {
            source: ev.source,
            session_id: sid.clone(),
            cwd: ev.cwd.clone(),
            task_title: None,
            last_tool: None,
            last_activity_at: now,
            phase: VibePhase::Unknown,
        });

        session.last_activity_at = now;
        if ev.cwd.is_some() {
            session.cwd = ev.cwd.clone();
        }
        if ev.task_title.is_some() {
            session.task_title = ev.task_title.clone();
        }
        if ev.tool_name.is_some() {
            session.last_tool = ev.tool_name.clone();
        }
        if let Some(p) = phase {
            session.phase = p;
        } else if session.phase == VibePhase::Unknown {
            session.phase = VibePhase::Active;
        }

        drop(g);
        self.broadcast().await;
    }

    pub async fn apply_lite_activity(
        &self,
        source: VibeSource,
        cwd_hint: Option<String>,
        task_title: Option<String>,
    ) {
        let now = Utc::now();
        let sid = format!("lite-{}", source.label());
        let key = session_key(source, &sid);
        let mut g = self.inner.write().await;
        if !g.lite_mode {
            return;
        }
        let h = g.source_health.entry(source).or_default();
        h.last_seen = Some(now);
        h.phase = VibePhase::Active;
        h.note = Some("lite_mode".to_string());

        let session = g.sessions.entry(key).or_insert_with(|| Session {
            source,
            session_id: sid,
            cwd: cwd_hint.clone(),
            task_title: None,
            last_tool: None,
            last_activity_at: now,
            phase: VibePhase::Active,
        });
        session.last_activity_at = now;
        session.phase = VibePhase::Active;
        if cwd_hint.is_some() {
            session.cwd = cwd_hint;
        }
        if task_title.is_some() {
            session.task_title = task_title;
        }
        drop(g);
        self.broadcast().await;
    }

    pub async fn tick_idle(&self) {
        let now = Utc::now();
        let threshold = Duration::seconds(IDLE_AFTER_SECS);
        let mut changed = false;
        let mut g = self.inner.write().await;
        for session in g.sessions.values_mut() {
            if session.phase == VibePhase::Stopped {
                continue;
            }
            if now.signed_duration_since(session.last_activity_at) > threshold {
                if session.phase != VibePhase::Idle {
                    session.phase = VibePhase::Idle;
                    changed = true;
                }
            }
        }
        let sources: Vec<VibeSource> = g.source_health.keys().copied().collect();
        for source in sources {
            let has_active = g.sessions.values().any(|s| {
                s.source == source
                    && s.phase == VibePhase::Active
                    && now.signed_duration_since(s.last_activity_at) <= threshold
            });
            if let Some(health) = g.source_health.get_mut(&source) {
                if let Some(last) = health.last_seen {
                    if now.signed_duration_since(last) > threshold {
                        if health.phase != VibePhase::Idle && health.phase != VibePhase::Unknown {
                            health.phase = VibePhase::Idle;
                            changed = true;
                        }
                    }
                }
                if !has_active && health.last_seen.is_some() && health.phase == VibePhase::Active {
                    health.phase = VibePhase::Idle;
                    changed = true;
                }
            }
        }
        drop(g);
        if changed {
            self.broadcast().await;
        }
    }

    pub async fn snapshot(&self) -> StatusSnapshot {
        let g = self.inner.read().await;
        let mut sessions: Vec<Session> = g.sessions.values().cloned().collect();
        sessions.sort_by(|a, b| b.last_activity_at.cmp(&a.last_activity_at));
        StatusSnapshot {
            daemon_ok: true,
            port: self.port,
            lite_mode: g.lite_mode,
            sources: g.source_health.clone(),
            sessions,
        }
    }

    async fn broadcast(&self) {
        let snap = self.snapshot().await;
        let _ = self.tx.send(snap);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::normalize_raw;
    use serde_json::json;

    #[tokio::test]
    async fn event_sets_active() {
        let store = SessionStore::new(17392);
        let raw = json!({
            "hook_event_name": "preToolUse",
            "session_id": "abc",
            "cwd": "/tmp/proj",
            "tool_name": "Shell"
        });
        store
            .apply_event(normalize_raw(VibeSource::Cursor, &raw))
            .await;
        let snap = store.snapshot().await;
        assert_eq!(snap.sessions.len(), 1);
        assert_eq!(snap.sessions[0].phase, VibePhase::Active);
        assert_eq!(snap.sessions[0].last_tool.as_deref(), Some("Shell"));
    }

    #[tokio::test]
    async fn idle_after_tick() {
        let store = SessionStore::new(17392);
        let raw = json!({
            "hook_event_name": "sessionStart",
            "session_id": "x"
        });
        store
            .apply_event(normalize_raw(VibeSource::Cursor, &raw))
            .await;
        {
            let mut g = store.inner.write().await;
            for s in g.sessions.values_mut() {
                s.last_activity_at = Utc::now() - Duration::seconds(120);
            }
            for h in g.source_health.values_mut() {
                h.last_seen = Some(Utc::now() - Duration::seconds(120));
            }
        }
        store.tick_idle().await;
        let snap = store.snapshot().await;
        assert_eq!(snap.sessions[0].phase, VibePhase::Idle);
    }
}
