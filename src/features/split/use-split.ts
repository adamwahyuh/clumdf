import { useState } from "react";
import { useTranslation } from "react-i18next";
import { dirname } from "@tauri-apps/api/path";
import { toast } from "sonner";
import { useJobProgress } from "@/hooks/use-job-progress";
import { splitPdf, type SplitMode, type SplitReport } from "@/lib/tauri/commands";
import { pickFolder } from "@/lib/tauri/dialogs";
import { errorMessage, toAppError } from "@/lib/tauri/errors";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";
import { useSettings } from "@/stores/settings-store";

// Split hanya memproses satu PDF; antrian dipakai ulang demi kartu & thumbnail.
export const useSplitQueue = createFileQueue(inspectPdfFile);

export function useSplit() {
  const { t } = useTranslation();
  const file = useSplitQueue((s) => s.files[0]);
  const settings = useSettings((s) => s.split);
  const job = useJobProgress();
  const [ranges, setRanges] = useState("");
  const [rangesInvalid, setRangesInvalid] = useState(false);
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<SplitReport | null>(null);

  const mode: SplitMode =
    settings.mode === "ranges"
      ? { mode: "ranges", ranges }
      : settings.mode === "everyN"
        ? { mode: "everyN", n: settings.n }
        : { mode: "eachPage" };

  /** Perkiraan jumlah file untuk ringkasan; Rust tetap yang memvalidasi. */
  const pages = file?.pageCount ?? 0;
  const fileCount =
    mode.mode === "eachPage"
      ? pages
      : mode.mode === "everyN"
        ? Math.ceil(pages / mode.n)
        : ranges.split(",").filter((part) => part.trim()).length;

  const canSplit = file?.status === "ready" && fileCount > 0 && !running;

  function changeRanges(value: string) {
    setRanges(value);
    setRangesInvalid(false);
  }

  async function split() {
    if (!canSplit) return;
    const outputDir = await pickFolder(await dirname(file.path));
    if (!outputDir) return;

    setRunning(true);
    try {
      const jobId = await job.start();
      setResult(await splitPdf({ input: file.path, outputDir, mode, jobId }));
    } catch (err) {
      const appError = toAppError(err);
      if (appError.code === "invalid_range") setRangesInvalid(true);
      toast.error(errorMessage(t, appError));
    } finally {
      job.stop();
      setRunning(false);
    }
  }

  function reset() {
    setResult(null);
    useSplitQueue.getState().clear();
  }

  return { file, fileCount, ranges, rangesInvalid, changeRanges, running, progress: job.progress, result, canSplit, split, reset };
}
