import { useTranslation } from "react-i18next";
import { FileUp, LayoutGrid } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ToolHeader } from "@/components/layout/ToolHeader";
import { ActionBar } from "@/components/common/ActionBar";
import { ProgressOverlay } from "@/components/common/ProgressOverlay";
import { ResultCard } from "@/components/common/ResultCard";
import { DropOverlay } from "@/components/files/DropOverlay";
import { DropZone } from "@/components/files/DropZone";
import { SortableFileGrid } from "@/components/files/SortableFileGrid";
import { SortableGrid } from "@/components/files/SortableGrid";
import { useFileInput } from "@/hooks/use-file-input";
import { useHotkey } from "@/hooks/use-hotkey";
import { formatBytes } from "@/lib/format";
import { PDF_EXTENSIONS } from "@/types/files";
import { OrganizeToolbar } from "./OrganizeToolbar";
import { PageCard } from "./PageCard";
import { useOrganize, useOrganizeQueue } from "./use-organize";

export function OrganizeView() {
  const { t, i18n } = useTranslation();
  const o = useOrganize();
  const { add, clear, remove: removeFile, move: moveFile } = useOrganizeQueue();

  // Hanya satu PDF: file baru menggantikan yang lama
  function setFile(paths: string[]) {
    if (o.result) o.reset();
    clear();
    add(paths.slice(0, 1));
  }

  const { dragging, browse } = useFileInput(PDF_EXTENSIONS, "PDF", setFile);
  useHotkey(["o"], browse, !o.running);
  useHotkey(["Enter"], o.save, !o.running && !o.result);
  useHotkey(["a"], o.toggleAll, o.pages.length > 0 && !o.result);

  const summary = !o.file
    ? t("actionBar.empty")
    : o.file.status === "error"
      ? t("files.hasProblems")
      : t("tools.organize.summary", { count: o.pages.length, selected: o.selectedIds.length });

  return (
    <>
      <ToolHeader title={t("tools.organize.title")} description={t("tools.organize.description")} />
      <div className="relative min-h-0 flex-1 overflow-y-auto px-6 pb-6">
        {o.result ? (
          <ResultCard
            title={t("tools.organize.done")}
            details={t("tools.organize.doneDetails", {
              count: o.result.report.pageCount,
              size: formatBytes(o.result.report.outputSize, i18n.language),
            })}
            outputPath={o.result.output}
            onReset={o.reset}
          />
        ) : !o.file ? (
          <DropZone icon={LayoutGrid} formats={t("tools.organize.formats")} onBrowse={browse} active={dragging} />
        ) : o.file.status !== "ready" ? (
          // Saat memuat atau bila file bermasalah, tampilkan kartu file biasa
          <SortableFileGrid files={[o.file]} kind="pdf" onMove={moveFile} onRemove={removeFile} />
        ) : (
          <>
            <OrganizeToolbar
              selectedCount={o.selectedIds.length}
              allSelected={o.allSelected}
              onToggleAll={o.toggleAll}
              onRotate={(delta) => o.rotate(o.selectedIds, delta)}
              onRemove={() => o.remove(o.selectedIds)}
            />
            <SortableGrid items={o.pages} onMove={o.move}>
              {(page) => (
                <PageCard
                  key={page.id}
                  page={page}
                  thumbnail={o.thumbnails[page.page]}
                  selected={o.selected.has(page.id)}
                  onToggle={() => o.toggle(page.id)}
                  onRotate={(delta) => o.rotate([page.id], delta)}
                  onRemove={() => o.remove([page.id])}
                />
              )}
            </SortableGrid>
          </>
        )}
        {dragging && o.file && !o.result && <DropOverlay />}
      </div>
      {!o.result && (
        <ActionBar summary={summary}>
          {o.file && (
            <Button variant="outline" onClick={browse}>
              <FileUp />
              {t("files.replace")}
            </Button>
          )}
          <Button variant="outline" disabled={!o.selectedIds.length || o.running} onClick={o.extract}>
            {t("tools.organize.extract")}
          </Button>
          <Button disabled={!o.pages.length || o.running} onClick={o.save}>
            {t("tools.organize.action")}
          </Button>
        </ActionBar>
      )}
      {o.running && <ProgressOverlay label={t("tools.organize.running")} progress={null} />}
    </>
  );
}
