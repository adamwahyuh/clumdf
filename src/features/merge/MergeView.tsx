import { useTranslation } from "react-i18next";
import { FileStack, Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ToolHeader } from "@/components/layout/ToolHeader";
import { ActionBar } from "@/components/common/ActionBar";
import { ProgressOverlay } from "@/components/common/ProgressOverlay";
import { ResultCard } from "@/components/common/ResultCard";
import { DropOverlay } from "@/components/files/DropOverlay";
import { DropZone } from "@/components/files/DropZone";
import { SortableFileGrid } from "@/components/files/SortableFileGrid";
import { useFileInput } from "@/hooks/use-file-input";
import { useHotkey } from "@/hooks/use-hotkey";
import { formatBytes } from "@/lib/format";
import { PDF_EXTENSIONS } from "@/types/files";
import { useMerge, useMergeQueue } from "./use-merge";

export function MergeView() {
  const { t, i18n } = useTranslation();
  const { files, running, progress, result, canMerge, merge, reset } = useMerge();
  const { add, remove, move } = useMergeQueue();

  function addFiles(paths: string[]) {
    if (result) reset();
    add(paths);
  }

  const { dragging, browse } = useFileInput(PDF_EXTENSIONS, "PDF", addFiles);
  useHotkey(["o"], browse, !running);
  useHotkey(["Enter"], merge, !running && !result);

  const totalPages = files.reduce((sum, f) => sum + (f.pageCount ?? 0), 0);
  const summary =
    files.length === 0
      ? t("actionBar.empty")
      : files.some((f) => f.status === "error")
        ? t("files.hasProblems")
        : files.length === 1
          ? t("tools.merge.needTwo")
          : t("tools.merge.summary", { files: files.length, pages: totalPages });

  return (
    <>
      <ToolHeader title={t("tools.merge.title")} description={t("tools.merge.description")} />
      <div className="relative min-h-0 flex-1 overflow-y-auto px-6 pb-6">
        {result ? (
          <ResultCard
            title={t("tools.merge.done")}
            details={t("tools.merge.doneDetails", {
              pages: result.report.pageCount,
              size: formatBytes(result.report.outputSize, i18n.language),
            })}
            outputPath={result.output}
            onReset={reset}
          />
        ) : files.length === 0 ? (
          <DropZone icon={FileStack} formats={t("tools.merge.formats")} onBrowse={browse} active={dragging} />
        ) : (
          <SortableFileGrid files={files} kind="pdf" onMove={move} onRemove={remove} />
        )}
        {dragging && files.length > 0 && !result && <DropOverlay />}
      </div>
      {!result && (
        <ActionBar summary={summary}>
          {files.length > 0 && (
            <Button variant="outline" onClick={browse}>
              <Plus />
              {t("files.add")}
            </Button>
          )}
          <Button disabled={!canMerge} onClick={merge}>
            {t("tools.merge.action")}
          </Button>
        </ActionBar>
      )}
      {running && <ProgressOverlay label={t("tools.merge.running")} progress={progress} />}
    </>
  );
}
