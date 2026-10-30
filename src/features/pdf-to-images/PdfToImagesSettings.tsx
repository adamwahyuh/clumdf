import { useTranslation } from "react-i18next";
import { SegmentedControl } from "@/components/common/SegmentedControl";
import { SettingRow } from "@/components/common/SettingRow";
import { SettingsPanel } from "@/components/common/SettingsPanel";
import { useSettings } from "@/stores/settings-store";

const formats = ["jpg", "png"] as const;
// Layar, standar, cetak, cetak tajam
const dpis = ["72", "150", "300", "600"] as const;

export function PdfToImagesSettings() {
  const { t } = useTranslation();
  const { pdfToImages: s, setPdfToImages } = useSettings();

  return (
    <SettingsPanel>
      <SettingRow label={t("tools.pdfToImages.formatLabel")} hint={t(`tools.pdfToImages.formatHints.${s.format}`)}>
        <SegmentedControl
          aria-label={t("tools.pdfToImages.formatLabel")}
          value={s.format}
          onChange={(format) => setPdfToImages({ format })}
          options={formats.map((value) => ({ value, label: value.toUpperCase() }))}
        />
      </SettingRow>
      <SettingRow label={t("tools.pdfToImages.dpiLabel")} hint={t(`tools.pdfToImages.dpiHints.${s.dpi}`)}>
        <SegmentedControl
          aria-label={t("tools.pdfToImages.dpiLabel")}
          value={String(s.dpi) as (typeof dpis)[number]}
          onChange={(dpi) => setPdfToImages({ dpi: Number(dpi) })}
          options={dpis.map((value) => ({ value, label: value }))}
        />
      </SettingRow>
    </SettingsPanel>
  );
}
