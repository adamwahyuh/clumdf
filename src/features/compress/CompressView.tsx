import { useTranslation } from "react-i18next";
import { Minimize2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ToolHeader } from "@/components/layout/ToolHeader";
import { ActionBar } from "@/components/common/ActionBar";
import { DropZone } from "@/components/files/DropZone";

export function CompressView() {
  const { t } = useTranslation();

  return (
    <>
      <ToolHeader title={t("tools.compress.title")} description={t("tools.compress.description")} />
      <div className="min-h-0 flex-1 overflow-y-auto px-6 pb-6">
        {/* shortcut: dialog pilih file belum tersambung, diisi saat fitur ini dikerjakan */}
        <DropZone icon={Minimize2} formats={t("tools.compress.formats")} onBrowse={() => {}} />
      </div>
      <ActionBar summary={t("actionBar.empty")}>
        <Button disabled>{t("tools.compress.action")}</Button>
      </ActionBar>
    </>
  );
}
