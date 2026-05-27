# 主浮窗透明磨砂底

归档日期：2026-05-25（2026-05-25 更新：整窗圆角 + 减弱磨砂）

## 范围

仅 **main** 浮窗；**wizard** 向导窗口保持普通不透明背景。

## 实现

| 层级 | 文件 | 说明 |
|------|------|------|
| 窗口 | `tauri.conf.json` | `transparent: true`, `macOSPrivateApi: true`（macOS 真透明必需） |
| 原生 | `lib.rs` | `set_background_color(0,0,0,0)` + `set_shadow(false)`；**仅** `set_effects` 单层 |
| 样式 | `styles.css` / `index.html` | `html/body/#root` 与 `.app` 同为 12px 圆角 + `overflow: hidden` |
| 自适应尺寸 | `useMainWindowAutoSize.ts` | `ResizeObserver` + `setSize(offsetWidth, offsetHeight)` |

## 平台

- **macOS**：`Effect::Popover` + `radius(12)`（不用 `HudWindow`，避免双层 `apply_vibrancy` 过糊）
- **Windows**：acrylic `(18,18,18,80)`；失败时 CSS 兜底
- **其他**：仅 CSS 半透明面板

## 圆角与磨砂要点

1. 勿再手写 `window_vibrancy::apply_vibrancy`：`set_effects` 内部已调用，重复会叠两层磨砂且首层无 `radius`。
2. WebView 根节点必须 `border-radius` + `overflow: hidden`，否则窗口四角露出方形磨砂区。
3. 窗口逻辑尺寸与 `.app` 外框（含 border）一致，避免透明边带。

## 验证

```bash
cd apps/desktop && pnpm run tauri dev
```

完全退出后重启；主浮窗四角圆角、磨砂弱于旧版 HudWindow 双层，仍可拖动。
