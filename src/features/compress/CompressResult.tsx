import { useTranslation } from "react-i18next";
import { ArrowRight } from "lucide-react";
import { ResultCard } from "@/components/common/ResultCard";
import type { BatchResult } from "@/hooks/use-batch";
import { formatBytes } from "@/lib/format";
import type { CompressReport } from "@/lib/tauri/commands";

interface CompressResultProps {
  results: BatchResult<CompressReport>[];
  onReset: () => void;
}

/** Ukuran sebelum → sesudah per file, plus persentase hemat. */
export function CompressResult({ results, onReset }: CompressResultProps) {
  const { t, i18n } = useTranslation();
  const size = (bytes: number) => formatBytes(bytes, i18n.language);
  const allOptimal = results.every((r) => r.report.alreadyOptimal);

  return (
    <ResultCard
      title={t(allOptimal ? "tools.compress.allOptimal" : "tools.compress.done")}
      outputPath={results[0].output}
      outputs={results.map((r) => r.output)}
      onReset={onReset}
      details={
        <ul className="mt-2 flex flex-col gap-2 text-left">
          {results.map(({ file: { name }, report }, i) => {
            const saved = Math.round((1 - report.compressedSize / report.originalSize) * 100);
            return (
              <li key={i} className="flex flex-col gap-0.5 rounded-lg bg-muted px-3 py-2">
                <span className="truncate font-medium text-foreground" title={name}>
                  {name}
                </span>
                {report.targetMet === false && (
                  <span className="text-destructive">{t("tools.compress.targetMissed")}</span>
                )}
                {report.alreadyOptimal ? (
                  <span>{t("tools.compress.optimal")}</span>
                ) : (
                  <span className="flex flex-wrap items-center gap-1.5 tabular-nums">
                    {size(report.originalSize)}
                    <ArrowRight className="size-3.5" aria-label={t("tools.compress.becomes")} />
                    <span className="font-medium text-foreground">{size(report.compressedSize)}</span>
                    <span className="font-medium text-success">{t("tools.compress.saved", { percent: saved })}</span>
                  </span>
                )}
              </li>
            );
          })}
        </ul>
      }
    />
  );
}
