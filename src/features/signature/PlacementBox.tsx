import { useRef, type KeyboardEvent, type PointerEvent as ReactPointerEvent } from "react";
import { useTranslation } from "react-i18next";
import { X } from "lucide-react";
import { cn } from "@/lib/utils";
import type { PlacedSignature } from "./use-signature";

interface PlacementBoxProps {
  placement: PlacedSignature;
  imageUrl: string;
  /** tinggi/lebar dalam satuan pecahan halaman, supaya rasio gambar tetap */
  ratio: number;
  onChange: (placement: PlacedSignature) => void;
  onRemove: () => void;
}

const clamp = (v: number, min: number, max: number) => Math.min(Math.max(v, min), max);
const MIN_WIDTH = 0.05;

/** Tanda tangan di atas pratinjau: geser, ubah ukuran (sudut kanan-bawah), atau pakai keyboard. */
export function PlacementBox({ placement: p, imageUrl, ratio, onChange, onRemove }: PlacementBoxProps) {
  const { t } = useTranslation();
  const ref = useRef<HTMLDivElement>(null);

  const moveTo = (x: number, y: number) => onChange({ ...p, x: clamp(x, 0, 1 - p.width), y: clamp(y, 0, 1 - p.height) });
  const resizeTo = (width: number) => {
    // Lebar dibatasi tepi kanan & bawah halaman, tinggi mengikuti rasio gambar
    const w = clamp(width, MIN_WIDTH, Math.min(1 - p.x, (1 - p.y) / ratio));
    onChange({ ...p, width: w, height: w * ratio });
  };

  function startDrag(e: ReactPointerEvent, mode: "move" | "resize") {
    e.preventDefault();
    e.stopPropagation();
    ref.current?.focus();
    const page = ref.current!.parentElement!.getBoundingClientRect();
    const start = { x: e.clientX, y: e.clientY };
    const onMove = (ev: PointerEvent) => {
      const dx = (ev.clientX - start.x) / page.width;
      const dy = (ev.clientY - start.y) / page.height;
      if (mode === "move") moveTo(p.x + dx, p.y + dy);
      else resizeTo(p.width + dx);
    };
    const onUp = () => {
      window.removeEventListener("pointermove", onMove);
      window.removeEventListener("pointerup", onUp);
    };
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
  }

  function onKeyDown(e: KeyboardEvent) {
    const step = e.shiftKey ? 0.05 : 0.01;
    const moves: Record<string, [number, number]> = {
      ArrowLeft: [-step, 0],
      ArrowRight: [step, 0],
      ArrowUp: [0, -step],
      ArrowDown: [0, step],
    };
    if (moves[e.key]) moveTo(p.x + moves[e.key][0], p.y + moves[e.key][1]);
    else if (e.key === "+" || e.key === "=") resizeTo(p.width + step);
    else if (e.key === "-") resizeTo(p.width - step);
    else if (e.key === "Delete" || e.key === "Backspace") onRemove();
    else return;
    e.preventDefault();
  }

  return (
    <div
      ref={ref}
      tabIndex={0}
      role="group"
      aria-label={t("tools.signature.boxLabel")}
      onPointerDown={(e) => startDrag(e, "move")}
      onKeyDown={onKeyDown}
      style={{ left: `${p.x * 100}%`, top: `${p.y * 100}%`, width: `${p.width * 100}%`, height: `${p.height * 100}%` }}
      className={cn(
        "group absolute cursor-move touch-none rounded-sm outline-1 outline-primary/60 outline-dashed",
        "hover:outline-primary focus-visible:outline-2 focus-visible:outline-solid focus-visible:outline-primary",
      )}
    >
      <img src={imageUrl} alt="" draggable={false} className="pointer-events-none size-full object-contain" />
      <button
        type="button"
        aria-label={t("tools.signature.remove")}
        onPointerDown={(e) => e.stopPropagation()}
        onClick={onRemove}
        className="absolute -top-2.5 -right-2.5 flex size-5 items-center justify-center rounded-full bg-destructive text-white opacity-0 shadow-card transition-opacity group-hover:opacity-100 group-focus-within:opacity-100 focus-visible:opacity-100"
      >
        <X className="size-3" />
      </button>
      <span
        aria-hidden
        onPointerDown={(e) => startDrag(e, "resize")}
        className="absolute -right-1.5 -bottom-1.5 size-3 cursor-nwse-resize rounded-full border-2 border-primary bg-background"
      />
    </div>
  );
}
