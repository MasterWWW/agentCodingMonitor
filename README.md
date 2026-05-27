# Vibe Monitor

跨平台桌面浮窗/托盘工具，监听 **Cursor**、**Claude Code**、**OpenAI Codex** 是否处于 Agent 式 vibe coding，并显示当前任务摘要。

数据仅通过 `127.0.0.1` 本地通信，不上传云端。

## 支持矩阵

| 能力 | macOS | Windows |
|------|-------|---------|
| 置顶浮窗 / 托盘 | 浮窗 + 托盘 | 托盘 + 可选浮窗 |
| Cursor hooks | 是 | 是（`vibe-hook.cmd`） |
| Claude Code hooks | 是 | 是 |
| Codex hooks | 是（需 `codex_hooks`） | 可能受限 |
| 轻量 transcript 模式 | 是 | 是 |

## 开箱即用

1. 安装 Release 或从源码构建（见下）。
2. 首次启动点击 **「启用监听」**（自动安装 `~/.vibe-monitor/bin/vibe-hook` 并写入三端 hook 配置）。
3. 之后无需配置；浮窗/托盘通过 SSE 自动刷新。

## 从源码构建

需要：Rust、Node 18+、pnpm 或 npm。

在**仓库根目录**执行：

```bash
# 必须先构建 hook（向导「启用监听」需要这个二进制）
cargo build -p vibe-hook

# 构建 hook 与 core（发布）
cargo build -p vibe-hook -p vibe-core --release

# 前端 + 桌面安装包
cd apps/desktop
npm install
npm run tauri build
```

开发模式（同样先在根目录 `cargo build -p vibe-hook`）：

```bash
cd apps/desktop && npm install && npm run tauri dev
```

## 架构

- `crates/vibe-core` — 本地 HTTP API、会话状态、hook 安装、轻量文件监视
- `crates/vibe-hook` — 三端 hook 调用的上报二进制
- `apps/desktop` — Tauri 2 + React UI

详见 [docs/architecture.md](docs/architecture.md)。

## 隐私

- Hook 上报截断后的任务标题，不记录完整 prompt。
- 状态文件位于系统数据目录（macOS/Linux：`~/.local/share` 或平台等效；Windows：`%APPDATA%`）。

## 许可证

MIT — 见 [LICENSE](LICENSE)。
