use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VibeSource {
    Cursor,
    #[serde(alias = "claude")]
    ClaudeCode,
    Codex,
}

impl VibeSource {
    pub fn all() -> [VibeSource; 3] {
        [
            VibeSource::Cursor,
            VibeSource::ClaudeCode,
            VibeSource::Codex,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            VibeSource::Cursor => "Cursor",
            VibeSource::ClaudeCode => "Claude Code",
            VibeSource::Codex => "Codex",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VibePhase {
    Idle,
    Active,
    WaitingUser,
    Stopped,
    Unknown,
}

impl Default for VibePhase {
    fn default() -> Self {
        VibePhase::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub source: VibeSource,
    pub session_id: String,
    pub cwd: Option<String>,
    pub task_title: Option<String>,
    pub last_tool: Option<String>,
    pub last_activity_at: DateTime<Utc>,
    pub phase: VibePhase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceHealth {
    pub hook_installed: bool,
    pub last_seen: Option<DateTime<Utc>>,
    pub phase: VibePhase,
    pub note: Option<String>,
}

impl Default for SourceHealth {
    fn default() -> Self {
        Self {
            hook_installed: false,
            last_seen: None,
            phase: VibePhase::Unknown,
            note: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusSnapshot {
    pub daemon_ok: bool,
    pub port: u16,
    pub lite_mode: bool,
    pub sources: HashMap<VibeSource, SourceHealth>,
    pub sessions: Vec<Session>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedEvent {
    pub source: VibeSource,
    pub event_name: String,
    pub session_id: Option<String>,
    pub cwd: Option<String>,
    pub task_title: Option<String>,
    pub tool_name: Option<String>,
    pub raw_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorReport {
    pub daemon_ok: bool,
    pub port: u16,
    pub hook_binary_installed: bool,
    pub cursor_hook: bool,
    pub claude_hook: bool,
    pub codex_hook: bool,
    pub codex_hooks_feature: Option<bool>,
    pub lite_mode: bool,
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallHooksResult {
    pub ok: bool,
    pub hook_path: String,
    pub messages: Vec<String>,
}
