import { Minimize2 } from "lucide-react";
import { BatchToolView } from "@/components/layout/BatchToolView";
import { CompressResult } from "./CompressResult";
import { CompressSettings } from "./CompressSettings";
import { useCompress, useCompressQueue } from "./use-compress";

export function CompressView() {
  return (
    <BatchToolView
      i18nKey="tools.compress"
      icon={Minimize2}
      queue={useCompressQueue}
      batch={useCompress()}
      settings={<CompressSettings />}
      renderResult={(results, reset) => <CompressResult results={results} onReset={reset} />}
    />
  );
}
