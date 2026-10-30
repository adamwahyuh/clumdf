// Target ukuran file; dipakai Kecilkan PDF dan Gambar ke PDF (workflow berantai).

export type CompressTarget = "none" | "500k" | "1m" | "2m" | "custom";

export const compressTargetOrder: CompressTarget[] = ["none", "500k", "1m", "2m", "custom"];

// Desimal (1 KB = 1000 byte): lebih ketat dari 1024, jadi lolos batas unggah mana pun
const targetBytes: Record<Exclude<CompressTarget, "none" | "custom">, number> = {
  "500k": 500_000,
  "1m": 1_000_000,
  "2m": 2_000_000,
};

/** Target dalam byte, atau null bila tanpa target. */
export function resolveTarget(target: CompressTarget, customMb: number): number | null {
  if (target === "none") return null;
  // Dibulatkan: Rust menerima byte sebagai bilangan bulat (u64)
  if (target === "custom") return Math.round(Math.max(0.05, customMb) * 1_000_000);
  return targetBytes[target];
}
