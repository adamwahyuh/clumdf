import { useTranslation } from "react-i18next";
import { Input } from "@/components/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { SegmentedControl } from "@/components/common/SegmentedControl";
import { SettingRow } from "@/components/common/SettingRow";
import { SettingsPanel } from "@/components/common/SettingsPanel";
import { useSettings, type PageNumberSettings } from "@/stores/settings-store";
import { fontSizeOrder } from "@/lib/stamp-sizes";
import { PositionPicker } from "./PositionPicker";

const formats: PageNumberSettings["format"][] = ["plain", "page", "ofTotal", "pageOfTotal"];

export function PageNumbersSettings() {
  const { t } = useTranslation();
  const { pageNumbers: s, setPageNumbers } = useSettings();
  // Contoh tampilan format memakai halaman 3 dari 10
  const example = (format: string) =>
    t(`tools.pageNumbers.templates.${format}`).replace("{page}", "3").replace("{total}", "10");

  return (
    <SettingsPanel>
      <SettingRow label={t("tools.pageNumbers.formatLabel")} htmlFor="pn-format">
        <Select value={s.format} onValueChange={(format) => setPageNumbers({ format: format as PageNumberSettings["format"] })}>
          <SelectTrigger id="pn-format" className="w-full">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {formats.map((format) => (
              <SelectItem key={format} value={format}>
                {example(format)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </SettingRow>
      <SettingRow label={t("tools.pageNumbers.positionLabel")}>
        <PositionPicker
          aria-label={t("tools.pageNumbers.positionLabel")}
          value={s.position}
          onChange={(position) => setPageNumbers({ position })}
        />
      </SettingRow>
      <SettingRow label={t("stamp.fontSize")}>
        <SegmentedControl
          aria-label={t("stamp.fontSize")}
          value={s.fontSize}
          onChange={(fontSize) => setPageNumbers({ fontSize })}
          options={fontSizeOrder.map((value) => ({ value, label: t(`stamp.sizes.${value}`) }))}
        />
      </SettingRow>
      <SettingRow label={t("tools.pageNumbers.firstNumber")} htmlFor="pn-first">
        <Input
          id="pn-first"
          type="number"
          min={0}
          value={s.firstNumber}
          onChange={(e) => setPageNumbers({ firstNumber: Math.max(0, Math.floor(Number(e.target.value)) || 0) })}
        />
      </SettingRow>
      <SettingRow label={t("tools.pageNumbers.skipFirst")} htmlFor="pn-skip" hint={t("tools.pageNumbers.skipFirstHint")} inline>
        <Switch id="pn-skip" checked={s.skipFirst} onCheckedChange={(skipFirst) => setPageNumbers({ skipFirst })} />
      </SettingRow>
    </SettingsPanel>
  );
}
