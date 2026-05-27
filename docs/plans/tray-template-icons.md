# 托盘 Template 单色图标

归档日期：2026-05-27（后续见 [`tray-brand-fixed-icon.md`](tray-brand-fixed-icon.md)）

## 变更

| 项 | 说明 |
|----|------|
| 资源 | `tray.png` 固定品牌标；`tray-*.png` 仅脚本生成备用 |
| 代码 | `icon_as_template(true)`；状态走菜单 + tooltip |
| 生成 | `icons/generate-tray-icons.py` |

## 形状对照

| 文件 | phase | 形状 |
|------|-------|------|
| `tray-active.png` | active | 实心圆 |
| `tray-waiting.png` | waiting_user | 缺口圆环 |
| `tray-idle.png` | idle | 细线空心圆 |
| `tray-stopped.png` | stopped | × |
| `tray-unknown.png` | unknown | 空心圆 + 中心点（避免虚线碎像素） |

图标在 88px 画布绘制后 Lanczos 缩至 22px；图形直径约 12px，与系统菜单栏图标视觉重量接近。

## 验证（macOS）

1. 完全退出后重新启动应用（菜单栏模式）。
2. 图标应随系统深浅色菜单栏自动变色，不再是大色块。
3. 切换 Agent 状态或模拟 phase 时，形状应变化（实心 / 环 / × 等）。
