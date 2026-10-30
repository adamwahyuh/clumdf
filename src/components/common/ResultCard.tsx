import { useEffect, useRef, type ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { CircleCheck, FolderOpen, RotateCcw } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useActiveTool } from "@/app/active-tool";
import { isMac, isWindows } from "@/lib/platform";
import { useHistory } from "@/stores/history-store";

interface ResultCardProps {
  title: string;
  details: ReactNode;
  /** File hasil yang ditampilkan di Finder/Explorer */
  outputPath: string;
  /** Semua file hasil untuk riwayat; bawaan `[outputPath]` */
  outputs?: string[];
  onReset: () => void;
}

/** Kartu hasil: centang hijau, ringkasan, "Tampilkan di Finder" dan "Proses lagi". */
export function ResultCard({ title, details, outputPath, outputs, onReset }: ResultCardProps) {
  const { t } = useTranslation();
  const toolKey = useActiveTool();
  const recorded = useRef(false);

  // Kartu hasil muncul tepat sekali per pekerjaan selesai: tempat yang pas untuk mencatat riwayat.
  // Ref mencegah catatan ganda saat efek dijalankan dua kali (StrictMode).
  useEffect(() => {
    if (recorded.current || !toolKey) return;
    recorded.current = true;
    useHistory.getState().add({ toolKey, outputs: outputs ?? [outputPath] });
  }, [toolKey, outputs, outputPath]);
  const revealLabel = t(isMac ? "result.revealMac" : isWindows ? "result.revealWindows" : "result.revealOther");

  return (
    <div className="flex min-h-full items-center justify-center py-4 animate-in fade-in zoom-in-95 duration-200">
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
