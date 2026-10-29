import { useTranslation } from "react-i18next";
import { ChevronLeft, ChevronRight, FileText } from "lucide-react";
import { Button } from "@/components/ui/button";
import { PlacementBox } from "./PlacementBox";
import type { PlacedSignature } from "./use-signature";

interface PagePreviewProps {
  preview: { url: string; aspect: number } | null;
  page: number;
  pageCount: number;
  onPageChange: (page: number) => void;
  placements: PlacedSignature[];
  signatureUrl?: string;
  /** tinggi/lebar tanda tangan dalam piksel */
  signatureRatio: number;
  onChange: (placement: PlacedSignature) => void;
  onRemove: (id: string) => void;
}

/** Satu halaman besar + tanda tangan yang sudah ditempel di halaman itu. */
export function PagePreview(props: PagePreviewProps) {
  const { t } = useTranslation();
  const { preview, page, pageCount } = props;

  return (
    <div className="flex flex-col items-center gap-3">
      <div className="flex items-center gap-2">
        <Button variant="ghost" size="icon-sm" disabled={page <= 1} onClick={() => props.onPageChange(page - 1)} aria-label={t("tools.signature.prevPage")}>
          <ChevronLeft />
        </Button>
        <span className="min-w-28 text-center text-sm tabular-nums text-muted-foreground">
          {t("tools.signature.pageOf", { page, total: pageCount })}
        </span>
        <Button variant="ghost" size="icon-sm" disabled={page >= pageCount} onClick={() => props.onPageChange(page + 1)} aria-label={t("tools.signature.nextPage")}>
          <ChevronRight />
        </Button>
      </div>
      {preview ? (
        // Kotak ini persis seukuran gambar halaman, jadi posisi % = pecahan halaman
        <div className="relative w-full max-w-[640px] shadow-lifted select-none" style={{ aspectRatio: preview.aspect }}>
          <img src={preview.url} alt="" draggable={false} className="size-full bg-white" />
          {props.signatureUrl &&
            props.placements
              .filter((p) => p.page === page)
              .map((p) => (
                <PlacementBox
                  key={p.id}
                  placement={p}
                  imageUrl={props.signatureUrl!}
                  ratio={props.signatureRatio * preview.aspect}
                  onChange={props.onChange}
                  onRemove={() => props.onRemove(p.id)}
                />
              ))}
        </div>
      ) : (
        <div className="flex aspect-[3/4] w-full max-w-[640px] items-center justify-center rounded-md bg-muted">
          <FileText className="size-10 animate-pulse text-muted-foreground/50" />
        </div>
      )}
    </div>
  );
}
