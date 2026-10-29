import type { ImageQuality, ImageToPdfOptions, Margin, PageOrientation, PageSize } from "@/lib/tauri/commands";

// Urutan pilihan di UI; label diambil dari i18n `tools.imageToPdf.<grup>.<nilai>`.
export const pageSizes: PageSize[] = ["fit", "a4", "f4", "letter", "legal"];
export const orientations: PageOrientation[] = ["auto", "portrait", "landscape"];
export const margins: Margin[] = ["none", "small", "medium", "large"];
export const qualities: ImageQuality[] = ["original", "high", "medium", "low"];

/** Preset berkas bawaan; label di i18n `tools.imageToPdf.builtInPresets.<id>`. */
export const imageToPdfBuiltInPresets: { id: string; values: ImageToPdfOptions }[] = [
  { id: "ktp", values: { pageSize: "a4", orientation: "portrait", margin: "medium", quality: "high" } },
  { id: "lamaran", values: { pageSize: "a4", orientation: "auto", margin: "small", quality: "medium" } },
  { id: "folio", values: { pageSize: "f4", orientation: "portrait", margin: "small", quality: "medium" } },
  { id: "foto", values: { pageSize: "fit", orientation: "auto", margin: "none", quality: "original" } },
];
