import { useTranslation } from "react-i18next";
import { ListOrdered } from "lucide-react";
import { BatchToolView } from "@/components/layout/BatchToolView";
import { useBatch } from "@/hooks/use-batch";
import { withSuffix } from "@/lib/format";
import { stampText, type StampReport } from "@/lib/tauri/commands";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";
import { useSettings } from "@/stores/settings-store";
import { fontSizes, STAMP_MARGIN_MM } from "@/lib/stamp-sizes";
import { PageNumbersSettings } from "./PageNumbersSettings";

const usePageNumbersQueue = createFileQueue(inspectPdfFile);

export function PageNumbersView() {
  const { t } = useTranslation();
  const s = useSettings((state) => state.pageNumbers);

  const batch = useBatch<StampReport>(usePageNumbersQueue, async (file) => {
    const output = withSuffix(file.path, "-numbered");
    const options = {
      stamps: [{ text: t(`tools.pageNumbers.templates.${s.format}`), position: s.position }],
      fontSize: fontSizes[s.fontSize],
      marginMm: STAMP_MARGIN_MM,
      firstNumber: s.firstNumber,
      skipFirst: s.skipFirst,
    };
    return { output, report: await stampText({ input: file.path, output, options }) };
  });

  return (
    <BatchToolView
      i18nKey="tools.pageNumbers"
      icon={ListOrdered}
      queue={usePageNumbersQueue}
      batch={batch}
      settings={<PageNumbersSettings />}
    />
  );
}
