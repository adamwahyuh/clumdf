import { useTranslation } from "react-i18next";
import { convertFileSrc } from "@tauri-apps/api/core";
import { FileUp, Signature } from "lucide-react";
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
import { PagePreview } from "./PagePreview";
import { SignaturePanel } from "./SignaturePanel";
import { usePagePreview } from "./use-page-preview";
import { useSignature, useSignatureQueue } from "./use-signature";

export function SignatureView() {
  const { t, i18n } = useTranslation();
  const s = useSignature();
  const { add, clear, remove: removeFile, move: moveFile } = useSignatureQueue();
  const ready = s.file?.status === "ready";
  const preview = usePagePreview(ready ? s.file!.path : undefined, s.page);
  // ?v= memaksa webview memuat ulang setelah tanda tangan diganti
  const signatureUrl = s.signature ? `${convertFileSrc(s.signature.path)}?v=${s.signature.version}` : undefined;

  function setFile(paths: string[]) {
    if (s.result) s.reset();
    clear();
    add(paths.slice(0, 1));
  }

  const { dragging, browse } = useFileInput(PDF_EXTENSIONS, "PDF", setFile);
  useHotkey(["o"], browse, !s.running);
  useHotkey(["Enter"], s.save, !s.running && !s.result);

  const summary = !s.file
    ? t("actionBar.empty")
    : !s.signature
      ? t("tools.signature.needSignature")
      : s.placements.length === 0
        ? t("tools.signature.needPlacement")
        : t("tools.signature.summary", { count: s.placements.length });

  return (
    <>
      <ToolHeader title={t("tools.signature.title")} description={t("tools.signature.description")} />
      <div className="flex min-h-0 flex-1">
        <div className="relative min-w-0 flex-1 overflow-y-auto px-6 pb-6">
          {s.result ? (
            <ResultCard
              title={t("tools.signature.done")}
              details={t("tools.signature.doneDetails", {
                count: s.result.report.signedPages,
                size: formatBytes(s.result.report.outputSize, i18n.language),
              })}
              outputPath={s.result.output}
              onReset={s.reset}
            />
          ) : !s.file ? (
            <DropZone icon={Signature} formats={t("tools.signature.formats")} onBrowse={browse} active={dragging} />
          ) : !ready ? (
            <SortableFileGrid files={[s.file]} kind="pdf" onMove={moveFile} onRemove={removeFile} />
          ) : (
            <PagePreview
              preview={preview}
              page={s.page}
              pageCount={s.file.pageCount ?? 1}
              onPageChange={s.setPage}
              placements={s.placements}
              signatureUrl={signatureUrl}
              signatureRatio={s.signature ? s.signature.height / s.signature.width : 1}
              onChange={s.update}
              onRemove={s.remove}
            />
          )}
          {dragging && s.file && !s.result && <DropOverlay />}
        </div>
        {!s.result && (
          <SignaturePanel
            signatureUrl={signatureUrl}
            onChoose={s.chooseSignature}
            onPlace={ready && s.signature && preview ? () => s.addOnPage(preview.aspect) : undefined}
          />
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
          <Button disabled={!s.canSave} onClick={s.save}>
            {t("tools.signature.action")}
          </Button>
        </ActionBar>
      )}
      {s.running && <ProgressOverlay label={t("tools.signature.running")} progress={null} />}
    </>
  );
}
