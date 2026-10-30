import { useTranslation } from "react-i18next";
import { tools, type Tool } from "@/app/tools";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { fileName } from "@/lib/format";
import { IMAGE_EXTENSIONS, PDF_EXTENSIONS, type FileKind } from "@/types/files";

interface OpenWithDialogProps {
  paths: string[];
  onChoose: (tool: Tool) => void;
  onClose: () => void;
}

function kindOf(path: string): FileKind | null {
  const ext = path.split(".").pop()?.toLowerCase() ?? "";
  if ((PDF_EXTENSIONS as readonly string[]).includes(ext)) return "pdf";
  if ((IMAGE_EXTENSIONS as readonly string[]).includes(ext)) return "image";
  return null;
}

/** "Mau diapakan file ini?" untuk file yang dibuka lewat OS; hanya fitur yang cocok ditampilkan. */
export function OpenWithDialog({ paths, onChoose, onClose }: OpenWithDialogProps) {
  const { t } = useTranslation();
  const kinds = new Set(paths.map(kindOf));
  // Folder (tanpa ekstensi) bisa berisi apa saja, jadi semua fitur ditawarkan
  const matching = tools.filter((tool) => tool.accepts && (kinds.has(tool.accepts) || kinds.has(null)));
  const summary = paths.length === 1 ? fileName(paths[0]) : t("openWith.count", { count: paths.length });

  return (
    <Dialog open={paths.length > 0} onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>{t("openWith.title")}</DialogTitle>
          <DialogDescription className="truncate">{summary}</DialogDescription>
        </DialogHeader>
        {matching.length ? (
          <ul className="-mx-2 flex max-h-96 flex-col gap-0.5 overflow-y-auto">
            {matching.map((tool) => (
              <li key={tool.id}>
                <button
                  type="button"
                  onClick={() => onChoose(tool)}
                  className="flex w-full items-center gap-3 rounded-lg px-2 py-2 text-left transition-colors duration-150 hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
                >
                  <span className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-primary/10">
                    <tool.icon className="size-4 text-primary" />
                  </span>
                  <span className="min-w-0">
                    <span className="block text-sm font-medium">{t(`${tool.i18nKey}.title`)}</span>
                    <span className="block truncate text-xs text-muted-foreground">{t(`${tool.i18nKey}.description`)}</span>
                  </span>
                </button>
              </li>
            ))}
          </ul>
        ) : (
          <p className="text-sm text-muted-foreground">{t("openWith.unsupported")}</p>
        )}
      </DialogContent>
    </Dialog>
  );
}
