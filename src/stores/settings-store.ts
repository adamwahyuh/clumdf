import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { CompressPreset, CompressTarget } from "@/features/compress/presets";
import type { ImageToPdfOptions, StampPosition } from "@/lib/tauri/commands";

export interface CompressSettings {
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

export type FontSizeName = "small" | "medium" | "large";
export type Align = "left" | "center" | "right";

export interface PageNumberSettings {
  /** Kunci template di i18n `tools.pageNumbers.templates` */
  format: "plain" | "page" | "ofTotal" | "pageOfTotal";
  position: StampPosition;
  fontSize: FontSizeName;
  firstNumber: number;
  skipFirst: boolean;
}

export interface HeaderFooterSettings {
  header: string;
  headerAlign: Align;
  footer: string;
  footerAlign: Align;
  fontSize: FontSizeName;
}

export interface UserPreset<T> {
  id: string;
  name: string;
  values: T;
}

interface UserPresets {
  compress: UserPreset<CompressSettings>[];
  imageToPdf: UserPreset<ImageToPdfOptions>[];
}

interface SettingsState {
  language: Language;
  compress: CompressSettings;
  imageToPdf: ImageToPdfOptions;
  split: SplitSettings;
  userPresets: UserPresets;
  pageNumbers: PageNumberSettings;
  headerFooter: HeaderFooterSettings;
  setLanguage: (language: Language) => void;
  setCompress: (patch: Partial<CompressSettings>) => void;
  setImageToPdf: (patch: Partial<ImageToPdfOptions>) => void;
  setSplit: (patch: Partial<SplitSettings>) => void;
  addUserPreset: <K extends keyof UserPresets>(tool: K, name: string, values: UserPresets[K][number]["values"]) => void;
  removeUserPreset: (tool: keyof UserPresets, id: string) => void;
  setPageNumbers: (patch: Partial<PageNumberSettings>) => void;
  setHeaderFooter: (patch: Partial<HeaderFooterSettings>) => void;
}

/** Pengaturan terakhir yang diingat antar sesi (localStorage webview). */
export const useSettings = create<SettingsState>()(
  persist(
    (set) => ({
      language: "id",
      compress: { preset: "balanced", grayscale: false, target: "none", customMb: 1 },
      imageToPdf: { pageSize: "a4", orientation: "auto", margin: "none", quality: "medium" },
      split: { mode: "eachPage", n: 2 },
      userPresets: { compress: [], imageToPdf: [] },
      pageNumbers: { format: "pageOfTotal", position: "bottomCenter", fontSize: "medium", firstNumber: 1, skipFirst: false },
      headerFooter: { header: "", headerAlign: "left", footer: "", footerAlign: "center", fontSize: "small" },
      setLanguage: (language) => set({ language }),
      setCompress: (patch) => set((s) => ({ compress: { ...s.compress, ...patch } })),
      setImageToPdf: (patch) => set((s) => ({ imageToPdf: { ...s.imageToPdf, ...patch } })),
      setSplit: (patch) => set((s) => ({ split: { ...s.split, ...patch } })),
      addUserPreset: (tool, name, values) =>
        set((s) => ({
          userPresets: { ...s.userPresets, [tool]: [...s.userPresets[tool], { id: crypto.randomUUID(), name, values }] },
        })),
      setPageNumbers: (patch) => set((s) => ({ pageNumbers: { ...s.pageNumbers, ...patch } })),
      setHeaderFooter: (patch) => set((s) => ({ headerFooter: { ...s.headerFooter, ...patch } })),
      removeUserPreset: (tool, id) =>
        set((s) => ({
          userPresets: { ...s.userPresets, [tool]: s.userPresets[tool].filter((p) => p.id !== id) },
        })),
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
          userPresets: { ...current.userPresets, ...s.userPresets },
          pageNumbers: { ...current.pageNumbers, ...s.pageNumbers },
          headerFooter: { ...current.headerFooter, ...s.headerFooter },
        };
      },
    },
  ),
);
