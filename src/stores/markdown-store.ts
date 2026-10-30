import { create } from "zustand";
import { persist } from "zustand/middleware";

interface MarkdownDoc {
  /** null = belum pernah menulis; editor diisi contoh dokumen */
  content: string | null;
  /** File .md asal/tujuan simpan; null = belum disimpan */
  filePath: string | null;
  /** Ada perubahan yang belum disimpan ke filePath */
  dirty: boolean;
  setContent: (content: string) => void;
  load: (content: string, filePath: string | null) => void;
  markSaved: (filePath: string) => void;
}

/** Draf editor Markdown; tersimpan otomatis supaya tidak hilang saat app ditutup. */
export const useMarkdownDoc = create<MarkdownDoc>()(
  persist(
    (set) => ({
      content: null,
      filePath: null,
      dirty: false,
      setContent: (content) => set({ content, dirty: true }),
      load: (content, filePath) => set({ content, filePath, dirty: false }),
      markSaved: (filePath) => set({ filePath, dirty: false }),
    }),
    { name: "berkas-markdown-draft", version: 1 },
  ),
);
