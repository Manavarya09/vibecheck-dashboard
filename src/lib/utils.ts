export function formatDuration(secs: number): string {
  if (secs < 60) return `${secs}s`;
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m`;
}

export function formatTime(iso: string): string {
  return new Date(iso).toLocaleTimeString([], {
    hour: "2-digit",
    minute: "2-digit",
  });
}

export function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString([], {
    month: "short",
    day: "numeric",
  });
}

export function categoryLabel(cat: string): string {
  const map: Record<string, string> = {
    ai_assisted: "AI-Assisted",
    manual_coding: "Manual Coding",
    non_coding: "Non-Coding",
  };
  return map[cat] ?? cat;
}

export function categoryColor(cat: string): string {
  const map: Record<string, string> = {
    ai_assisted: "var(--primary)",
    manual_coding: "var(--success)",
    non_coding: "var(--text-tertiary)",
  };
  return map[cat] ?? "var(--text-secondary)";
}

export function pct(value: number, total: number): number {
  if (total === 0) return 0;
  return Math.round((value / total) * 100);
}
