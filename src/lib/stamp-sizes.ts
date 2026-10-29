import type { FontSizeName } from "@/stores/settings-store";

/** Ukuran huruf (pt) untuk pilihan Kecil/Sedang/Besar; dipakai nomor halaman & header/footer. */
export const fontSizes: Record<FontSizeName, number> = { small: 9, medium: 11, large: 14 };
export const fontSizeOrder: FontSizeName[] = ["small", "medium", "large"];

/** Jarak teks dari tepi halaman */
export const STAMP_MARGIN_MM = 12;
