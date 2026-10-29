import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { CompressPreset } from "@/features/compress/presets";

interface CompressSettings {
  preset: CompressPreset;
  grayscale: boolean;
}

interface SettingsState {
  compress: CompressSettings;
  setCompress: (patch: Partial<CompressSettings>) => void;
}

/** Pengaturan terakhir yang diingat antar sesi (localStorage webview). */
export const useSettings = create<SettingsState>()(
  persist(
    (set) => ({
      compress: { preset: "balanced", grayscale: false },
      setCompress: (patch) => set((s) => ({ compress: { ...s.compress, ...patch } })),
    }),
    { name: "berkas-settings", version: 1 },
  ),
);
