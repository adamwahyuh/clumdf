import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { Minimize2, Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ToolHeader } from "@/components/layout/ToolHeader";
import { ActionBar } from "@/components/common/ActionBar";
import { ProgressOverlay } from "@/components/common/ProgressOverlay";
import { DropOverlay } from "@/components/files/DropOverlay";
import { DropZone } from "@/components/files/DropZone";
import { SortableFileGrid } from "@/components/files/SortableFileGrid";
import { useFileDrop } from "@/hooks/use-file-drop";
import { formatBytes } from "@/lib/format";
import { pickFiles } from "@/lib/tauri/dialogs";
import { PDF_EXTENSIONS } from "@/types/files";
import { CompressResult } from "./CompressResult";
import { CompressSettings } from "./CompressSettings";
import { useCompress, useCompressQueue } from "./use-compress";

export function CompressView() {
  const { t, i18n } = useTranslation();
  const { files, current, running, progress, results, canCompress, compress, reset } = useCompress();
  const { add, remove, move } = useCompressQueue();

  function addFiles(paths: string[]) {
    if (results) reset();
    add(paths);
  }

  const dragging = useFileDrop(PDF_EXTENSIONS, (paths, rejected) => {
    if (rejected) toast.warning(t("files.unsupported", { count: rejected }));
    if (paths.length) addFiles(paths);
  });

  async function browse() {
    addFiles(await pickFiles("PDF", [...PDF_EXTENSIONS]));
  }

  const totalSize = files.reduce((sum, f) => sum + (f.size ?? 0), 0);
  const summary =
    files.length === 0
      ? t("actionBar.empty")
      : files.some((f) => f.status === "error")
        ? t("files.hasProblems")
        : t("tools.compress.summary", { count: files.length, size: formatBytes(totalSize, i18n.language) });

  return (
    <>
      <ToolHeader title={t("tools.compress.title")} description={t("tools.compress.description")} />
      <div className="flex min-h-0 flex-1">
        <div className="relative min-w-0 flex-1 overflow-y-auto px-6 pb-6">
          {results ? (
            <CompressResult results={results} onReset={reset} />
          ) : files.length === 0 ? (
            <DropZone icon={Minimize2} formats={t("tools.compress.formats")} onBrowse={browse} active={dragging} />
          ) : (
            <SortableFileGrid files={files} kind="pdf" onMove={move} onRemove={remove} />
          )}
          {dragging && files.length > 0 && !results && <DropOverlay />}
        </div>
        {!results && <CompressSettings />}
      </div>
      {!results && (
        <ActionBar summary={summary}>
          {files.length > 0 && (
            <Button variant="outline" onClick={browse}>
              <Plus />
              {t("files.add")}
            </Button>
          )}
          <Button disabled={!canCompress} onClick={compress}>
            {t("tools.compress.action")}
          </Button>
        </ActionBar>
      )}
      {running && (
        <ProgressOverlay
          label={t("tools.compress.running", { current: (current ?? 0) + 1, total: files.length })}
          progress={progress}
        />
      )}
    </>
  );
}
