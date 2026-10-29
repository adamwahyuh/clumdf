export type FileKind = "pdf" | "image";

export interface QueuedFile {
  /** ID unik per entri (file yang sama boleh masuk dua kali) */
  id: string;
  path: string;
  name: string;
  status: "loading" | "ready" | "error";
  size?: number;
  pageCount?: number;
  /** Kode error dari Rust, mis. "encrypted" */
  errorCode?: string;
}

export const PDF_EXTENSIONS = ["pdf"] as const;
export const IMAGE_EXTENSIONS = ["jpg", "jpeg", "png", "webp", "bmp", "tif", "tiff", "gif"] as const;
