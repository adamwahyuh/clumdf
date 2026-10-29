import { create } from "zustand";
import { arrayMove } from "@dnd-kit/sortable";
import { fileName } from "@/lib/format";
import { toAppError } from "@/lib/tauri/errors";
import type { QueuedFile } from "@/types/files";

/** Ambil info file dari Rust; error-nya menandai file sebagai bermasalah. */
export type InspectFile = (path: string) => Promise<Partial<QueuedFile>>;

export interface FileQueue {
  files: QueuedFile[];
  add: (paths: string[]) => void;
  remove: (id: string) => void;
  move: (activeId: string, overId: string) => void;
  clear: () => void;
}

/** Factory: tiap fitur punya antrian file sendiri. */
export function createFileQueue(inspect?: InspectFile) {
  return create<FileQueue>()((set) => {
    const update = (id: string, patch: Partial<QueuedFile>) =>
      set((s) => ({ files: s.files.map((f) => (f.id === id ? { ...f, ...patch } : f)) }));

    return {
      files: [],
      add: (paths) => {
        const added: QueuedFile[] = paths.map((path) => ({
          id: crypto.randomUUID(),
          path,
          name: fileName(path),
          status: inspect ? "loading" : "ready",
        }));
        set((s) => ({ files: [...s.files, ...added] }));
        if (!inspect) return;
        for (const file of added) {
          inspect(file.path).then(
            (info) => update(file.id, { ...info, status: "ready" }),
            (err) => update(file.id, { status: "error", errorCode: toAppError(err).code }),
          );
        }
      },
      remove: (id) => set((s) => ({ files: s.files.filter((f) => f.id !== id) })),
      move: (activeId, overId) =>
        set((s) => {
          const from = s.files.findIndex((f) => f.id === activeId);
          const to = s.files.findIndex((f) => f.id === overId);
          return from < 0 || to < 0 ? s : { files: arrayMove(s.files, from, to) };
        }),
      clear: () => set({ files: [] }),
    };
  });
}
