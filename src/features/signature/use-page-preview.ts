import { useEffect, useState } from "react";
import type { PDFDocumentProxy } from "pdfjs-dist";
import { openPdf, renderPage } from "@/lib/pdfjs";

const PREVIEW_WIDTH = 1000;

/** Pratinjau satu halaman besar; PDF dibuka sekali per file, halaman dirender saat dipilih. */
export function usePagePreview(path: string | undefined, pageNumber: number) {
  const [doc, setDoc] = useState<PDFDocumentProxy | null>(null);
  const [preview, setPreview] = useState<{ url: string; aspect: number } | null>(null);

  useEffect(() => {
    if (!path) return;
    let close: (() => Promise<void>) | undefined;
    let cancelled = false;
    openPdf(path).then(
      (pdf) => {
        close = pdf.close;
        if (cancelled) void pdf.close();
        else setDoc(pdf.doc);
      },
      () => {},
    );
    return () => {
      cancelled = true;
      setDoc(null);
      void close?.();
    };
  }, [path]);

  useEffect(() => {
    if (!doc) return;
    let cancelled = false;
    let url: string | undefined;
    renderPage(doc, pageNumber, PREVIEW_WIDTH).then((rendered) => {
      if (cancelled) return URL.revokeObjectURL(rendered);
      url = rendered;
      // Rasio dari gambar hasil render (sudah memperhitungkan /Rotate)
      const img = new Image();
      img.onload = () => !cancelled && setPreview({ url: rendered, aspect: img.naturalWidth / img.naturalHeight });
      img.src = rendered;
    }, () => {});
    return () => {
      cancelled = true;
      if (url) URL.revokeObjectURL(url);
      setPreview(null);
    };
  }, [doc, pageNumber]);

  return preview;
}
