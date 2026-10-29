import type { ImageQuality, Margin, PageOrientation, PageSize } from "@/lib/tauri/commands";

// Urutan pilihan di UI; label diambil dari i18n `tools.imageToPdf.<grup>.<nilai>`.
export const pageSizes: PageSize[] = ["fit", "a4", "f4", "letter", "legal"];
export const orientations: PageOrientation[] = ["auto", "portrait", "landscape"];
export const margins: Margin[] = ["none", "small", "medium", "large"];
export const qualities: ImageQuality[] = ["original", "high", "medium", "low"];
