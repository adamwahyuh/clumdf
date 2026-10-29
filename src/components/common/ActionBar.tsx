import type { ReactNode } from "react";

interface ActionBarProps {
  /** Ringkasan di kiri, mis. "3 file · 12 halaman" */
  summary: ReactNode;
  /** Tombol utama (dan tombol sekunder bila ada) di kanan */
  children: ReactNode;
}

export function ActionBar({ summary, children }: ActionBarProps) {
  return (
    <footer className="flex h-14 shrink-0 items-center justify-between gap-4 border-t bg-background/80 px-6 backdrop-blur">
      <div className="min-w-0 truncate text-sm text-muted-foreground">{summary}</div>
      <div className="flex shrink-0 items-center gap-2">{children}</div>
    </footer>
  );
}
