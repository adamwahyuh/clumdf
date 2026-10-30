import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface HistoryEntry {
  id: string;
  /** Prefix i18n fitur, mis. "tools.merge" */
  toolKey: string;
  outputs: string[];
  /** Waktu selesai (ms epoch) */
  at: number;
}

const MAX_ENTRIES = 50;

interface HistoryState {
  entries: HistoryEntry[];
  add: (entry: Omit<HistoryEntry, "id" | "at">) => void;
  remove: (id: string) => void;
  clear: () => void;
}

/** Riwayat file yang baru diproses, terbaru di atas. */
export const useHistory = create<HistoryState>()(
  persist(
    (set) => ({
      entries: [],
      add: (entry) =>
        set((s) => ({
          entries: [{ ...entry, id: crypto.randomUUID(), at: Date.now() }, ...s.entries].slice(0, MAX_ENTRIES),
        })),
      remove: (id) => set((s) => ({ entries: s.entries.filter((e) => e.id !== id) })),
      clear: () => set({ entries: [] }),
    }),
    { name: "berkas-history", version: 1 },
  ),
);
