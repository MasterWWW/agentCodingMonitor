# 状态栏托盘：从「固定图标」到「状态联动内容」

归档日期：2026-05-27（取代 [`tray-brand-fixed-icon.md`](tray-brand-fixed-icon.md)）

## 背景

此前 `tray-brand-fixed-icon.md` 选择**固定品牌图标**，状态信息全部走菜单首行 + 悬停 tooltip。用户反馈：状态栏目前只是一个静态图标，希望改成可以**和状态联动的内容**，让 phase / 源在菜单栏第一眼就能看到。

## 定稿

托盘同时承载两层「联动内容」：

| 层 | 行为 | 平台 |
|----|------|------|
| 图标 | 按 phase 切换 `tray-active.png` / `tray-waiting.png` / `tray-idle.png` / `tray-stopped.png` / `tray-unknown.png` | macOS / Linux / Windows |
| 标题文本 | `Active` / `WaitingUser` 时展示「`Source · 状态`」（如 `Cursor · 进行中`），其余状态留空避免占位 | macOS / Linux（Windows 不支持 `set_title`） |
| Tooltip | 维持现有 `status_line` 完整文案 | 全平台 |

显示源仍走 `state::pick_display_source`，与浮窗 HUD、菜单首行保持同一选择策略。

## 关键改动

`apps/desktop/src-tauri/src/lib.rs`：

1. 新增 `tray_icon_for_phase(phase)`，按 phase 查找 `icons/tray-*.png`，失败回退到 `tray.png` / `icon.png`。
2. 新增 `tray_status_title(snap)` 与可测的 `tray_status_title_for(snap, default)`：仅在 `Active` / `WaitingUser` 时返回 `Some(...)`。
3. `refresh_tray_status` 在更新 tooltip 之外，调用 `set_icon` + `set_icon_as_template(true)` + `set_title`。
4. `setup_tray` 初始图标改为 `tray_icon_for_phase(VibePhase::Unknown)`，避免启动瞬间从品牌图闪到状态图。
5. 新增 `ClaudeCode → "Claude"` 的短标签（仅托盘标题使用），保持菜单栏紧凑。

## 测试

`cargo test -p vibe-monitor --lib`：

- `title_shows_source_and_phase_when_active`
- `title_shows_when_waiting_user`
- `title_hidden_when_idle_or_stopped_or_unknown`
- `title_uses_claude_short_label`
- `icon_resolves_for_every_phase`

## 备注

- 图标资源沿用 `icons/generate-tray-icons.py` 生成的 5 个 phase Template PNG，无需重新生成。
- Windows 上 `set_title` 在 tauri-2.11 中为 no-op；状态可见性完全由图标承担，因此图标切换是「全平台保底通道」。
- 若后续希望禁用菜单栏文字（仅留图标），可让 `tray_status_title` 始终返回 `None`，无需调整图标逻辑。
