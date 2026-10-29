import { useTranslation } from "react-i18next";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { CircleAlert, Lock, X } from "lucide-react";
import { formatBytes } from "@/lib/format";
import { cn } from "@/lib/utils";
import type { FileKind, QueuedFile } from "@/types/files";
import { FileThumbnail } from "./FileThumbnail";

interface FileCardProps {
  file: QueuedFile;
  index: number;
  kind: FileKind;
  onRemove: (id: string) => void;
}

/** Kartu file yang bisa di-drag; hanya dipakai di dalam SortableFileGrid. */
export function FileCard({ file, index, kind, onRemove }: FileCardProps) {
  const { t, i18n } = useTranslation();
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({
    id: file.id,
  });

  // Kartu yang sedang di-drag sedikit membesar
  const style = {
    transform: CSS.Transform.toString(transform && { ...transform, scaleX: isDragging ? 1.02 : 1, scaleY: isDragging ? 1.02 : 1 }),
    transition,
  };

  const info =
    file.status === "loading"
      ? t("files.loading")
      : file.status === "error"
        ? t(`files.problem.${file.errorCode === "encrypted" ? "encrypted" : "invalid"}`)
        : [file.pageCount != null && t("files.pages", { count: file.pageCount }), file.size != null && formatBytes(file.size, i18n.language)]
            .filter(Boolean)
            .join(" · ");

  return (
    <div
      ref={setNodeRef}
      style={style}
      {...attributes}
      {...listeners}
      // Delete/Backspace pada kartu yang difokus menghapusnya; tombol lain tetap untuk dnd-kit
      onKeyDown={(e) => {
        if (e.key === "Delete" || e.key === "Backspace") {
          e.preventDefault();
          onRemove(file.id);
        } else {
          listeners?.onKeyDown?.(e);
        }
      }}
      aria-label={file.name}
      className={cn(
        "group relative flex cursor-grab flex-col gap-2 rounded-xl bg-card p-2 shadow-card transition-shadow duration-200 ease-out",
        "focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none",
        isDragging && "z-10 cursor-grabbing shadow-lifted",
        file.status === "error" && "ring-1 ring-destructive/60",
      )}
    >
      <div className="relative aspect-[3/4] overflow-hidden rounded-lg bg-muted">
        <FileThumbnail path={file.path} kind={kind} />
        <span className="absolute top-1.5 left-1.5 flex h-5 min-w-5 items-center justify-center rounded-full bg-foreground/75 px-1.5 text-[11px] font-semibold text-background">
          {index + 1}
        </span>
        <button
          type="button"
          aria-label={t("files.remove", { name: file.name })}
          // Cegah klik hapus memulai drag
          onPointerDown={(e) => e.stopPropagation()}
          onClick={() => onRemove(file.id)}
          className="absolute top-1.5 right-1.5 flex size-6 items-center justify-center rounded-full bg-foreground/75 text-background opacity-0 transition-opacity duration-150 group-hover:opacity-100 hover:bg-destructive focus-visible:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        >
          <X className="size-3.5" />
        </button>
        {file.status === "error" && (
          <span className="absolute inset-x-0 bottom-0 flex items-center justify-center bg-destructive/90 py-1 text-white">
            {file.errorCode === "encrypted" ? <Lock className="size-3.5" /> : <CircleAlert className="size-3.5" />}
          </span>
        )}
      </div>
      <div className="min-w-0 px-1 pb-0.5">
        <p className="truncate text-[13px] font-medium" title={file.name}>
          {file.name}
        </p>
        <p className={cn("truncate text-xs", file.status === "error" ? "text-destructive" : "text-muted-foreground")}>
          {info}
        </p>
      </div>
    </div>
  );
}
