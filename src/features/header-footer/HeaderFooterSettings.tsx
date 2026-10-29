import { useTranslation } from "react-i18next";
import { Input } from "@/components/ui/input";
import { SegmentedControl } from "@/components/common/SegmentedControl";
import { SettingRow } from "@/components/common/SettingRow";
import { SettingsPanel } from "@/components/common/SettingsPanel";
import { fontSizeOrder } from "@/lib/stamp-sizes";
import { useSettings, type Align } from "@/stores/settings-store";

const aligns: Align[] = ["left", "center", "right"];

export function HeaderFooterSettings() {
  const { t } = useTranslation();
  const { headerFooter: s, setHeaderFooter } = useSettings();
  const alignOptions = aligns.map((value) => ({ value, label: t(`tools.headerFooter.aligns.${value}`) }));

  return (
    <SettingsPanel>
      <SettingRow label={t("tools.headerFooter.header")} htmlFor="hf-header">
        <Input
          id="hf-header"
          value={s.header}
          onChange={(e) => setHeaderFooter({ header: e.target.value })}
          placeholder={t("tools.headerFooter.headerPlaceholder")}
          autoFocus
        />
        <SegmentedControl
          aria-label={t("tools.headerFooter.headerAlign")}
          value={s.headerAlign}
          onChange={(headerAlign) => setHeaderFooter({ headerAlign })}
          options={alignOptions}
        />
      </SettingRow>
      <SettingRow label={t("tools.headerFooter.footer")} htmlFor="hf-footer" hint={t("tools.headerFooter.placeholdersHint")}>
        <Input
          id="hf-footer"
          value={s.footer}
          onChange={(e) => setHeaderFooter({ footer: e.target.value })}
          placeholder={t("tools.headerFooter.footerPlaceholder")}
        />
        <SegmentedControl
          aria-label={t("tools.headerFooter.footerAlign")}
          value={s.footerAlign}
          onChange={(footerAlign) => setHeaderFooter({ footerAlign })}
          options={alignOptions}
        />
      </SettingRow>
      <SettingRow label={t("stamp.fontSize")}>
        <SegmentedControl
          aria-label={t("stamp.fontSize")}
          value={s.fontSize}
          onChange={(fontSize) => setHeaderFooter({ fontSize })}
          options={fontSizeOrder.map((value) => ({ value, label: t(`stamp.sizes.${value}`) }))}
        />
      </SettingRow>
    </SettingsPanel>
  );
}
