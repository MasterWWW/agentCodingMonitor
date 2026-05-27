use crate::types::{NormalizedEvent, VibePhase, VibeSource};

const TITLE_MAX: usize = 120;

pub fn redact_title(input: &str) -> String {
    let lower = input.to_lowercase();
    if lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("sk-")
        || lower.contains("password")
        || lower.contains("secret")
    {
        return "[redacted]".to_string();
    }
    let trimmed: String = input.chars().take(TITLE_MAX).collect();
    trimmed.trim().to_string()
}

pub fn extract_title_from_prompt(prompt: &str) -> String {
    let text = prompt
        .replace("<user_query>", "")
        .replace("</user_query>", "");
    redact_title(text.trim())
}

pub fn normalize_raw(source: VibeSource, raw: &serde_json::Value) -> NormalizedEvent {
    let event_name = raw
        .get("hook_event_name")
        .or_else(|| raw.get("event"))
        .or_else(|| raw.get("type"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let session_id = raw
        .get("session_id")
        .or_else(|| raw.get("conversation_id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let cwd = raw
        .get("cwd")
        .or_else(|| raw.get("workspace"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let tool_name = raw
        .get("tool_name")
        .or_else(|| raw.get("tool"))
        .or_else(|| raw.get("tool_type"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let prompt = raw
        .get("prompt")
        .or_else(|| raw.get("user_message"))
        .or_else(|| raw.get("message"))
        .and_then(|v| v.as_str());

    let task_title = prompt.map(extract_title_from_prompt);

    NormalizedEvent {
        source,
        event_name: event_name.clone(),
        session_id,
        cwd,
        task_title,
        tool_name,
        raw_hint: Some(event_name),
    }
}

pub fn phase_for_event(event_name: &str) -> Option<VibePhase> {
    let n = event_name.to_lowercase();
    if n.contains("sessionstart") || n == "sessionstart" {
        return Some(VibePhase::Active);
    }
    if n.contains("pretooluse") || n.contains("posttooluse") {
        return Some(VibePhase::Active);
    }
    if n.contains("beforesubmitprompt")
        || n.contains("userpromptsubmit")
        || n.contains("submitprompt")
    {
        return Some(VibePhase::Active);
    }
    if n == "stop" || n.contains("sessionend") {
        return Some(VibePhase::Stopped);
    }
    if n.contains("afteragentresponse") || n.contains("agentresponse") {
        return Some(VibePhase::WaitingUser);
    }
    None
}

pub fn session_key(source: VibeSource, session_id: &str) -> String {
    let slug = match source {
        VibeSource::Cursor => "cursor",
        VibeSource::ClaudeCode => "claude",
        VibeSource::Codex => "codex",
    };
    format!("{slug}:{session_id}")
}
