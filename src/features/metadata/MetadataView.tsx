import { FileX2 } from "lucide-react";
import { BatchToolView } from "@/components/layout/BatchToolView";
import { useBatch } from "@/hooks/use-batch";
import { withSuffix } from "@/lib/format";
import { stripMetadata, type StripReport } from "@/lib/tauri/commands";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";

const useMetadataQueue = createFileQueue(inspectPdfFile);

export function MetadataView() {
  const batch = useBatch<StripReport>(useMetadataQueue, async (file) => {
    const output = withSuffix(file.path, "-clean");
    return { output, report: await stripMetadata({ input: file.path, output }) };
  });

  return <BatchToolView i18nKey="tools.metadata" icon={FileX2} queue={useMetadataQueue} batch={batch} />;
}
