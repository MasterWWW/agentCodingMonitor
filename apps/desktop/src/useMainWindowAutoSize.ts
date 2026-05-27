import { LogicalSize } from "@tauri-apps/api/dpi";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { useEffect, type RefObject } from "react";

/** Resize the main HUD window to match .app outer box (logical px, incl. border). */
export function useMainWindowAutoSize(
  rootRef: RefObject<HTMLElement | null>,
  deps: unknown[] = []
) {
  useEffect(() => {
    const win = getCurrentWebviewWindow();
    if (win.label !== "main") return;

    const el = rootRef.current;
    if (!el) return;

    let frame = 0;
    const apply = () => {
      cancelAnimationFrame(frame);
      frame = requestAnimationFrame(() => {
        const width = Math.ceil(el.offsetWidth);
        const height = Math.ceil(el.offsetHeight);
        if (width < 1 || height < 1) return;
        void win.setSize(new LogicalSize(width, height));
      });
    };

    const ro = new ResizeObserver(apply);
    ro.observe(el);
    apply();

    return () => {
      cancelAnimationFrame(frame);
      ro.disconnect();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, deps);
}
