# 修复：macOS hook 路径含空格导致 Cursor 不执行

归档日期：2026-05-27

## 问题

用户数据目录在 macOS 为 `~/Library/Application Support/com.VibeMonitor.vibe-monitor/`。安装 hook 时 `hooks.json` 的 `command` 未对路径加引号，Cursor 按 shell 解析会把路径拆断，hook 从未执行；轻量模式仍可用 transcript，故表现为「关轻量不变、开轻量变」。

## 改动

| 文件 | 说明 |
|------|------|
| `crates/vibe-core/src/install.rs` | `hook_command` 统一为双引号包裹可执行路径 |
| `apps/desktop/src-tauri/tauri.conf.json` | `bundle.externalBin` 包含 `binaries/vibe-hook` |
| `install.rs` | 诊断文案去掉错误的 `~/.vibe-monitor` |

## 用户操作

1. 更新 App 或本地 `cargo build` 后，托盘 → **重新安装 hook**（或再次完成向导「启用监听」）。
2. **完全退出并重启 Cursor**（Cmd+Q）。
3. 确认 `~/.cursor/hooks.json` 中 `command` 为带引号形式，例如：`"/Users/.../Application Support/.../vibe-hook" --source cursor`。

## 验证

```bash
echo '{"hook_event_name":"preToolUse","session_id":"test","cwd":"/tmp"}' | \
  "$HOME/Library/Application Support/com.VibeMonitor.vibe-monitor/bin/vibe-hook" --source cursor
curl -s "http://127.0.0.1:$(cat "$HOME/Library/Application Support/com.VibeMonitor.vibe-monitor/port")/api/status"
```

Cursor Agent 触发工具后，`sources.cursor.last_seen` 应更新。
