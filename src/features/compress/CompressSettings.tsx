import { useTranslation } from "react-i18next";
import { Switch } from "@/components/ui/switch";
import { SegmentedControl } from "@/components/common/SegmentedControl";
import { SettingRow } from "@/components/common/SettingRow";
import { SettingsPanel } from "@/components/common/SettingsPanel";
import { useSettings } from "@/stores/settings-store";
import { compressPresetOrder } from "./presets";

export function CompressSettings() {
  const { t } = useTranslation();
  const { compress, setCompress } = useSettings();

  return (
    <SettingsPanel>
      <SettingRow label={t("tools.compress.level")} hint={t(`tools.compress.presets.${compress.preset}.hint`)}>
        <SegmentedControl
          aria-label={t("tools.compress.level")}
          value={compress.preset}
          onChange={(preset) => setCompress({ preset })}
          options={compressPresetOrder.map((value) => ({
            value,
            label: t(`tools.compress.presets.${value}.label`),
          }))}
        />
      </SettingRow>
      <SettingRow label={t("tools.compress.grayscale")} htmlFor="compress-grayscale" hint={t("tools.compress.grayscaleHint")} inline>
        <Switch
          id="compress-grayscale"
          checked={compress.grayscale}
          onCheckedChange={(grayscale) => setCompress({ grayscale })}
        />
      </SettingRow>
    </SettingsPanel>
  );
}
