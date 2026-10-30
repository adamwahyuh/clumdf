import { useTranslation } from "react-i18next";
import { FileUp, Scissors } from "lucide-react";
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
import { PDF_EXTENSIONS } from "@/types/files";
import { SplitSettings } from "./SplitSettings";
import { useSplit, useSplitQueue } from "./use-split";

export function SplitView() {
  const { t } = useTranslation();
  const s = useSplit();
  const { add, remove, move, clear } = useSplitQueue();

  // Hanya satu PDF: file baru menggantikan yang lama
  function setFile(paths: string[]) {
    if (s.result) s.reset();
    clear();
    add(paths.slice(0, 1));
  }

  const { dragging, browse } = useFileInput(PDF_EXTENSIONS, "PDF", setFile);
  useHotkey(["o"], browse, !s.running);
  useHotkey(["Enter"], s.split, !s.running && !s.result);

  const summary = !s.file
    ? t("actionBar.empty")
    : s.file.status === "error"
      ? t("files.hasProblems")
      : t("tools.split.summary", { count: s.fileCount });

  return (
    <>
      <ToolHeader title={t("tools.split.title")} description={t("tools.split.description")} />
      <div className="flex min-h-0 flex-1">
        <div className="relative min-w-0 flex-1 overflow-y-auto px-6 pb-6">
          {s.result ? (
            <ResultCard
              title={t("tools.split.done")}
              details={t("tools.split.doneDetails", { count: s.result.files.length })}
              outputPath={s.result.files[0]}
              outputs={s.result.files}
              onReset={s.reset}
            />
          ) : !s.file ? (
            <DropZone icon={Scissors} formats={t("tools.split.formats")} onBrowse={browse} active={dragging} />
          ) : (
            <SortableFileGrid files={[s.file]} kind="pdf" onMove={move} onRemove={remove} />
          )}
          {dragging && s.file && !s.result && <DropOverlay />}
        </div>
        {!s.result && (
          <SplitSettings ranges={s.ranges} onRangesChange={s.changeRanges} rangesInvalid={s.rangesInvalid} />
        )}
      </div>
      {!s.result && (
        <ActionBar summary={summary}>
          {s.file && (
            <Button variant="outline" onClick={browse}>
              <FileUp />
              {t("files.replace")}
            </Button>
          )}
          <Button disabled={!s.canSplit} onClick={s.split}>
            {t("tools.split.action")}
          </Button>
        </ActionBar>
      )}
      {s.running && <ProgressOverlay label={t("tools.split.running")} progress={s.progress} />}
    </>
  );
}
