import type { ComponentType } from "react";
import { FileStack, Images, LayoutGrid, Minimize2, Scissors, type LucideIcon } from "lucide-react";
import { MergeView } from "@/features/merge/MergeView";
import { CompressView } from "@/features/compress/CompressView";
import { ImageToPdfView } from "@/features/image-to-pdf/ImageToPdfView";
import { OrganizeView } from "@/features/organize/OrganizeView";
import { SplitView } from "@/features/split/SplitView";

export type ToolGroup = "organize" | "optimize" | "convert";

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
];

export const toolGroups: ToolGroup[] = ["organize", "optimize", "convert"];
