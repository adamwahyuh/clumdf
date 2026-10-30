import { useBatch } from "@/hooks/use-batch";
import { withSuffix } from "@/lib/format";
import { compressPdf, type CompressReport } from "@/lib/tauri/commands";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";
import { useSettings } from "@/stores/settings-store";
import { resolveTarget } from "@/lib/compress-targets";
import { compressPresets } from "./presets";

export const useCompressQueue = createFileQueue(inspectPdfFile);

export function useCompress() {
  const settings = useSettings((s) => s.compress);

  return useBatch<CompressReport>(useCompressQueue, async (file, jobId) => {
    const options = {
      ...compressPresets[settings.preset],
      grayscale: settings.grayscale,
      targetSize: resolveTarget(settings.target, settings.customMb),
    };
    const output = withSuffix(file.path, "-compressed");
    return { output, report: await compressPdf({ input: file.path, output, options, jobId }) };
  });
}
