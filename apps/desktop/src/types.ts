export type VibePhase =
  | "idle"
  | "active"
  | "waiting_user"
  | "stopped"
  | "unknown";

export type VibeSource = "cursor" | "claude_code" | "codex";

export interface Session {
  source: VibeSource;
  session_id: string;
  cwd?: string;
  task_title?: string;
  last_tool?: string;
  last_activity_at: string;
  phase: VibePhase;
}

export interface SourceHealth {
  hook_installed: boolean;
  last_seen?: string;
  phase: VibePhase;
  note?: string;
}

export interface StatusSnapshot {
  daemon_ok: boolean;
  port: number;
  lite_mode: boolean;
  sources: Record<VibeSource, SourceHealth>;
  sessions: Session[];
}

export interface DoctorReport {
  daemon_ok: boolean;
  port: number;
  hook_binary_installed: boolean;
  cursor_hook: boolean;
  claude_hook: boolean;
  codex_hook: boolean;
  codex_hooks_feature?: boolean;
  lite_mode: boolean;
  messages: string[];
}

export const SOURCE_LABELS: Record<VibeSource, string> = {
  cursor: "Cursor",
  claude_code: "Claude Code",
  codex: "Codex",
};
