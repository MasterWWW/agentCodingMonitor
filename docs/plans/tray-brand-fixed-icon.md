# 菜单栏托盘图标 — 定稿

归档日期：2026-05-27

## 定稿结论

菜单栏使用**固定品牌标**，状态不在图标上切换；与微信等原生图标对齐，采用**实心色块**而非细线/V 字。

| 项 | 定稿 |
|----|------|
| 文件 | `apps/desktop/src-tauri/icons/tray.png` |
| 画布 | 22×22 px（菜单栏 extra 工作区） |
| 图形 | 自 **`icon.png`** 中心裁剪 → 简化 Template（显示器 + V，与 App Logo 一致） |
| 颜色 | Template：纯黑 + Alpha（系统随深浅色菜单栏着色） |
| 运行时 | `lib.rs` 仅加载 `tray.png`，`icon_as_template(true)` |
| 状态 | 托盘菜单首行「当前 · …」+ 悬停 tooltip；浮窗 HUD 仍用 phase 彩色主题 |

## 图形来源

`generate-tray-icons.py` 从 `icon.png` 做中心裁剪（去掉周围手写文字），阈值转纯黑 + Alpha，缩放到 19×19 glyph 区。缺文件时回退为矢量「显示器 + V + 电源圆点」。

## 技术约束（勿再改画布比例）

```
显示宽度(pt) = png宽度 × 18 ÷ png高度
```

- 保持 **22×22 正方形**；勿用加宽画布（会撑大菜单栏占位）。
- 调大小改 `GLYPH`（当前 19）；裁剪范围改 `LOGO_CROP_FRAC`（当前 0.30）。

## 生成

```bash
python3 apps/desktop/src-tauri/icons/generate-tray-icons.py
```

## 示意

菜单栏显示为 Template 单色；源稿为 App `icon.png` 中的显示器 + V 标识（非圆角方块占位符）。

## 验证

1. 完全退出 Vibe Monitor 后重启。
2. 菜单栏图标为稳定 Logo 造型（显示器 + V），不随 phase 变化。
3. 与相邻图标高度接近；悬停/右键可见状态文案。

## 参考

- [Designing macOS menu bar extras](https://bjango.com/articles/designingmenubarextras/)
- `tray-icon` 0.23 `platform_impl/macos/mod.rs` — `icon_height: 18.0`
