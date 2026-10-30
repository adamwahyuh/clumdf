import type { ComponentType } from "react";
import {
  FileStack,
  FileX2,
  History,
  Images,
  LayoutGrid,
  ListOrdered,
  Lock,
  LockOpen,
  Minimize2,
  PanelTop,
  Scissors,
  Signature,
  type LucideIcon,
} from "lucide-react";
import { MergeView } from "@/features/merge/MergeView";
import { CompressView } from "@/features/compress/CompressView";
import { HeaderFooterView } from "@/features/header-footer/HeaderFooterView";
import { HistoryView } from "@/features/history/HistoryView";
import { ImageToPdfView } from "@/features/image-to-pdf/ImageToPdfView";
import { MetadataView } from "@/features/metadata/MetadataView";
import { OrganizeView } from "@/features/organize/OrganizeView";
import { PageNumbersView } from "@/features/page-numbers/PageNumbersView";
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
}

// Urutan di sini = urutan di sidebar = urutan shortcut ⌘1..9.
export const tools: Tool[] = [
  { id: "merge", group: "organize", icon: FileStack, i18nKey: "tools.merge", view: MergeView },
  { id: "split", group: "organize", icon: Scissors, i18nKey: "tools.split", view: SplitView },
  { id: "organize", group: "organize", icon: LayoutGrid, i18nKey: "tools.organize", view: OrganizeView },
  { id: "signature", group: "edit", icon: Signature, i18nKey: "tools.signature", view: SignatureView },
  { id: "page-numbers", group: "edit", icon: ListOrdered, i18nKey: "tools.pageNumbers", view: PageNumbersView },
  { id: "header-footer", group: "edit", icon: PanelTop, i18nKey: "tools.headerFooter", view: HeaderFooterView },
  { id: "compress", group: "optimize", icon: Minimize2, i18nKey: "tools.compress", view: CompressView },
  { id: "image-to-pdf", group: "convert", icon: Images, i18nKey: "tools.imageToPdf", view: ImageToPdfView },
  { id: "protect", group: "security", icon: Lock, i18nKey: "tools.protect", view: ProtectView },
  { id: "unlock", group: "security", icon: LockOpen, i18nKey: "tools.unlock", view: UnlockView },
  { id: "metadata", group: "security", icon: FileX2, i18nKey: "tools.metadata", view: MetadataView },
  { id: "history", group: "other", icon: History, i18nKey: "tools.history", view: HistoryView },
];

export const toolGroups: ToolGroup[] = ["organize", "edit", "optimize", "convert", "security", "other"];
