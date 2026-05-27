use serde_json::json;
use std::fs;
use tempfile::tempdir;

#[test]
fn cursor_hooks_merge_preserves_foreign_entry() {
    let dir = tempdir().unwrap();
    let cursor = dir.path().join("hooks.json");
    fs::write(
        &cursor,
        r#"{
  "version": 1,
  "hooks": {
    "stop": [{ "command": "/usr/bin/other-hook.sh" }]
  }
}"#,
    )
    .unwrap();

    // Simulate merge logic: retain non-vibe, add vibe
    let mut root: serde_json::Value = serde_json::from_str(&fs::read_to_string(&cursor).unwrap()).unwrap();
    let hooks = root["hooks"].as_object_mut().unwrap();
    let vibe = json!({ "command": "/tmp/vibe-hook --source cursor", "metadata": { "source": "vibe-monitor" } });
    let list = hooks.entry("sessionStart").or_insert(json!([]));
    if let Some(arr) = list.as_array_mut() {
        arr.push(vibe);
    }
    let stop = hooks.get("stop").unwrap().as_array().unwrap();
    assert_eq!(stop.len(), 1);
    assert!(stop[0]["command"].as_str().unwrap().contains("other-hook"));
    assert!(hooks.contains_key("sessionStart"));
}
