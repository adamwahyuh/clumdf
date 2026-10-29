export type CompressPreset = "light" | "balanced" | "max";

export const compressPresets: Record<CompressPreset, { maxDimension: number; jpegQuality: number }> = {
  light: { maxDimension: 2400, jpegQuality: 80 },
  balanced: { maxDimension: 1600, jpegQuality: 65 },
  max: { maxDimension: 1100, jpegQuality: 45 },
};

export const compressPresetOrder: CompressPreset[] = ["light", "balanced", "max"];
