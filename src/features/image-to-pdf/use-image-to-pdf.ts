import { useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { useJobProgress } from "@/hooks/use-job-progress";
import { resolveTarget } from "@/lib/compress-targets";
import { imagesToPdf, type ImageToPdfReport } from "@/lib/tauri/commands";
import { pickSavePath } from "@/lib/tauri/dialogs";
import { errorMessage, toAppError } from "@/lib/tauri/errors";
import { createFileQueue } from "@/stores/create-file-queue";
import { useSettings } from "@/stores/settings-store";

// Gambar tidak diinspeksi dulu; gambar rusak dilaporkan Rust saat diproses.
export const useImageQueue = createFileQueue();

export function useImageToPdf() {
  const { t } = useTranslation();
  const files = useImageQueue((s) => s.files);
  const settings = useSettings((s) => s.imageToPdf);
  const job = useJobProgress();
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<{ report: ImageToPdfReport; output: string } | null>(null);

  const canCreate = files.length > 0 && !running;

  async function create() {
    if (!canCreate) return;
    const output = await pickSavePath(`${t("tools.imageToPdf.defaultName")}.pdf`);
    if (!output) return;

    const { compressAfter, ...options } = settings;
    const targetSize = resolveTarget(compressAfter, 0);
    // Workflow berantai: preset "Seimbang" dengan target ukuran
    const compress = targetSize ? { maxDimension: 1600, jpegQuality: 65, grayscale: false, targetSize } : null;

    setRunning(true);
    try {
      const jobId = await job.start();
      const report = await imagesToPdf({ inputs: files.map((f) => f.path), output, options: { ...options, compress }, jobId });
      setResult({ report, output });
    } catch (err) {
      toast.error(errorMessage(t, toAppError(err)));
    } finally {
      job.stop();
      setRunning(false);
    }
  }

  function reset() {
    setResult(null);
    useImageQueue.getState().clear();
  }

  return { files, running, progress: job.progress, result, canCreate, create, reset };
}
