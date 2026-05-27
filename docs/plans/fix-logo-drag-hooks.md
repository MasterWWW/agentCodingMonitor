# 修复：Logo / 浮窗拖动 / Hook 状态同步

归档日期：2026-05-25

## 问题

1. 替换 `src-tauri/icons/` 后菜单栏 logo 不更新，且出现两个托盘图标。
2. 无边框浮窗无法拖动。
3. Cursor 已配置 hook 仍显示「未配置 hook / 未知」；轻量模式重启后丢失。

## 改动摘要

| 区域 | 文件 | 说明 |
|------|------|------|
| 托盘 | `tauri.conf.json` | 移除 `app.trayIcon`，避免与代码重复创建 |
| 托盘 | `lib.rs` | `setup_tray` 从 `icons/icon.png` 加载，`icon_as_template(false)` |
| 托盘 | `Cargo.toml` | `tauri` 启用 `image-png` |
| 拖动 | `App.tsx` | `header` 增加 `data-tauri-drag-region` |
| 拖动 | `capabilities/default.json` | `core:window:allow-start-dragging` |
| 状态 | `vibe-core/state.rs` | `state.json` 持久化 `lite_mode`；macOS 默认开启 |
| Hook | `install.rs` | `sync_hook_health_from_disk`；安装成功提示重启 Cursor |
| Hook | `server.rs` | 启动时同步磁盘 hook 配置 |
| Hook | `vibe-hook` | POST 失败时 `eprintln` 日志 |
| UI | `App.tsx` | 已配置 hook 且 unknown 时显示「等待活动（已配置 hook）」 |

## 用户验证

1. 完全退出 Vibe Monitor 后重新 `pnpm run tauri dev`。
2. 菜单栏应只剩一个图标，且为新 logo。
3. 拖浮窗标题栏可移动窗口。
4. 重启应用后 Cursor 行不应再误报「未配置 hook」。
5. 完全退出并重新打开 Cursor，在 Agent 中触发工具后状态应变更为进行中。
6. 可选：开轻量模式，有 transcript 写入时可见 lite 活动。

## 手动测试 hook

```bash
echo '{"hook_event_name":"preToolUse","session_id":"test","cwd":"/tmp","tool_name":"Shell"}' | \
  "$HOME/Library/Application Support/com.VibeMonitor.vibe-monitor/bin/vibe-hook" --source cursor
```
