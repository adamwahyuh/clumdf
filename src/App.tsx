import { useState } from "react";
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
  const ActiveView = tools.find((tool) => tool.id === activeId)!.view;

  return (
    <TooltipProvider>
      <AppShell activeId={activeId} onSelect={setActiveId}>
        {/* key: reset state layar saat pindah fitur */}
        <ActiveView key={activeId} />
      </AppShell>
      <Toaster />
    </TooltipProvider>
  );
}
