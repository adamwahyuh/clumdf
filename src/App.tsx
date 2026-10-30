import { useState } from "react";
import { ActiveToolContext } from "@/app/active-tool";
import { tools } from "@/app/tools";
import { OpenWithDialog } from "@/components/layout/OpenWithDialog";
import { useOsOpenedFiles } from "@/hooks/use-os-opened-files";
import { useOpenedFiles } from "@/stores/opened-files-store";
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
  const opened = useOsOpenedFiles();
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
      <OpenWithDialog
        paths={opened.pending}
        onClose={opened.clear}
        onChoose={(tool) => {
          // Layar fitur mengambil file ini lewat useFileInput
          useOpenedFiles.getState().setHandoff(opened.pending);
          setActiveId(tool.id);
          opened.clear();
        }}
      />
      <Toaster />
    </TooltipProvider>
  );
}
