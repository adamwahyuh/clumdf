import { create } from "zustand";

interface OpenedFilesState {
  /** File dari OS yang sudah dipilihkan fiturnya, menunggu diambil layar fitur itu */
  handoff: string[] | null;
  setHandoff: (paths: string[] | null) => void;
}

export const useOpenedFiles = create<OpenedFilesState>()((set) => ({
  handoff: null,
  setHandoff: (handoff) => set({ handoff }),
}));
