import type { StatusSnapshot, VibePhase, VibeSource } from "./types";

const IN_PROGRESS: VibePhase[] = ["active", "waiting_user"];

export function pickHudSource(
  snap: StatusSnapshot | null,
  fallback: VibeSource
): VibeSource {
  if (!snap) return fallback;
  const candidates = snap.sessions.filter((s) =>
    IN_PROGRESS.includes(s.phase)
  );
  if (candidates.length === 0) return fallback;
  return candidates.sort(
    (a, b) =>
      new Date(b.last_activity_at).getTime() -
      new Date(a.last_activity_at).getTime()
  )[0].source;
}
