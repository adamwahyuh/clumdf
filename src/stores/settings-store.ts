import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { CompressPreset } from "@/features/compress/presets";
import type { ImageToPdfOptions } from "@/lib/tauri/commands";

interface CompressSettings {
  preset: CompressPreset;
  grayscale: boolean;
}

interface SettingsState {
  compress: CompressSettings;
  imageToPdf: ImageToPdfOptions;
  setCompress: (patch: Partial<CompressSettings>) => void;
  setImageToPdf: (patch: Partial<ImageToPdfOptions>) => void;
}

/** Pengaturan terakhir yang diingat antar sesi (localStorage webview). */
export const useSettings = create<SettingsState>()(
  persist(
    (set) => ({
      compress: { preset: "balanced", grayscale: false },
      imageToPdf: { pageSize: "a4", orientation: "auto", margin: "none", quality: "medium" },
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
