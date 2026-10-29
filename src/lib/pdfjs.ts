import { GlobalWorkerOptions, getDocument } from "pdfjs-dist";
import workerUrl from "pdfjs-dist/build/pdf.worker.min.mjs?url";
import { convertFileSrc } from "@tauri-apps/api/core";

GlobalWorkerOptions.workerSrc = workerUrl;

/** Render halaman 1 menjadi object URL PNG. Pemanggil wajib `URL.revokeObjectURL`. */
export async function renderFirstPage(path: string, width: number): Promise<string> {
  // shortcut: seluruh file dibaca ke memori hanya untuk thumbnail; pakai range request bila PDF besar terasa lambat
  const data = await fetch(convertFileSrc(path)).then((res) => res.arrayBuffer());
  const task = getDocument({ data });
  const doc = await task.promise;
  try {
    const page = await doc.getPage(1);
    const viewport = page.getViewport({ scale: width / page.getViewport({ scale: 1 }).width });
    const canvas = document.createElement("canvas");
    canvas.width = Math.ceil(viewport.width);
    canvas.height = Math.ceil(viewport.height);
    await page.render({ canvas, viewport }).promise;
    const blob = await new Promise<Blob>((resolve, reject) =>
      canvas.toBlob((b) => (b ? resolve(b) : reject(new Error("canvas kosong"))), "image/png"),
    );
    return URL.createObjectURL(blob);
  } finally {
    await task.destroy();
  }
}
