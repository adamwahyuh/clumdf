import { useEffect, useState } from "react";
import { openPdf, renderPage } from "@/lib/pdfjs";

const THUMBNAIL_WIDTH = 240;

/** Thumbnail semua halaman (kunci = nomor halaman asli), dirender berurutan. */
export function usePageThumbnails(path: string | undefined, pageCount: number): Record<number, string> {
  const [thumbnails, setThumbnails] = useState<Record<number, string>>({});

  useEffect(() => {
    if (!path || !pageCount) return;
    let cancelled = false;
    const urls: string[] = [];

    (async () => {
      const pdf = await openPdf(path);
      try {
        // Satu per satu supaya halaman awal cepat muncul dan memori tetap kecil
        for (let n = 1; n <= pageCount && !cancelled; n++) {
          const url = await renderPage(pdf.doc, n, THUMBNAIL_WIDTH);
          if (cancelled) {
            URL.revokeObjectURL(url);
            break;
          }
          urls.push(url);
          setThumbnails((prev) => ({ ...prev, [n]: url }));
        }
      } finally {
        await pdf.close();
      }
    })().catch(() => {
      // Halaman yang gagal dirender tetap tampil dengan ikon pengganti
    });

    return () => {
      cancelled = true;
      urls.forEach((url) => URL.revokeObjectURL(url));
      setThumbnails({});
    };
  }, [path, pageCount]);

  return thumbnails;
}
