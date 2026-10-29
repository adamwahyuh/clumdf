import { useRef, type MouseEvent, type PointerEvent } from "react";
import { useTranslation } from "react-i18next";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { Check, FileText, RotateCcw, RotateCw, Trash2 } from "lucide-react";
import { cn } from "@/lib/utils";
import type { OrganizedPage } from "./use-organize";

interface PageCardProps {
  page: OrganizedPage;
  thumbnail?: string;
  selected: boolean;
  onToggle: () => void;
  onRotate: (delta: number) => void;
  onRemove: () => void;
}

// Tombol di dalam kartu tidak boleh memulai drag atau ikut memilih kartu
const stop = (e: PointerEvent | MouseEvent) => e.stopPropagation();

export function PageCard({ page, thumbnail, selected, onToggle, onRotate, onRemove }: PageCardProps) {
  const { t } = useTranslation();
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id: page.id });
  const label = t("tools.organize.pageLabel", { page: page.page });
  // Klik yang muncul saat drag dilepas tidak boleh ikut memilih kartu
  const dragged = useRef(false);
  if (isDragging) dragged.current = true;

  const actions = [
    { icon: RotateCcw, label: t("tools.organize.rotateLeft"), run: () => onRotate(-90) },
    { icon: RotateCw, label: t("tools.organize.rotateRight"), run: () => onRotate(90) },
    { icon: Trash2, label: t("tools.organize.delete"), run: onRemove },
  ];

  return (
    <div
      ref={setNodeRef}
      style={{
        transform: CSS.Transform.toString(transform && { ...transform, scaleX: isDragging ? 1.02 : 1, scaleY: isDragging ? 1.02 : 1 }),
        transition,
      }}
      {...attributes}
      {...listeners}
      onClick={() => {
        if (dragged.current) dragged.current = false;
        else onToggle();
      }}
      // Delete/Backspace menghapus halaman yang difokus; tombol lain untuk dnd-kit
      onKeyDown={(e) => {
        if (e.key === "Delete" || e.key === "Backspace") {
          e.preventDefault();
          onRemove();
        } else {
          listeners?.onKeyDown?.(e);
        }
      }}
      aria-label={label}
      aria-pressed={selected}
      className={cn(
        "group relative flex cursor-grab flex-col gap-2 rounded-xl bg-card p-2 shadow-card transition-shadow duration-200 ease-out",
        "focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none",
        selected && "ring-2 ring-primary",
        isDragging && "z-10 cursor-grabbing shadow-lifted",
      )}
    >
      <div className="relative flex aspect-square items-center justify-center overflow-hidden rounded-lg bg-muted p-2">
        {thumbnail ? (
          <img
            src={thumbnail}
            alt=""
            draggable={false}
            style={{ transform: `rotate(${page.rotate}deg)` }}
            className="max-h-full max-w-full object-contain shadow-card transition-transform duration-200 ease-out"
          />
        ) : (
          <FileText className="size-8 animate-pulse text-muted-foreground/50" />
        )}
        <span
          className={cn(
            "absolute top-1.5 left-1.5 flex size-5 items-center justify-center rounded-full border-2 transition-opacity duration-150",
            selected ? "border-primary bg-primary text-primary-foreground" : "border-foreground/40 bg-card/80 opacity-0 group-hover:opacity-100",
          )}
        >
          {selected && <Check className="size-3" strokeWidth={3} />}
        </span>
        <div className="absolute inset-x-0 bottom-1.5 flex justify-center gap-1 opacity-0 transition-opacity duration-150 group-hover:opacity-100 group-focus-within:opacity-100">
          {actions.map(({ icon: Icon, label: actionLabel, run }) => (
            <button
              key={actionLabel}
              type="button"
              aria-label={actionLabel}
              title={actionLabel}
              onPointerDown={stop}
              onClick={(e) => {
                stop(e);
                run();
              }}
              className="flex size-7 items-center justify-center rounded-full bg-foreground/75 text-background hover:bg-foreground focus-visible:opacity-100 focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
            >
              <Icon className="size-3.5" />
            </button>
          ))}
        </div>
      </div>
      <p className="text-center text-xs text-muted-foreground tabular-nums">{label}</p>
    </div>
  );
}
