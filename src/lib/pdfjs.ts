import { GlobalWorkerOptions, getDocument, type PDFDocumentProxy } from "pdfjs-dist";
import workerUrl from "pdfjs-dist/build/pdf.worker.min.mjs?url";
import { convertFileSrc } from "@tauri-apps/api/core";

GlobalWorkerOptions.workerSrc = workerUrl;

/** Buka PDF lokal untuk dirender; panggil `close()` setelah selesai. */
export async function openPdf(path: string): Promise<{ doc: PDFDocumentProxy; close: () => Promise<void> }> {
  // shortcut: seluruh file dibaca ke memori hanya untuk thumbnail; pakai range request bila PDF besar terasa lambat
  // no-store: file pratinjau ditimpa di path yang sama, jangan pakai salinan cache
  const data = await fetch(convertFileSrc(path), { cache: "no-store" }).then((res) => res.arrayBuffer());
  const task = getDocument({ data });
  return { doc: await task.promise, close: () => task.destroy() };
}

/** Render satu halaman (mulai 1) menjadi object URL PNG. Pemanggil wajib `URL.revokeObjectURL`. */
export async function renderPage(doc: PDFDocumentProxy, pageNumber: number, width: number): Promise<string> {
  const page = await doc.getPage(pageNumber);
  const viewport = page.getViewport({ scale: width / page.getViewport({ scale: 1 }).width });
  const canvas = document.createElement("canvas");
  canvas.width = Math.ceil(viewport.width);
  canvas.height = Math.ceil(viewport.height);
  await page.render({ canvas, viewport }).promise;
  const blob = await new Promise<Blob>((resolve, reject) =>
    canvas.toBlob((b) => (b ? resolve(b) : reject(new Error("canvas kosong"))), "image/png"),
  );
  return URL.createObjectURL(blob);
}

/** Render halaman 1 menjadi object URL PNG. Pemanggil wajib `URL.revokeObjectURL`. */
export async function renderFirstPage(path: string, width: number): Promise<string> {
  const pdf = await openPdf(path);
  try {
    return await renderPage(pdf.doc, 1, width);
  } finally {
    await pdf.close();
  }
}
