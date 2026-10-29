import { useTranslation } from "react-i18next";
import { FileStack } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ToolHeader } from "@/components/layout/ToolHeader";
import { ActionBar } from "@/components/common/ActionBar";
import { DropZone } from "@/components/files/DropZone";

export function MergeView() {
  const { t } = useTranslation();

  return (
    <>
      <ToolHeader title={t("tools.merge.title")} description={t("tools.merge.description")} />
      <div className="min-h-0 flex-1 overflow-y-auto px-6 pb-6">
        {/* shortcut: dialog pilih file belum tersambung, diisi saat fitur ini dikerjakan */}
        <DropZone icon={FileStack} formats={t("tools.merge.formats")} onBrowse={() => {}} />
      </div>
      <ActionBar summary={t("actionBar.empty")}>
        <Button disabled>{t("tools.merge.action")}</Button>
      </ActionBar>
    </>
  );
}
