import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { Plus, type LucideIcon } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ActionBar } from "@/components/common/ActionBar";
import { ProgressOverlay } from "@/components/common/ProgressOverlay";
import { ResultCard } from "@/components/common/ResultCard";
import { DropOverlay } from "@/components/files/DropOverlay";
import { DropZone } from "@/components/files/DropZone";
import { SortableFileGrid } from "@/components/files/SortableFileGrid";
import type { Batch, BatchResult, FileQueueStore } from "@/hooks/use-batch";
import { useFileInput } from "@/hooks/use-file-input";
import { useHotkey } from "@/hooks/use-hotkey";
import { formatBytes } from "@/lib/format";
import { PDF_EXTENSIONS } from "@/types/files";
import { ToolHeader } from "./ToolHeader";

interface BatchToolViewProps<R> {
  /** Prefix i18n fitur; dipakai: title, description, formats, action, running, done, doneDetails */
  i18nKey: string;
  icon: LucideIcon;
  queue: FileQueueStore;
  batch: Batch<R>;
  settings?: ReactNode;
  /** Bawaan: kartu hasil dengan jumlah file */
  renderResult?: (results: BatchResult<R>[], reset: () => void) => ReactNode;
}

/** Layar fitur yang memproses banyak PDF satu per satu (kecilkan, kunci, hapus metadata, ...). */
export function BatchToolView<R>({ i18nKey, icon, queue, batch, settings, renderResult }: BatchToolViewProps<R>) {
  const { t, i18n } = useTranslation();
  const { files, results, running, current, progress, canRun, run, reset } = batch;
  const { add, remove, move } = queue();

  function addFiles(paths: string[]) {
    if (results) reset();
    add(paths);
  }

  const { dragging, browse } = useFileInput(PDF_EXTENSIONS, "PDF", addFiles);
  useHotkey(["o"], browse, !running);
  useHotkey(["Enter"], run, !running && !results);

  const totalSize = files.reduce((sum, f) => sum + (f.size ?? 0), 0);
  const summary =
    files.length === 0
      ? t("actionBar.empty")
      : files.some((f) => f.status === "error")
        ? t("files.hasProblems")
        : t("files.summary", { count: files.length, size: formatBytes(totalSize, i18n.language) });

  return (
    <>
      <ToolHeader title={t(`${i18nKey}.title`)} description={t(`${i18nKey}.description`)} />
      <div className="flex min-h-0 flex-1">
        <div className="relative min-w-0 flex-1 overflow-y-auto px-6 pb-6">
          {results ? (
            (renderResult?.(results, reset) ?? (
              <ResultCard
                title={t(`${i18nKey}.done`)}
                details={t(`${i18nKey}.doneDetails`, { count: results.length })}
                outputPath={results[0].output}
                onReset={reset}
              />
            ))
          ) : files.length === 0 ? (
            <DropZone icon={icon} formats={t(`${i18nKey}.formats`)} onBrowse={browse} active={dragging} />
          ) : (
            <SortableFileGrid files={files} kind="pdf" onMove={move} onRemove={remove} />
          )}
          {dragging && files.length > 0 && !results && <DropOverlay />}
        </div>
        {!results && settings}
      </div>
      {!results && (
        <ActionBar summary={summary}>
          {files.length > 0 && (
            <Button variant="outline" onClick={browse}>
              <Plus />
              {t("files.add")}
            </Button>
          )}
          <Button disabled={!canRun} onClick={run}>
            {t(`${i18nKey}.action`)}
          </Button>
        </ActionBar>
      )}
      {running && (
        <ProgressOverlay
          label={t(`${i18nKey}.running`, { current: (current ?? 0) + 1, total: files.length })}
          progress={progress}
        />
      )}
    </>
  );
}
