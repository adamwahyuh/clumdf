import type { ComponentType } from "react";
import { FileStack, FileX2, Images, LayoutGrid, Lock, LockOpen, Minimize2, Scissors, type LucideIcon } from "lucide-react";
import { MergeView } from "@/features/merge/MergeView";
import { CompressView } from "@/features/compress/CompressView";
import { ImageToPdfView } from "@/features/image-to-pdf/ImageToPdfView";
import { MetadataView } from "@/features/metadata/MetadataView";
import { OrganizeView } from "@/features/organize/OrganizeView";
import { ProtectView } from "@/features/password/ProtectView";
import { UnlockView } from "@/features/password/UnlockView";
import { SplitView } from "@/features/split/SplitView";

export type ToolGroup = "organize" | "optimize" | "convert" | "security";

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
  { id: "compress", group: "optimize", icon: Minimize2, i18nKey: "tools.compress", view: CompressView },
  { id: "image-to-pdf", group: "convert", icon: Images, i18nKey: "tools.imageToPdf", view: ImageToPdfView },
  { id: "protect", group: "security", icon: Lock, i18nKey: "tools.protect", view: ProtectView },
  { id: "unlock", group: "security", icon: LockOpen, i18nKey: "tools.unlock", view: UnlockView },
  { id: "metadata", group: "security", icon: FileX2, i18nKey: "tools.metadata", view: MetadataView },
];

export const toolGroups: ToolGroup[] = ["organize", "optimize", "convert", "security"];
