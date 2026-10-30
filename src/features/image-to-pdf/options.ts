import type { CompressTarget } from "@/lib/compress-targets";
import type { ImageQuality, Margin, PageOrientation, PageSize } from "@/lib/tauri/commands";
import type { ImageToPdfSettings } from "@/stores/settings-store";

// Urutan pilihan di UI; label diambil dari i18n `tools.imageToPdf.<grup>.<nilai>`.
export const pageSizes: PageSize[] = ["fit", "a4", "f4", "letter", "legal"];
export const orientations: PageOrientation[] = ["auto", "portrait", "landscape"];
export const margins: Margin[] = ["none", "small", "medium", "large"];
export const qualities: ImageQuality[] = ["original", "high", "medium", "low"];
/** Target "Kecilkan hasilnya"; tanpa ukuran custom supaya tetap sederhana */
export const compressAfterOptions: CompressTarget[] = ["none", "500k", "1m", "2m"];

/** Preset berkas bawaan; label di i18n `tools.imageToPdf.builtInPresets.<id>`. */
export const imageToPdfBuiltInPresets: { id: string; values: ImageToPdfSettings }[] = [
  { id: "ktp", values: { pageSize: "a4", orientation: "portrait", margin: "medium", quality: "high", compressAfter: "none" } },
  { id: "lamaran", values: { pageSize: "a4", orientation: "auto", margin: "small", quality: "medium", compressAfter: "2m" } },
  { id: "folio", values: { pageSize: "f4", orientation: "portrait", margin: "small", quality: "medium", compressAfter: "none" } },
  { id: "foto", values: { pageSize: "fit", orientation: "auto", margin: "none", quality: "original", compressAfter: "none" } },
];
