import { useTranslation } from "react-i18next";
import { Input } from "@/components/ui/input";
import { SegmentedControl } from "@/components/common/SegmentedControl";
import { SettingRow } from "@/components/common/SettingRow";
import { SettingsPanel } from "@/components/common/SettingsPanel";
import { useSettings, type SplitModeName } from "@/stores/settings-store";

const modes: SplitModeName[] = ["eachPage", "ranges", "everyN"];

interface SplitSettingsProps {
  ranges: string;
  onRangesChange: (ranges: string) => void;
  /** true setelah percobaan dengan rentang yang ditolak */
  rangesInvalid: boolean;
}

export function SplitSettings({ ranges, onRangesChange, rangesInvalid }: SplitSettingsProps) {
  const { t } = useTranslation();
  const { split, setSplit } = useSettings();

  return (
    <SettingsPanel>
      <SettingRow label={t("tools.split.modeLabel")}>
        <SegmentedControl
          aria-label={t("tools.split.modeLabel")}
          value={split.mode}
          onChange={(mode) => setSplit({ mode })}
          options={modes.map((value) => ({ value, label: t(`tools.split.modes.${value}`) }))}
        />
      </SettingRow>
      {split.mode === "ranges" && (
        <SettingRow label={t("tools.split.rangesLabel")} htmlFor="split-ranges" hint={t("tools.split.rangesHint")}>
          <Input
            id="split-ranges"
            value={ranges}
            onChange={(e) => onRangesChange(e.target.value)}
            placeholder="1-3, 5, 8-10"
            aria-invalid={rangesInvalid || undefined}
            autoFocus
          />
        </SettingRow>
      )}
      {split.mode === "everyN" && (
        <SettingRow label={t("tools.split.everyNLabel")} htmlFor="split-n">
          <Input
            id="split-n"
            type="number"
            min={1}
            value={split.n}
            onChange={(e) => setSplit({ n: Math.max(1, Math.floor(Number(e.target.value)) || 1) })}
          />
        </SettingRow>
      )}
    </SettingsPanel>
  );
}
