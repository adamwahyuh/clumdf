import { useTranslation } from "react-i18next";
import { Images } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ToolHeader } from "@/components/layout/ToolHeader";
import { ActionBar } from "@/components/common/ActionBar";
import { DropZone } from "@/components/files/DropZone";

export function ImageToPdfView() {
  const { t } = useTranslation();

  return (
    <>
      <ToolHeader title={t("tools.imageToPdf.title")} description={t("tools.imageToPdf.description")} />
      <div className="min-h-0 flex-1 overflow-y-auto px-6 pb-6">
        {/* shortcut: dialog pilih file belum tersambung, diisi saat fitur ini dikerjakan */}
        <DropZone icon={Images} formats={t("tools.imageToPdf.formats")} onBrowse={() => {}} />
      </div>
      <ActionBar summary={t("actionBar.empty")}>
        <Button disabled>{t("tools.imageToPdf.action")}</Button>
      </ActionBar>
    </>
  );
}
