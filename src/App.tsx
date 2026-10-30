import { useState } from "react";
import { ActiveToolContext } from "@/app/active-tool";
import { tools } from "@/app/tools";
import { useHotkey } from "@/hooks/use-hotkey";
import { AppShell } from "@/components/layout/AppShell";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";

export default function App() {
  const [activeId, setActiveId] = useState(tools[0].id);
  // ⌘/Ctrl + 1..9 pindah fitur sesuai urutan sidebar
  useHotkey(["1", "2", "3", "4", "5", "6", "7", "8", "9"], (key) => {
    const tool = tools[Number(key) - 1];
    if (tool) setActiveId(tool.id);
  });
  const active = tools.find((tool) => tool.id === activeId)!;
  const ActiveView = active.view;

  return (
    <TooltipProvider>
      <AppShell activeId={activeId} onSelect={setActiveId}>
        {/* key: reset state layar saat pindah fitur */}
        <ActiveToolContext.Provider value={active.i18nKey}>
          <ActiveView key={activeId} />
        </ActiveToolContext.Provider>
      </AppShell>
      <Toaster />
    </TooltipProvider>
  );
}
