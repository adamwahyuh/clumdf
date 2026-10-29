import { useTranslation } from "react-i18next";
import { Upload } from "lucide-react";

/** Overlay saat file di-drag di atas jendela yang sudah berisi file. */
export function DropOverlay() {
  const { t } = useTranslation();

  return (
    <div className="pointer-events-none absolute inset-3 z-20 flex flex-col items-center justify-center gap-3 rounded-xl border-2 border-dashed border-primary bg-background/85 backdrop-blur-sm animate-in fade-in duration-150">
      <Upload className="size-8 text-primary" strokeWidth={1.5} />
      <p className="text-[15px] font-medium">{t("files.dropToAdd")}</p>
    </div>
  );
}
