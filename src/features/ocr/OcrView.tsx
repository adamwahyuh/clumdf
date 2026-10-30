import { useEffect, useState } from "react";
import { ScanText } from "lucide-react";
import { BatchToolView } from "@/components/layout/BatchToolView";
import { useBatch } from "@/hooks/use-batch";
import { withSuffix } from "@/lib/format";
import { ocrPdf, ocrStatus, type OcrReport } from "@/lib/tauri/commands";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";
import { OcrSetup } from "./OcrSetup";

const useOcrQueue = createFileQueue(inspectPdfFile);

export function OcrView() {
  const [tesseract, setTesseract] = useState<string | null | undefined>(undefined);
  const check = () => void ocrStatus().then(setTesseract, () => setTesseract(null));
  useEffect(check, []);

  const batch = useBatch<OcrReport>(
    useOcrQueue,
    async (file, jobId) => {
      const output = withSuffix(file.path, "-ocr");
      return { output, report: await ocrPdf({ input: file.path, output, jobId }) };
    },
    !!tesseract,
  );

  return (
    <BatchToolView
      i18nKey="tools.ocr"
      icon={ScanText}
      queue={useOcrQueue}
      batch={batch}
      settings={<OcrSetup tesseract={tesseract} onRecheck={check} />}
    />
  );
}
