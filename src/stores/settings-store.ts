import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { CompressPreset } from "@/features/compress/presets";
import type { ImageToPdfOptions } from "@/lib/tauri/commands";

interface CompressSettings {
  preset: CompressPreset;
  grayscale: boolean;
}

export type Language = "id" | "en";

interface SettingsState {
  language: Language;
  compress: CompressSettings;
  imageToPdf: ImageToPdfOptions;
  setLanguage: (language: Language) => void;
  setCompress: (patch: Partial<CompressSettings>) => void;
  setImageToPdf: (patch: Partial<ImageToPdfOptions>) => void;
}

/** Pengaturan terakhir yang diingat antar sesi (localStorage webview). */
export const useSettings = create<SettingsState>()(
  persist(
    (set) => ({
      language: "id",
      compress: { preset: "balanced", grayscale: false },
      imageToPdf: { pageSize: "a4", orientation: "auto", margin: "none", quality: "medium" },
      setLanguage: (language) => set({ language }),
      setCompress: (patch) => set((s) => ({ compress: { ...s.compress, ...patch } })),
      setImageToPdf: (patch) => set((s) => ({ imageToPdf: { ...s.imageToPdf, ...patch } })),
    }),
    {
      name: "berkas-settings",
      version: 1,
      // Gabung per bagian supaya pengaturan baru tetap punya default saat data lama dimuat
      merge: (saved, current) => {
        const s = (saved ?? {}) as Partial<SettingsState>;
        return {
          ...current,
          compress: { ...current.compress, ...s.compress },
          imageToPdf: { ...current.imageToPdf, ...s.imageToPdf },
        };
      },
    },
  ),
);
