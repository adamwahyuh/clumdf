import type { ReactNode } from "react";
import { Sidebar } from "./Sidebar";

interface AppShellProps {
  activeId: string;
  onSelect: (id: string) => void;
  children: ReactNode;
}

export function AppShell({ activeId, onSelect, children }: AppShellProps) {
  return (
    <div className="flex h-full overflow-hidden">
      <Sidebar activeId={activeId} onSelect={onSelect} />
      <main className="relative flex min-w-0 flex-1 flex-col">{children}</main>
    </div>
  );
}
