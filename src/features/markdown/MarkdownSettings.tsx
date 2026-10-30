import { useTranslation } from "react-i18next";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { SegmentedControl } from "@/components/common/SegmentedControl";
import { SettingRow } from "@/components/common/SettingRow";
import { SettingsPanel } from "@/components/common/SettingsPanel";
import type { MarkdownOptions } from "@/lib/tauri/commands";
import { useSettings } from "@/stores/settings-store";

const pageSizes: MarkdownOptions["pageSize"][] = ["a4", "f4", "letter", "legal"];
const fontSizes = ["10", "11", "12", "14"] as const;
// Sempit / Normal / Lebar, kira-kira seperti pengolah kata
const margins = { narrow: 15, normal: 25, wide: 35 } as const;

export function MarkdownSettings() {
  const { t } = useTranslation();
  const { markdown: o, setMarkdown } = useSettings();
  const k = (key: string) => t(`tools.markdown.settings.${key}`);
  const margin = (Object.entries(margins).find(([, mm]) => mm === o.marginMm)?.[0] ?? "normal") as keyof typeof margins;

  return (
    <SettingsPanel>
      <SettingRow label={k("paper")} htmlFor="md-paper">
        <Select value={o.pageSize} onValueChange={(v) => setMarkdown({ pageSize: v as MarkdownOptions["pageSize"] })}>
          <SelectTrigger id="md-paper" className="w-full">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {pageSizes.map((size) => (
              <SelectItem key={size} value={size}>
                {t(`tools.imageToPdf.pageSize.${size}`)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </SettingRow>
      <SettingRow label={k("font")}>
        <SegmentedControl
          aria-label={k("font")}
          value={o.font}
          onChange={(font) => setMarkdown({ font })}
          options={[
            { value: "sans", label: k("sans") },
            { value: "serif", label: k("serif") },
          ]}
        />
      </SettingRow>
      <SettingRow label={k("fontSize")}>
        <SegmentedControl
          aria-label={k("fontSize")}
          value={String(o.fontSize) as (typeof fontSizes)[number]}
          onChange={(size) => setMarkdown({ fontSize: Number(size) })}
          options={fontSizes.map((value) => ({ value, label: `${value} pt` }))}
        />
      </SettingRow>
      <SettingRow label={k("margin")}>
        <SegmentedControl
          aria-label={k("margin")}
          value={margin}
          onChange={(name) => setMarkdown({ marginMm: margins[name] })}
          options={(Object.keys(margins) as (keyof typeof margins)[]).map((value) => ({ value, label: k(`margins.${value}`) }))}
        />
      </SettingRow>
      <SettingRow label={k("pageNumbers")} htmlFor="md-page-numbers" inline>
        <Switch id="md-page-numbers" checked={o.pageNumbers} onCheckedChange={(pageNumbers) => setMarkdown({ pageNumbers })} />
      </SettingRow>
      <details className="group text-xs text-muted-foreground">
        <summary className="cursor-default list-none font-medium text-foreground/80 hover:text-foreground">{k("tipsTitle")}</summary>
        <p className="mt-2 leading-5 whitespace-pre-line select-text">{k("tips")}</p>
      </details>
    </SettingsPanel>
  );
}
