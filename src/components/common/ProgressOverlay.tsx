import { Progress } from "@/components/ui/progress";
import type { JobProgress } from "@/hooks/use-job-progress";

interface ProgressOverlayProps {
  label: string;
  progress: JobProgress | null;
}

/** Menutupi area konten selama proses berjalan supaya antrian tidak diubah. */
export function ProgressOverlay({ label, progress }: ProgressOverlayProps) {
  // Tanpa data progress (pekerjaan singkat), bar berdenyut alih-alih diam di 0%
  const percent = progress && progress.total > 0 ? (progress.current / progress.total) * 100 : null;

  return (
    <div className="absolute inset-0 z-30 flex items-center justify-center bg-background/70 backdrop-blur-sm animate-in fade-in duration-200">
      <div role="status" className="flex w-72 flex-col gap-3 rounded-xl bg-card p-5 shadow-lifted">
        <p className="text-sm font-medium">{label}</p>
        <Progress value={percent ?? 100} aria-label={label} className={percent === null ? "animate-pulse" : undefined} />
      </div>
    </div>
  );
}
