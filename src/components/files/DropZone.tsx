import { useTranslation } from "react-i18next";
import type { LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";

interface DropZoneProps {
  icon: LucideIcon;
  /** Daftar format, mis. "PDF" atau "JPG, PNG" */
  formats: string;
  onBrowse: () => void;
  /** true saat file dari OS sedang di-drag di atas jendela */
  active?: boolean;
}

/** Empty state: ajakan menarik file atau klik untuk memilih. */
export function DropZone({ icon: Icon, formats, onBrowse, active }: DropZoneProps) {
  const { t } = useTranslation();

  return (
    <button
      type="button"
      onClick={onBrowse}
      className={cn(
        "flex h-full min-h-64 w-full flex-col items-center justify-center gap-4 rounded-xl border-2 border-dashed p-8",
        "transition-colors duration-200 ease-out focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none",
        active
          ? "border-primary bg-primary/5"
          : "border-border hover:border-primary/40 hover:bg-muted/60",
      )}
    >
      <span className="flex size-16 items-center justify-center rounded-2xl bg-primary/10">
        <Icon className="size-8 text-primary" strokeWidth={1.5} />
      </span>
      <span className="flex flex-col items-center gap-1">
        <span className="text-[15px] font-medium">{t("dropZone.title")}</span>
        <span className="text-sm text-muted-foreground">{t("dropZone.formats", { formats })}</span>
      </span>
    </button>
  );
}
