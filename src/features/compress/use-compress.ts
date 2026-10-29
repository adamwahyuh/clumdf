import { useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { useJobProgress } from "@/hooks/use-job-progress";
import { withSuffix } from "@/lib/format";
import { compressPdf, type CompressReport } from "@/lib/tauri/commands";
import { errorMessage, toAppError } from "@/lib/tauri/errors";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";
import { useSettings } from "@/stores/settings-store";
import { compressPresets, resolveTarget } from "./presets";

export const useCompressQueue = createFileQueue(inspectPdfFile);

export interface CompressedFile {
  name: string;
  output: string;
  report: CompressReport;
}

export function useCompress() {
  const { t } = useTranslation();
  const files = useCompressQueue((s) => s.files);
  const settings = useSettings((s) => s.compress);
  const job = useJobProgress();
  /** Indeks file yang sedang diproses; null = tidak berjalan */
  const [current, setCurrent] = useState<number | null>(null);
  const [results, setResults] = useState<CompressedFile[] | null>(null);

  const running = current !== null;
  const canCompress = files.length > 0 && files.every((f) => f.status === "ready") && !running;

  async function compress() {
    if (!canCompress) return;
    const options = {
      ...compressPresets[settings.preset],
      grayscale: settings.grayscale,
      targetSize: resolveTarget(settings.target, settings.customMb),
    };
    const done: CompressedFile[] = [];

    try {
      const jobId = await job.start();
      // Satu per satu: gambar di tiap file sudah diproses paralel di Rust
      for (const [i, file] of files.entries()) {
        setCurrent(i);
        const output = withSuffix(file.path, "-compressed");
        try {
          const report = await compressPdf({ input: file.path, output, options, jobId });
          done.push({ name: file.name, output, report });
        } catch (err) {
          toast.error(errorMessage(t, toAppError(err)));
        }
      }
    } finally {
      job.stop();
      setCurrent(null);
    }
    if (done.length) setResults(done);
  }

  function reset() {
    setResults(null);
    useCompressQueue.getState().clear();
  }

  return { files, current, running, progress: job.progress, results, canCompress, compress, reset };
}
