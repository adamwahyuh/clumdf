import { useTranslation } from "react-i18next";
import { Images } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ToolHeader } from "@/components/layout/ToolHeader";
import { ActionBar } from "@/components/common/ActionBar";
import { ProgressOverlay } from "@/components/common/ProgressOverlay";
import { ResultCard } from "@/components/common/ResultCard";
import { DropOverlay } from "@/components/files/DropOverlay";
import { AddFilesButtons } from "@/components/files/AddFilesButtons";
import { DropZone } from "@/components/files/DropZone";
import { SortableFileGrid } from "@/components/files/SortableFileGrid";
import { useFileInput } from "@/hooks/use-file-input";
import { useHotkey } from "@/hooks/use-hotkey";
import { formatBytes } from "@/lib/format";
import { IMAGE_EXTENSIONS } from "@/types/files";
import { ImageToPdfSettings } from "./ImageToPdfSettings";
import { useImageQueue, useImageToPdf } from "./use-image-to-pdf";

export function ImageToPdfView() {
  const { t, i18n } = useTranslation();
  const { files, running, progress, result, canCreate, create, reset } = useImageToPdf();
  const { add, remove, move } = useImageQueue();

  function addFiles(paths: string[]) {
    if (result) reset();
    add(paths);
  }

  const { dragging, browse, browseFolder } = useFileInput(IMAGE_EXTENSIONS, t("tools.imageToPdf.filterName"), addFiles);
  useHotkey(["o"], browse, !running);
  useHotkey(["Enter"], create, !running && !result);

  return (
    <>
      <ToolHeader title={t("tools.imageToPdf.title")} description={t("tools.imageToPdf.description")} />
      <div className="flex min-h-0 flex-1">
        <div className="relative min-w-0 flex-1 overflow-y-auto px-6 pb-6">
          {result ? (
            <ResultCard
              title={t("tools.imageToPdf.done")}
              details={t("tools.imageToPdf.doneDetails", {
                count: result.report.pageCount,
                size: formatBytes(result.report.outputSize, i18n.language),
              })}
              outputPath={result.output}
              onReset={reset}
            />
          ) : files.length === 0 ? (
            <DropZone icon={Images} formats={t("tools.imageToPdf.formats")} onBrowse={browse} onBrowseFolder={browseFolder} active={dragging} />
          ) : (
            <SortableFileGrid files={files} kind="image" onMove={move} onRemove={remove} />
          )}
          {dragging && files.length > 0 && !result && <DropOverlay />}
        </div>
        {!result && <ImageToPdfSettings />}
      </div>
      {!result && (
        <ActionBar summary={files.length ? t("tools.imageToPdf.summary", { count: files.length }) : t("actionBar.empty")}>
          {files.length > 0 && (
            <AddFilesButtons onBrowse={browse} onBrowseFolder={browseFolder} />
          )}
          <Button disabled={!canCreate} onClick={create}>
            {t("tools.imageToPdf.action")}
          </Button>
        </ActionBar>
      )}
      {running && <ProgressOverlay label={t("tools.imageToPdf.running")} progress={progress} />}
    </>
  );
}
