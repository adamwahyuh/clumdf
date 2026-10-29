import { PanelTop } from "lucide-react";
import { BatchToolView } from "@/components/layout/BatchToolView";
import { useBatch } from "@/hooks/use-batch";
import { withSuffix } from "@/lib/format";
import { fontSizes, STAMP_MARGIN_MM } from "@/lib/stamp-sizes";
import { stampText, type StampPosition, type StampReport } from "@/lib/tauri/commands";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";
import { useSettings, type Align } from "@/stores/settings-store";
import { HeaderFooterSettings } from "./HeaderFooterSettings";

const useHeaderFooterQueue = createFileQueue(inspectPdfFile);

const position = (row: "top" | "bottom", align: Align) =>
  `${row}${align[0].toUpperCase()}${align.slice(1)}` as StampPosition;

export function HeaderFooterView() {
  const s = useSettings((state) => state.headerFooter);
  const stamps = [
    { text: s.header, position: position("top", s.headerAlign) },
    { text: s.footer, position: position("bottom", s.footerAlign) },
  ].filter((stamp) => stamp.text.trim());

  const batch = useBatch<StampReport>(
    useHeaderFooterQueue,
    async (file) => {
      const output = withSuffix(file.path, "-header");
      const options = { stamps, fontSize: fontSizes[s.fontSize], marginMm: STAMP_MARGIN_MM, firstNumber: 1, skipFirst: false };
      return { output, report: await stampText({ input: file.path, output, options }) };
    },
    stamps.length > 0,
  );

  return (
    <BatchToolView
      i18nKey="tools.headerFooter"
      icon={PanelTop}
      queue={useHeaderFooterQueue}
      batch={batch}
      settings={<HeaderFooterSettings />}
    />
  );
}
