# idle 超时改为 30 秒

归档日期：2026-05-26

## 变更

| 项 | 原值 | 新值 |
|----|------|------|
| `IDLE_AFTER_SECS` | 90 | 30 |

位置：[`crates/vibe-core/src/store.rs`](../crates/vibe-core/src/store.rs) 中 `tick_idle()` 使用。

## 原因

Cursor Agent 自然结束后往往无新 hook，状态会长时间停在 `active`（绿色）。90 秒体感偏慢；30 秒在「尽快变灰」与「长 Shell 误报」之间折中。

## 预期体感

- `tick_idle` 仍每 **15s** 扫描一次。
- 无新事件时，约 **30～45s** 内 HUD 由绿变灰（`idle`）。
- 无需重新 `install-hooks`；重启 Vibe Monitor 即生效。

## 未纳入

- 缩短 `tick_idle` 间隔（可后续单独改）
- 安装 `afterAgentResponse` hook（结束即变黄）
