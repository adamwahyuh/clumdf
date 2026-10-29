import { useEffect, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { FileText } from "lucide-react";
import { renderFirstPage } from "@/lib/pdfjs";
import type { FileKind } from "@/types/files";

const THUMBNAIL_WIDTH = 320; // ±2x lebar kartu supaya tajam di layar Retina

interface FileThumbnailProps {
  path: string;
  kind: FileKind;
}

/** Gambar: tampil langsung lewat asset protocol. PDF: render halaman 1 dengan pdf.js. */
export function FileThumbnail({ path, kind }: FileThumbnailProps) {
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    if (kind !== "pdf") return;
    let cancelled = false;
    let objectUrl: string | null = null;
    renderFirstPage(path, THUMBNAIL_WIDTH).then(
      (url) => {
        if (cancelled) return URL.revokeObjectURL(url);
        objectUrl = url;
        setPdfUrl(url);
      },
      () => !cancelled && setFailed(true),
    );
    // Revoke saat kartu dihapus dari daftar supaya memori tidak bocor
    return () => {
      cancelled = true;
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  }, [path, kind]);

  const src = kind === "image" ? convertFileSrc(path) : pdfUrl;

  if (failed || !src) {
    return (
      <div className="flex size-full items-center justify-center">
        <FileText className={failed ? "size-8 text-muted-foreground" : "size-8 animate-pulse text-muted-foreground/50"} />
      </div>
    );
  }
  return (
    <img
      src={src}
      alt=""
      draggable={false}
      onError={() => setFailed(true)}
      className="size-full object-contain"
    />
  );
}
