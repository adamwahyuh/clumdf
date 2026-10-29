import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { CompressPreset, CompressTarget } from "@/features/compress/presets";
import type { ImageToPdfOptions } from "@/lib/tauri/commands";

interface CompressSettings {
  preset: CompressPreset;
  grayscale: boolean;
  target: CompressTarget;
  /** Target custom dalam MB */
  customMb: number;
}

export type Language = "id" | "en";

export type SplitModeName = "eachPage" | "ranges" | "everyN";

interface SplitSettings {
  mode: SplitModeName;
  /** Jumlah halaman per file untuk mode "everyN" */
  n: number;
}

interface SettingsState {
  language: Language;
  compress: CompressSettings;
  imageToPdf: ImageToPdfOptions;
  split: SplitSettings;
  setLanguage: (language: Language) => void;
  setCompress: (patch: Partial<CompressSettings>) => void;
  setImageToPdf: (patch: Partial<ImageToPdfOptions>) => void;
  setSplit: (patch: Partial<SplitSettings>) => void;
}

/** Pengaturan terakhir yang diingat antar sesi (localStorage webview). */
export const useSettings = create<SettingsState>()(
  persist(
    (set) => ({
      language: "id",
      compress: { preset: "balanced", grayscale: false, target: "none", customMb: 1 },
      imageToPdf: { pageSize: "a4", orientation: "auto", margin: "none", quality: "medium" },
      split: { mode: "eachPage", n: 2 },
      setLanguage: (language) => set({ language }),
      setCompress: (patch) => set((s) => ({ compress: { ...s.compress, ...patch } })),
      setImageToPdf: (patch) => set((s) => ({ imageToPdf: { ...s.imageToPdf, ...patch } })),
      setSplit: (patch) => set((s) => ({ split: { ...s.split, ...patch } })),
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
          split: { ...current.split, ...s.split },
        };
      },
    },
  ),
);
