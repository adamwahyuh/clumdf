import type { CompressSettings } from "@/stores/settings-store";

export type CompressPreset = "light" | "balanced" | "max";

export const compressPresets: Record<CompressPreset, { maxDimension: number; jpegQuality: number }> = {
  light: { maxDimension: 2400, jpegQuality: 80 },
  balanced: { maxDimension: 1600, jpegQuality: 65 },
  max: { maxDimension: 1100, jpegQuality: 45 },
};

export const compressPresetOrder: CompressPreset[] = ["light", "balanced", "max"];

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

/** Preset berkas bawaan; label di i18n `tools.compress.builtInPresets.<id>`. */
export const compressBuiltInPresets: { id: string; values: Partial<CompressSettings> }[] = [
  { id: "lamaran", values: { target: "2m", grayscale: false } },
  { id: "formulir", values: { target: "500k", grayscale: false } },
  { id: "scanHitamPutih", values: { target: "none", preset: "max", grayscale: true } },
];
