import { FileImage } from "lucide-react";
import { dirname } from "@tauri-apps/api/path";
import { BatchToolView } from "@/components/layout/BatchToolView";
import { useBatch } from "@/hooks/use-batch";
import { pdfToImages, type RenderReport } from "@/lib/tauri/commands";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";
import { useSettings } from "@/stores/settings-store";
import { PdfToImagesSettings } from "./PdfToImagesSettings";

const usePdfToImagesQueue = createFileQueue(inspectPdfFile);

export function PdfToImagesView() {
  const options = useSettings((s) => s.pdfToImages);

  // Gambar disimpan di samping PDF-nya, mis. "Surat-01.jpg", "Surat-02.jpg"
  const batch = useBatch<RenderReport>(usePdfToImagesQueue, async (file, jobId) => {
    const report = await pdfToImages({ input: file.path, outputDir: await dirname(file.path), options, jobId });
    return { output: report.files[0], report };
  });

  return (
    <BatchToolView
      i18nKey="tools.pdfToImages"
      icon={FileImage}
      queue={usePdfToImagesQueue}
      batch={batch}
      settings={<PdfToImagesSettings />}
    />
  );
}
