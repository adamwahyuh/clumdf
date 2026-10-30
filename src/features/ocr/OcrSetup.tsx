import { useTranslation } from "react-i18next";
import { openUrl } from "@tauri-apps/plugin-opener";
import { CircleCheck, CircleAlert, ExternalLink, RefreshCw } from "lucide-react";
import { Button } from "@/components/ui/button";
import { SettingsPanel } from "@/components/common/SettingsPanel";
import { isMac, isWindows } from "@/lib/platform";

const WINDOWS_INSTALLER = "https://github.com/UB-Mannheim/tesseract/wiki";

interface OcrSetupProps {
  /** undefined = sedang dicek, null = belum terpasang */
  tesseract: string | null | undefined;
  onRecheck: () => void;
}

/** Status program OCR; bila belum terpasang, tampilkan cara memasangnya sesuai OS. */
export function OcrSetup({ tesseract, onRecheck }: OcrSetupProps) {
  const { t } = useTranslation();

  return (
    <SettingsPanel>
      {tesseract ? (
        <div className="flex gap-2 text-sm">
          <CircleCheck className="mt-0.5 size-4 shrink-0 text-success" />
          <div className="min-w-0">
            <p className="font-medium">{t("tools.ocr.ready")}</p>
            <p className="truncate text-xs text-muted-foreground" title={tesseract}>{tesseract}</p>
          </div>
        </div>
      ) : tesseract === null ? (
        <div className="flex flex-col gap-3 text-sm">
          <div className="flex gap-2">
            <CircleAlert className="mt-0.5 size-4 shrink-0 text-destructive" />
            <p className="font-medium">{t("tools.ocr.missing")}</p>
          </div>
          <p className="text-muted-foreground">{t("tools.ocr.missingHint")}</p>
          {isWindows ? (
            <Button variant="outline" size="sm" onClick={() => openUrl(WINDOWS_INSTALLER)}>
              <ExternalLink />
              {t("tools.ocr.installWindows")}
            </Button>
          ) : (
            <code className="rounded-md bg-muted px-2 py-1.5 text-xs select-text">
              {isMac ? "brew install tesseract" : "sudo apt install tesseract-ocr"}
            </code>
          )}
          <Button variant="ghost" size="sm" onClick={onRecheck}>
            <RefreshCw />
            {t("tools.ocr.recheck")}
          </Button>
        </div>
      ) : null}
      <p className="text-xs text-muted-foreground">{t("tools.ocr.about")}</p>
    </SettingsPanel>
  );
}
