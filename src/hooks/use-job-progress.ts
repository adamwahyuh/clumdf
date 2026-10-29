import { useCallback, useEffect, useRef, useState } from "react";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface JobProgress {
  current: number;
  total: number;
}

/** Dengarkan event "job-progress" dari Rust untuk satu pekerjaan. */
export function useJobProgress() {
  const [progress, setProgress] = useState<JobProgress | null>(null);
  const unlistenRef = useRef<UnlistenFn | null>(null);

  const stop = useCallback(() => {
    unlistenRef.current?.();
    unlistenRef.current = null;
    setProgress(null);
  }, []);

  /** Pasang listener dulu, baru kembalikan jobId, supaya event pertama tidak terlewat. */
  const start = useCallback(async () => {
    stop();
    const jobId = crypto.randomUUID();
    setProgress({ current: 0, total: 1 });
    unlistenRef.current = await listen<JobProgress & { jobId: string }>("job-progress", ({ payload }) => {
      if (payload.jobId === jobId) setProgress({ current: payload.current, total: payload.total });
    });
    return jobId;
  }, [stop]);

  useEffect(() => () => unlistenRef.current?.(), []);

  return { progress, start, stop };
}
