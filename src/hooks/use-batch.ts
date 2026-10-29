import { useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import type { StoreApi, UseBoundStore } from "zustand";
import { errorMessage, toAppError } from "@/lib/tauri/errors";
import type { FileQueue } from "@/stores/create-file-queue";
import type { QueuedFile } from "@/types/files";
import { useJobProgress } from "./use-job-progress";

export interface BatchResult<R> {
  file: QueuedFile;
  output: string;
  report: R;
}

export type FileQueueStore = UseBoundStore<StoreApi<FileQueue>>;

/**
 * Proses tiap file di antrian satu per satu (mis. kecilkan, kunci).
 * File yang gagal dilaporkan lewat toast; yang berhasil dikumpulkan sebagai hasil.
 */
export function useBatch<R>(
  queue: FileQueueStore,
  process: (file: QueuedFile, jobId: string) => Promise<{ output: string; report: R }>,
  /** Syarat tambahan dari pengaturan, mis. password sudah diisi */
  ready = true,
) {
  const { t } = useTranslation();
  const files = queue((s) => s.files);
  const job = useJobProgress();
  /** Indeks file yang sedang diproses; null = tidak berjalan */
  const [current, setCurrent] = useState<number | null>(null);
  const [results, setResults] = useState<BatchResult<R>[] | null>(null);

  const running = current !== null;
  const canRun = ready && files.length > 0 && files.every((f) => f.status === "ready") && !running;

  async function run() {
    if (!canRun) return;
    const done: BatchResult<R>[] = [];
    try {
      const jobId = await job.start();
      for (const [i, file] of files.entries()) {
        setCurrent(i);
        try {
          done.push({ file, ...(await process(file, jobId)) });
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
    queue.getState().clear();
  }

  return { files, current, running, progress: job.progress, results, canRun, run, reset };
}

export type Batch<R> = ReturnType<typeof useBatch<R>>;
