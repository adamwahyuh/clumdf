import { useTranslation } from "react-i18next";
import { SegmentedControl } from "@/components/common/SegmentedControl";
import { useSettings, type Language } from "@/stores/settings-store";

const languages: Language[] = ["id", "en"];

export function LanguageSwitch() {
  const { t } = useTranslation();
  const { language, setLanguage } = useSettings();

  return (
    <SegmentedControl
      aria-label={t("language.label")}
      value={language}
      onChange={setLanguage}
      options={languages.map((value) => ({ value, label: t(`language.${value}`) }))}
    />
  );
}
