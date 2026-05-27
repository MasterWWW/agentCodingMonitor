# Contributing

1. Fork 并创建分支。
2. `cargo test -p vibe-core` 与 `cd apps/desktop && npm run build` 通过后再提 PR。
3. 新 hook 事件或来源请更新 `docs/architecture.md` 与 README 支持矩阵。
4. 不要提交含密钥的 transcript 或 `.env` 文件。

## 开发提示

- Hook 二进制路径：开发时先 `cargo build -p vibe-hook`，桌面 App 会从 `target/debug/vibe-hook` 安装到用户目录。
- 本地 API 默认端口 `17392`，冲突时自动递增；端口写入数据目录下的 `port` 文件。
