import { useEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { FileText, Loader2 } from "lucide-react";
import { openPdf, renderPage } from "@/lib/pdfjs";

const RENDER_WIDTH = 1100; // ±2x lebar tampilan supaya tajam di layar Retina

interface PdfPreviewProps {
  preview: { path: string; version: number } | null;
  loading: boolean;
  error: string | null;
}

/** Halaman PDF hasil Rust ditampilkan bertumpuk seperti di penampil dokumen. */
export function PdfPreview({ preview, loading, error }: PdfPreviewProps) {
  const { t } = useTranslation();
  const [pages, setPages] = useState<string[]>([]);
  const shown = useRef<string[]>([]);

  useEffect(() => {
    if (!preview) return;
    let cancelled = false;
    (async () => {
      const pdf = await openPdf(preview.path);
      const urls: string[] = [];
      try {
        for (let n = 1; n <= pdf.doc.numPages && !cancelled; n++) urls.push(await renderPage(pdf.doc, n, RENDER_WIDTH));
      } finally {
        await pdf.close();
      }
      if (cancelled) return urls.forEach((u) => URL.revokeObjectURL(u));
      // Ganti sekaligus setelah semua halaman siap supaya tidak berkedip saat mengetik
      shown.current.forEach((u) => URL.revokeObjectURL(u));
      shown.current = urls;
      setPages(urls);
    })().catch(() => {});
    return () => {
      cancelled = true;
    };
  }, [preview]);

  useEffect(() => () => shown.current.forEach((u) => URL.revokeObjectURL(u)), []);

  return (
    <div className="relative min-h-0 flex-1 overflow-y-auto bg-muted/50 px-6 py-6">
      {loading && (
        <span className="absolute top-3 right-4 z-10 flex items-center gap-1.5 rounded-full bg-card px-2.5 py-1 text-xs text-muted-foreground shadow-card">
          <Loader2 className="size-3 animate-spin" />
          {t("tools.markdown.updating")}
        </span>
      )}
      {error ? (
        <p className="mx-auto max-w-md rounded-lg bg-card p-4 text-center text-sm text-destructive shadow-card">{error}</p>
      ) : pages.length ? (
        <div className="mx-auto flex max-w-[640px] flex-col gap-6">
          {pages.map((url, i) => (
            <img key={url} src={url} alt={t("tools.markdown.pageAlt", { page: i + 1 })} className="w-full bg-white shadow-lifted" />
          ))}
        </div>
      ) : (
        <div className="flex h-full items-center justify-center">
          <FileText className="size-10 animate-pulse text-muted-foreground/50" />
        </div>
      )}
    </div>
  );
}
