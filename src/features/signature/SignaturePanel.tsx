import { useTranslation } from "react-i18next";
import { ImageUp, Stamp } from "lucide-react";
import { Button } from "@/components/ui/button";
import { SettingRow } from "@/components/common/SettingRow";
import { SettingsPanel } from "@/components/common/SettingsPanel";

interface SignaturePanelProps {
  signatureUrl?: string;
  onChoose: () => void;
  /** Tidak ada = belum ada PDF atau tanda tangan */
  onPlace?: () => void;
}

export function SignaturePanel({ signatureUrl, onChoose, onPlace }: SignaturePanelProps) {
  const { t } = useTranslation();

  return (
    <SettingsPanel>
      <SettingRow label={t("tools.signature.yourSignature")} hint={t("tools.signature.signatureHint")}>
        {signatureUrl && (
          // Pola papan catur menunjukkan bagian yang transparan
          <div className="flex h-28 items-center justify-center rounded-lg bg-[conic-gradient(var(--muted)_25%,transparent_0_50%,var(--muted)_0_75%,transparent_0)] bg-[length:16px_16px] p-3 shadow-card">
            <img src={signatureUrl} alt={t("tools.signature.yourSignature")} className="max-h-full max-w-full object-contain" />
          </div>
        )}
        <Button variant="outline" size="sm" onClick={onChoose}>
          <ImageUp />
          {t(signatureUrl ? "tools.signature.replace" : "tools.signature.choose")}
        </Button>
      </SettingRow>
      <Button disabled={!onPlace} onClick={onPlace}>
        <Stamp />
        {t("tools.signature.place")}
      </Button>
      <p className="text-xs text-muted-foreground">{t("tools.signature.moveHint")}</p>
    </SettingsPanel>
  );
}
