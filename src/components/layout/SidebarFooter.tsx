import { useTranslation } from "react-i18next";
import { openUrl } from "@tauri-apps/plugin-opener";
import version from "../../../VERSION?raw";
import { LanguageSwitch } from "./LanguageSwitch";

const LICENSE_URL = "https://github.com/adamwahyuh/clumdf/blob/main/LICENSE";

/** Versi, hak cipta, lisensi, dan pilihan bahasa di dasar sidebar. */
export function SidebarFooter() {
  const { t } = useTranslation();

  return (
    <div className="mt-auto flex flex-col gap-2 p-3">
      <div className="flex flex-col gap-0.5 px-1 text-[11px] text-muted-foreground">
        <span className="tabular-nums">{version.trim()}</span>
        <span>
          {t("about.copyright", { year: new Date().getFullYear() })}
          {" · "}
          <button
            type="button"
            onClick={() => void openUrl(LICENSE_URL)}
            className="rounded-sm underline-offset-2 hover:text-foreground hover:underline focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
          >
            {t("about.license")}
          </button>
        </span>
      </div>
      <LanguageSwitch />
    </div>
  );
}
