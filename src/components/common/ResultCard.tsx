import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { CircleCheck, FolderOpen, RotateCcw } from "lucide-react";
import { Button } from "@/components/ui/button";
import { isMac, isWindows } from "@/lib/platform";

interface ResultCardProps {
  title: string;
  details: ReactNode;
  /** File hasil yang ditampilkan di Finder/Explorer */
  outputPath: string;
  onReset: () => void;
}

/** Kartu hasil: centang hijau, ringkasan, "Tampilkan di Finder" dan "Proses lagi". */
export function ResultCard({ title, details, outputPath, onReset }: ResultCardProps) {
  const { t } = useTranslation();
  const revealLabel = t(isMac ? "result.revealMac" : isWindows ? "result.revealWindows" : "result.revealOther");

  return (
    <div className="flex h-full items-center justify-center animate-in fade-in zoom-in-95 duration-200">
      <div className="flex w-full max-w-md flex-col items-center gap-4 rounded-xl bg-card p-8 text-center shadow-card">
        <CircleCheck className="size-12 text-success" strokeWidth={1.5} />
        <div className="flex flex-col gap-1">
          <h2 className="text-lg font-semibold">{title}</h2>
          <div className="text-sm text-muted-foreground">{details}</div>
        </div>
        <div className="flex gap-2 pt-2">
          <Button variant="outline" onClick={onReset}>
            <RotateCcw />
            {t("result.again")}
          </Button>
          <Button autoFocus onClick={() => revealItemInDir(outputPath)}>
            <FolderOpen />
            {revealLabel}
          </Button>
        </div>
      </div>
    </div>
  );
}
