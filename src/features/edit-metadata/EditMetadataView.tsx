import { useTranslation } from "react-i18next";
import { FilePen, FileUp } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ActionBar } from "@/components/common/ActionBar";
import { ProgressOverlay } from "@/components/common/ProgressOverlay";
import { ResultCard } from "@/components/common/ResultCard";
import { DropOverlay } from "@/components/files/DropOverlay";
import { DropZone } from "@/components/files/DropZone";
import { SortableFileGrid } from "@/components/files/SortableFileGrid";
import { ToolHeader } from "@/components/layout/ToolHeader";
import { useFileInput } from "@/hooks/use-file-input";
import { useHotkey } from "@/hooks/use-hotkey";
import { PDF_EXTENSIONS } from "@/types/files";
import { MetadataForm } from "./MetadataForm";
import { useEditMetadata, useEditMetadataQueue } from "./use-edit-metadata";

export function EditMetadataView() {
  const { t } = useTranslation();
  const m = useEditMetadata();
  const { add, clear, remove, move } = useEditMetadataQueue();

  // Hanya satu PDF: file baru menggantikan yang lama
  function setFile(paths: string[]) {
    if (m.result) m.reset();
    clear();
    add(paths.slice(0, 1));
  }

  const { dragging, browse } = useFileInput(PDF_EXTENSIONS, "PDF", setFile);
  useHotkey(["o"], browse, !m.running);
  useHotkey(["Enter"], m.save, !m.running && !m.result);

  const summary = !m.file
    ? t("actionBar.empty")
    : m.file.status === "error"
      ? t("files.hasProblems")
      : t(m.changed ? "tools.editMetadata.changed" : "tools.editMetadata.unchanged");

  return (
    <>
      <ToolHeader title={t("tools.editMetadata.title")} description={t("tools.editMetadata.description")} />
      <div className="relative min-h-0 flex-1 overflow-y-auto px-6 pb-6">
        {m.result ? (
          <ResultCard
            title={t("tools.editMetadata.done")}
            details={t("tools.editMetadata.doneDetails")}
            outputPath={m.result.output}
            onReset={m.reset}
          />
        ) : !m.file ? (
          <DropZone icon={FilePen} formats={t("tools.editMetadata.formats")} onBrowse={browse} active={dragging} />
        ) : m.form ? (
          <MetadataForm fileName={m.file.name} form={m.form} onChange={m.setField} />
        ) : (
          // Saat membaca, atau bila file terkunci/rusak: tampilkan kartu file biasa
          <SortableFileGrid files={[m.file]} kind="pdf" onMove={move} onRemove={remove} />
        )}
        {dragging && m.file && !m.result && <DropOverlay />}
      </div>
      {!m.result && (
        <ActionBar summary={summary}>
          {m.file && (
            <Button variant="outline" onClick={browse}>
              <FileUp />
              {t("files.replace")}
            </Button>
          )}
          <Button disabled={!m.form || m.running} onClick={m.save}>
            {t("tools.editMetadata.action")}
          </Button>
        </ActionBar>
      )}
      {m.running && <ProgressOverlay label={t("tools.editMetadata.running")} progress={null} />}
    </>
  );
}
