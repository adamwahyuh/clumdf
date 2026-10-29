import { useState } from "react";
import { tools } from "@/app/tools";
import { AppShell } from "@/components/layout/AppShell";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";

export default function App() {
  const [activeId, setActiveId] = useState(tools[0].id);
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
