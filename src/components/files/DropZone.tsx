import { useTranslation } from "react-i18next";
import type { LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";

interface DropZoneProps {
  icon: LucideIcon;
  /** Daftar format, mis. "PDF" atau "JPG, PNG" */
  formats: string;
  onBrowse: () => void;
  /** Ada = tampilkan pilihan "pilih satu folder" (fitur yang menerima banyak file) */
  onBrowseFolder?: () => void;
  /** true saat file dari OS sedang di-drag di atas jendela */
  active?: boolean;
}

/** Empty state: ajakan menarik file atau klik untuk memilih. */
export function DropZone({ icon: Icon, formats, onBrowse, onBrowseFolder, active }: DropZoneProps) {
  const { t } = useTranslation();

  return (
    <div
      className={cn(
        "group relative flex h-full min-h-64 w-full flex-col items-center justify-center gap-4 rounded-xl border-2 border-dashed p-8",
        "transition-colors duration-200 ease-out",
        active ? "border-primary bg-primary/5" : "border-border hover:border-primary/40 hover:bg-muted/60",
      )}
    >
      {/* Seluruh area bisa diklik; tombol folder di bawah berada di atasnya */}
      <button
        type="button"
        onClick={onBrowse}
        aria-label={t("dropZone.title")}
        className="absolute inset-0 rounded-xl focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
      />
      <span className="pointer-events-none flex size-16 items-center justify-center rounded-2xl bg-primary/10">
        <Icon className="size-8 text-primary" strokeWidth={1.5} />
      </span>
      <span className="pointer-events-none flex flex-col items-center gap-1 text-center">
        <span className="text-[15px] font-medium">{t("dropZone.title")}</span>
        <span className="text-sm text-muted-foreground">{t("dropZone.formats", { formats })}</span>
      </span>
      {onBrowseFolder && (
        <button
          type="button"
          onClick={onBrowseFolder}
          className="relative rounded-md px-2 py-1 text-sm font-medium text-primary hover:underline focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
        >
          {t("dropZone.folder")}
        </button>
      )}
    </div>
  );
}
