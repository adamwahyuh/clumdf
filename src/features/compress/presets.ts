import type { CompressSettings } from "@/stores/settings-store";

export type CompressPreset = "light" | "balanced" | "max";

export const compressPresets: Record<CompressPreset, { maxDimension: number; jpegQuality: number }> = {
  light: { maxDimension: 2400, jpegQuality: 80 },
  balanced: { maxDimension: 1600, jpegQuality: 65 },
  max: { maxDimension: 1100, jpegQuality: 45 },
};

export const compressPresetOrder: CompressPreset[] = ["light", "balanced", "max"];

/** Preset berkas bawaan; label di i18n `tools.compress.builtInPresets.<id>`. */
export const compressBuiltInPresets: { id: string; values: Partial<CompressSettings> }[] = [
  { id: "lamaran", values: { target: "2m", grayscale: false } },
  { id: "formulir", values: { target: "500k", grayscale: false } },
  { id: "scanHitamPutih", values: { target: "none", preset: "max", grayscale: true } },
];
