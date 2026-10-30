import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { ChevronRight } from "lucide-react";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { PresetPicker } from "@/components/common/PresetPicker";
import { SegmentedControl } from "@/components/common/SegmentedControl";
import { SettingRow } from "@/components/common/SettingRow";
import { SettingsPanel } from "@/components/common/SettingsPanel";
import type { CompressTarget } from "@/lib/compress-targets";
import type { PageSize } from "@/lib/tauri/commands";
import { useSettings } from "@/stores/settings-store";
import { compressAfterOptions, imageToPdfBuiltInPresets, margins, orientations, pageSizes, qualities } from "./options";

export function ImageToPdfSettings() {
  const { t } = useTranslation();
  const { imageToPdf: o, setImageToPdf, userPresets, addUserPreset, removeUserPreset } = useSettings();
  const k = (group: string, value: string) => t(`tools.imageToPdf.${group}.${value}`);
  const options = <T extends string>(group: string, values: T[]) => values.map((value) => ({ value, label: k(group, value) }));

  return (
    <SettingsPanel>
      <PresetPicker
        builtIn={imageToPdfBuiltInPresets.map((p) => ({
          id: `builtin:${p.id}`,
          label: t(`tools.imageToPdf.builtInPresets.${p.id}`),
          values: p.values,
        }))}
        user={userPresets.imageToPdf.map((p) => ({ id: p.id, label: p.name, values: p.values }))}
        current={o}
        onApply={setImageToPdf}
        onSave={(name) => {
          addUserPreset("imageToPdf", name, o);
          toast.success(t("presets.saved", { name }));
        }}
        onDelete={(id) => removeUserPreset("imageToPdf", id)}
      />
      <SettingRow label={t("tools.imageToPdf.pageSizeLabel")} htmlFor="img-page-size">
        <Select value={o.pageSize} onValueChange={(pageSize) => setImageToPdf({ pageSize: pageSize as PageSize })}>
          <SelectTrigger id="img-page-size" className="w-full">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {pageSizes.map((size) => (
              <SelectItem key={size} value={size}>
                {k("pageSize", size)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </SettingRow>
      <SettingRow
        label={t("tools.imageToPdf.orientationLabel")}
        hint={o.pageSize === "fit" ? t("tools.imageToPdf.orientationFitHint") : undefined}
      >
        <SegmentedControl
          aria-label={t("tools.imageToPdf.orientationLabel")}
          value={o.orientation}
          onChange={(orientation) => setImageToPdf({ orientation })}
          options={options("orientation", orientations)}
          disabled={o.pageSize === "fit"}
        />
      </SettingRow>
      <SettingRow label={t("tools.imageToPdf.scan")} htmlFor="img-scan" hint={t("tools.imageToPdf.scanHint")} inline>
        <Switch id="img-scan" checked={o.scan} onCheckedChange={(scan) => setImageToPdf({ scan })} />
      </SettingRow>
      <SettingRow label={t("tools.imageToPdf.compressAfter")} htmlFor="img-compress" hint={t("tools.imageToPdf.compressAfterHint")}>
        <Select value={o.compressAfter} onValueChange={(v) => setImageToPdf({ compressAfter: v as CompressTarget })}>
          <SelectTrigger id="img-compress" className="w-full">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {compressAfterOptions.map((target) => (
              <SelectItem key={target} value={target}>
                {target === "none" ? t("tools.imageToPdf.compressNone") : t(`tools.compress.targets.${target}`)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </SettingRow>

      {/* Pengaturan lanjutan disembunyikan supaya layar tetap tenang */}
      <details className="group flex flex-col gap-5">
        <summary className="flex cursor-default list-none items-center gap-1 rounded-md text-[13px] font-medium text-muted-foreground hover:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring">
          <ChevronRight className="size-4 transition-transform duration-150 group-open:rotate-90" />
          {t("tools.imageToPdf.more")}
        </summary>
        <div className="mt-4 flex flex-col gap-5">
          <SettingRow label={t("tools.imageToPdf.marginLabel")}>
            <SegmentedControl
              aria-label={t("tools.imageToPdf.marginLabel")}
              value={o.margin}
              onChange={(margin) => setImageToPdf({ margin })}
              options={options("margin", margins)}
            />
          </SettingRow>
          <SettingRow label={t("tools.imageToPdf.qualityLabel")} hint={k("qualityHint", o.quality)}>
            <SegmentedControl
              aria-label={t("tools.imageToPdf.qualityLabel")}
              value={o.quality}
              onChange={(quality) => setImageToPdf({ quality })}
              options={options("quality", qualities)}
            />
          </SettingRow>
        </div>
      </details>
    </SettingsPanel>
  );
}
