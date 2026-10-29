import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { Input } from "@/components/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { PresetPicker } from "@/components/common/PresetPicker";
import { SegmentedControl } from "@/components/common/SegmentedControl";
import { SettingRow } from "@/components/common/SettingRow";
import { SettingsPanel } from "@/components/common/SettingsPanel";
import { useSettings } from "@/stores/settings-store";
import { compressBuiltInPresets, compressPresetOrder, compressTargetOrder, type CompressTarget } from "./presets";

export function CompressSettings() {
  const { t } = useTranslation();
  const { compress, setCompress, userPresets, addUserPreset, removeUserPreset } = useSettings();
  const hasTarget = compress.target !== "none";

  return (
    <SettingsPanel>
      <PresetPicker
        builtIn={compressBuiltInPresets.map((p) => ({
          id: `builtin:${p.id}`,
          label: t(`tools.compress.builtInPresets.${p.id}`),
          values: p.values,
        }))}
        user={userPresets.compress.map((p) => ({ id: p.id, label: p.name, values: p.values }))}
        current={compress}
        onApply={setCompress}
        onSave={(name) => {
          addUserPreset("compress", name, compress);
          toast.success(t("presets.saved", { name }));
        }}
        onDelete={(id) => removeUserPreset("compress", id)}
      />
      <SettingRow
        label={t("tools.compress.level")}
        hint={t(hasTarget ? "tools.compress.levelAuto" : `tools.compress.presets.${compress.preset}.hint`)}
      >
        <SegmentedControl
          aria-label={t("tools.compress.level")}
          value={compress.preset}
          onChange={(preset) => setCompress({ preset })}
          options={compressPresetOrder.map((value) => ({
            value,
            label: t(`tools.compress.presets.${value}.label`),
          }))}
          disabled={hasTarget}
        />
      </SettingRow>
      <SettingRow label={t("tools.compress.targetLabel")} htmlFor="compress-target">
        <Select value={compress.target} onValueChange={(target) => setCompress({ target: target as CompressTarget })}>
          <SelectTrigger id="compress-target" className="w-full">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {compressTargetOrder.map((target) => (
              <SelectItem key={target} value={target}>
                {t(`tools.compress.targets.${target}`)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </SettingRow>
      {compress.target === "custom" && (
        <SettingRow label={t("tools.compress.customLabel")} htmlFor="compress-custom">
          <Input
            id="compress-custom"
            type="number"
            min={0.05}
            step={0.1}
            value={compress.customMb}
            onChange={(e) => setCompress({ customMb: Number(e.target.value) || 0 })}
          />
        </SettingRow>
      )}
      <SettingRow
        label={t("tools.compress.grayscale")}
        htmlFor="compress-grayscale"
        hint={t("tools.compress.grayscaleHint")}
        inline
      >
        <Switch
          id="compress-grayscale"
          checked={compress.grayscale}
          onCheckedChange={(grayscale) => setCompress({ grayscale })}
        />
      </SettingRow>
    </SettingsPanel>
  );
}
