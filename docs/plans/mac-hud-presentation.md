# macOS HUD 展示增强

归档日期：2026-05-26

## 变更摘要

| 能力 | 实现 |
|------|------|
| 切换 Space 仍显示浮窗 | `tauri.conf` `visibleOnAllWorkspaces` + `set_visible_on_all_workspaces` |
| Dock 不常驻 | `bundle.macOS.infoPlist.LSUIElement` + `ActivationPolicy::Accessory` |
| 浮窗 / 菜单栏图标 | `HudPresentation` 持久化；菜单栏固定 `tray.png` Template 品牌标 |

## 关键文件

- [`crates/vibe-core/src/state.rs`](../crates/vibe-core/src/state.rs) — `HudPresentation`、`load_presentation` / `write_presentation`
- [`apps/desktop/src-tauri/src/lib.rs`](../apps/desktop/src-tauri/src/lib.rs) — `apply_presentation`、`refresh_tray_status`、托盘菜单勾选项
- [`apps/desktop/src-tauri/icons/tray.png`](../apps/desktop/src-tauri/icons/tray.png) — 菜单栏品牌 Template 图标
- [`apps/desktop/src-tauri/tauri.conf.json`](../apps/desktop/src-tauri/tauri.conf.json)

## 默认

- macOS：`float`
- 其他平台：`menubar`（与原先 Windows 默认隐藏浮窗一致）

## 手动验证（macOS 打包 `.app`）

1. **浮窗模式**：切换 Space，HUD 仍可见；Dock 无应用图标。
2. **菜单栏模式**：无浮窗；托盘图标固定品牌标；悬停 tooltip / 菜单首行显示状态。
3. 托盘切换展示方式后重启，偏好保留。

说明：`tauri dev` 下 Dock / Info.plist 行为可能与正式包不同，以 Finder 启动的 `.app` 为准。
