import type { ComponentType } from "react";
import type { FileKind } from "@/types/files";
import {
  FileImage,
  FileStack,
  FileX2,
  History,
  Images,
  LayoutGrid,
  ListOrdered,
  Lock,
  LockOpen,
  Minimize2,
  NotebookPen,
  PanelTop,
  ScanText,
  Scissors,
  Signature,
  type LucideIcon,
} from "lucide-react";
import { MergeView } from "@/features/merge/MergeView";
import { CompressView } from "@/features/compress/CompressView";
import { HeaderFooterView } from "@/features/header-footer/HeaderFooterView";
import { HistoryView } from "@/features/history/HistoryView";
import { ImageToPdfView } from "@/features/image-to-pdf/ImageToPdfView";
import { MarkdownView } from "@/features/markdown/MarkdownView";
import { MetadataView } from "@/features/metadata/MetadataView";
import { OcrView } from "@/features/ocr/OcrView";
import { OrganizeView } from "@/features/organize/OrganizeView";
import { PageNumbersView } from "@/features/page-numbers/PageNumbersView";
import { PdfToImagesView } from "@/features/pdf-to-images/PdfToImagesView";
import { ProtectView } from "@/features/password/ProtectView";
import { UnlockView } from "@/features/password/UnlockView";
import { SignatureView } from "@/features/signature/SignatureView";
import { SplitView } from "@/features/split/SplitView";

export type ToolGroup = "organize" | "edit" | "optimize" | "convert" | "security" | "other";

export interface Tool {
  id: string;
  group: ToolGroup;
  icon: LucideIcon;
  /** Prefix kunci i18n, mis. "tools.merge" → "tools.merge.title" */
  i18nKey: string;
  view: ComponentType;
  /** Jenis file yang diterima; dipakai dialog "Buka dengan" */
  accepts?: FileKind;
}

// Urutan di sini = urutan di sidebar = urutan shortcut ⌘1..9.
export const tools: Tool[] = [
  { id: "merge", group: "organize", icon: FileStack, i18nKey: "tools.merge", view: MergeView, accepts: "pdf" },
  { id: "split", group: "organize", icon: Scissors, i18nKey: "tools.split", view: SplitView, accepts: "pdf" },
  { id: "organize", group: "organize", icon: LayoutGrid, i18nKey: "tools.organize", view: OrganizeView, accepts: "pdf" },
  { id: "signature", group: "edit", icon: Signature, i18nKey: "tools.signature", view: SignatureView, accepts: "pdf" },
  { id: "page-numbers", group: "edit", icon: ListOrdered, i18nKey: "tools.pageNumbers", view: PageNumbersView, accepts: "pdf" },
  { id: "header-footer", group: "edit", icon: PanelTop, i18nKey: "tools.headerFooter", view: HeaderFooterView, accepts: "pdf" },
  { id: "compress", group: "optimize", icon: Minimize2, i18nKey: "tools.compress", view: CompressView, accepts: "pdf" },
  { id: "image-to-pdf", group: "convert", icon: Images, i18nKey: "tools.imageToPdf", view: ImageToPdfView, accepts: "image" },
  { id: "markdown", group: "convert", icon: NotebookPen, i18nKey: "tools.markdown", view: MarkdownView },
  { id: "ocr", group: "convert", icon: ScanText, i18nKey: "tools.ocr", view: OcrView, accepts: "pdf" },
  { id: "pdf-to-images", group: "convert", icon: FileImage, i18nKey: "tools.pdfToImages", view: PdfToImagesView, accepts: "pdf" },
  { id: "protect", group: "security", icon: Lock, i18nKey: "tools.protect", view: ProtectView, accepts: "pdf" },
  { id: "unlock", group: "security", icon: LockOpen, i18nKey: "tools.unlock", view: UnlockView, accepts: "pdf" },
  { id: "metadata", group: "security", icon: FileX2, i18nKey: "tools.metadata", view: MetadataView, accepts: "pdf" },
  { id: "history", group: "other", icon: History, i18nKey: "tools.history", view: HistoryView },
];

export const toolGroups: ToolGroup[] = ["organize", "edit", "optimize", "convert", "security", "other"];
