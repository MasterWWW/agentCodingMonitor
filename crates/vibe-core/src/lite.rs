use crate::event::extract_title_from_prompt;
use crate::store::SessionStore;
use crate::types::VibeSource;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

pub fn spawn_lite_watcher(store: SessionStore) {
    std::thread::spawn(move || {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(e) => {
                tracing::warn!("lite watcher runtime: {e}");
                return;
            }
        };
        if let Err(e) = run_watcher(store, &rt) {
            tracing::warn!("lite watcher stopped: {e}");
        }
    });
}

fn run_watcher(store: SessionStore, rt: &tokio::runtime::Runtime) -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(ev) = res {
                let _ = tx.send(ev);
            }
        },
        Config::default(),
    )?;

    for root in watch_roots() {
        if root.exists() {
            let _ = watcher.watch(&root, RecursiveMode::Recursive);
        }
    }

    loop {
        match rx.recv_timeout(Duration::from_millis(500)) {
            Ok(event) => {
                let lite = rt.block_on(store.get_lite_mode());
                if !lite {
                    continue;
                }
                for path in event.paths {
                    if path.extension().and_then(|s| s.to_str()) != Some("jsonl") {
                        continue;
                    }
                    if let Some(source) = source_for_path(&path) {
                        let (title, cwd) = parse_transcript_tail(&path);
                        rt.block_on(store.apply_lite_activity(source, cwd, title));
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    Ok(())
}

fn watch_roots() -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Some(p) = crate::paths::cursor_transcripts_root() {
        v.push(p);
    }
    if let Some(p) = crate::paths::claude_transcripts_root() {
        v.push(p);
    }
    v
}

fn source_for_path(path: &std::path::Path) -> Option<VibeSource> {
    let s = path.to_string_lossy();
    if s.contains(".cursor/projects") {
        return Some(VibeSource::Cursor);
    }
    if s.contains(".claude/projects") {
        return Some(VibeSource::ClaudeCode);
    }
    if s.contains(".codex") {
        return Some(VibeSource::Codex);
    }
    None
}

fn parse_transcript_tail(path: &std::path::Path) -> (Option<String>, Option<String>) {
    let Ok(content) = std::fs::read_to_string(path) else {
        return (None, None);
    };
    let Some(last) = content.lines().last() else {
        return (None, None);
    };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(last) else {
        return (None, None);
    };
    let mut title = None;
    if v.get("role").and_then(|r| r.as_str()) == Some("user") {
        if let Some(text) = v
            .get("message")
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_array())
            .and_then(|a| a.first())
            .and_then(|x| x.get("text"))
            .and_then(|t| t.as_str())
        {
            title = Some(extract_title_from_prompt(text));
        }
    }
    let cwd = v
        .get("cwd")
        .and_then(|c| c.as_str())
        .map(|s| s.to_string());
    (title, cwd)
}
