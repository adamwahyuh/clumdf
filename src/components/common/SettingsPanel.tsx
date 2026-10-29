import type { ReactNode } from "react";

/** Panel pengaturan di kanan area konten (±280px). */
export function SettingsPanel({ children }: { children: ReactNode }) {
  return (
    <aside className="flex w-[280px] shrink-0 flex-col gap-5 overflow-y-auto border-l bg-muted/40 p-5">
      {children}
    </aside>
  );
}
