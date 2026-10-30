import { useTranslation } from "react-i18next";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { toast } from "sonner";
import { FileText, FolderOpen, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import { fileName, formatRelativeTime } from "@/lib/format";
import { isMac, isWindows } from "@/lib/platform";
import type { HistoryEntry } from "@/stores/history-store";

interface HistoryItemProps {
  entry: HistoryEntry;
  onRemove: () => void;
}

export function HistoryItem({ entry, onRemove }: HistoryItemProps) {
  const { t, i18n } = useTranslation();
  const [first, ...rest] = entry.outputs;
  const revealLabel = t(isMac ? "result.revealMac" : isWindows ? "result.revealWindows" : "result.revealOther");

  async function reveal() {
    try {
      await revealItemInDir(first);
    } catch {
      // File bisa sudah dipindah atau dihapus sejak diproses
      toast.error(t("history.missing", { name: fileName(first) }));
    }
  }

  return (
    <li className="group flex items-center gap-3 rounded-xl bg-card px-4 py-3 shadow-card">
      <FileText className="size-5 shrink-0 text-muted-foreground" />
      <div className="min-w-0 flex-1">
        <p className="truncate text-sm font-medium" title={first}>
          {fileName(first)}
          {rest.length > 0 && <span className="font-normal text-muted-foreground"> {t("history.more", { count: rest.length })}</span>}
        </p>
        <p className="truncate text-xs text-muted-foreground">
          {t(`${entry.toolKey}.title`)} · {formatRelativeTime(entry.at, i18n.language)}
        </p>
      </div>
      <Button variant="ghost" size="sm" onClick={reveal}>
        <FolderOpen />
        {revealLabel}
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        aria-label={t("history.remove")}
        onClick={onRemove}
        className="opacity-0 group-hover:opacity-100 focus-visible:opacity-100"
      >
        <X />
      </Button>
    </li>
  );
}
