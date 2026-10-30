import { useTranslation } from "react-i18next";
import logo from "../../../src-tauri/icons/128x128.png";
import { isMac } from "@/lib/platform";
import { cn } from "@/lib/utils";

/**
 * Logo + nama app di atas sidebar. Di macOS duduk di kanan tombol traffic light
 * (area yang memang kosong), dan seluruhnya tetap bisa dipakai menggeser jendela.
 */
export function SidebarBrand() {
  const { t } = useTranslation();

  return (
    <div
      data-tauri-drag-region
      className={cn("flex shrink-0 items-center gap-2", isMac ? "h-[52px] pl-[84px]" : "h-14 px-4")}
    >
      <img data-tauri-drag-region src={logo} alt="" draggable={false} className="size-7" />
      <span data-tauri-drag-region className="text-[15px] font-semibold tracking-tight">
        {t("app.name")}
      </span>
    </div>
  );
}
