import { useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { useJobProgress } from "@/hooks/use-job-progress";
import { mergePdfs, type MergeReport } from "@/lib/tauri/commands";
import { pickSavePath } from "@/lib/tauri/dialogs";
import { errorMessage, toAppError } from "@/lib/tauri/errors";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";

export const useMergeQueue = createFileQueue(inspectPdfFile);

export function useMerge() {
  const { t } = useTranslation();
  const files = useMergeQueue((s) => s.files);
  const job = useJobProgress();
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<{ report: MergeReport; output: string } | null>(null);

  const canMerge = files.length >= 2 && files.every((f) => f.status === "ready") && !running;

  async function merge() {
    if (!canMerge) return;
    const output = await pickSavePath(`${t("tools.merge.defaultName")}.pdf`);
    if (!output) return;

    setRunning(true);
    try {
      const jobId = await job.start();
      const report = await mergePdfs({ inputs: files.map((f) => f.path), output, jobId });
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
    useMergeQueue.getState().clear();
  }

  return { files, running, progress: job.progress, result, canMerge, merge, reset };
}
