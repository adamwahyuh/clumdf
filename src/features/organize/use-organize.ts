import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { arrayMove } from "@dnd-kit/sortable";
import { toast } from "sonner";
import { withSuffix } from "@/lib/format";
import { editPages, type PagesReport } from "@/lib/tauri/commands";
import { pickSavePath } from "@/lib/tauri/dialogs";
import { errorMessage, toAppError } from "@/lib/tauri/errors";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";
import { usePageThumbnails } from "./use-page-thumbnails";

// Atur Halaman bekerja pada satu PDF; antrian dipakai ulang demi inspeksi & tanda error.
export const useOrganizeQueue = createFileQueue(inspectPdfFile);

export interface OrganizedPage {
  id: string;
  /** Nomor halaman asli, mulai 1 */
  page: number;
  /** Rotasi tambahan kumulatif (derajat); tidak dinormalkan supaya animasi tidak berbalik */
  rotate: number;
}

export function useOrganize() {
  const { t } = useTranslation();
  const file = useOrganizeQueue((s) => s.files[0]);
  const ready = file?.status === "ready";
  const [pages, setPages] = useState<OrganizedPage[]>([]);
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<{ report: PagesReport; output: string } | null>(null);
  const thumbnails = usePageThumbnails(ready ? file.path : undefined, file?.pageCount ?? 0);

  // Susunan halaman dimulai ulang setiap kali file berganti
  useEffect(() => {
    setSelected(new Set());
    setPages(ready ? Array.from({ length: file.pageCount ?? 0 }, (_, i) => ({ id: `${file.id}-${i + 1}`, page: i + 1, rotate: 0 })) : []);
  }, [file?.id, ready]); // sengaja hanya saat file/statusnya berubah, bukan tiap render

  const selectedIds = pages.filter((p) => selected.has(p.id)).map((p) => p.id);
  const allSelected = pages.length > 0 && selectedIds.length === pages.length;

  const move = (activeId: string, overId: string) =>
    setPages((ps) => arrayMove(ps, ps.findIndex((p) => p.id === activeId), ps.findIndex((p) => p.id === overId)));
  const rotate = (ids: string[], delta: number) =>
    setPages((ps) => ps.map((p) => (ids.includes(p.id) ? { ...p, rotate: p.rotate + delta } : p)));
  const remove = (ids: string[]) => {
    setPages((ps) => ps.filter((p) => !ids.includes(p.id)));
    setSelected((s) => new Set([...s].filter((id) => !ids.includes(id))));
  };
  const toggle = (id: string) =>
    setSelected((s) => {
      const next = new Set(s);
      if (!next.delete(id)) next.add(id);
      return next;
    });
  const toggleAll = () => setSelected(allSelected ? new Set() : new Set(pages.map((p) => p.id)));

  async function write(list: OrganizedPage[], suffixKey: string) {
    if (!ready || !list.length || running) return;
    const output = await pickSavePath(withSuffix(file.path, t(suffixKey)));
    if (!output) return;

    setRunning(true);
    try {
      const report = await editPages({ input: file.path, output, pages: list.map(({ page, rotate }) => ({ page, rotate })) });
      setResult({ report, output });
    } catch (err) {
      toast.error(errorMessage(t, toAppError(err)));
    } finally {
      setRunning(false);
    }
  }

  function reset() {
    setResult(null);
    useOrganizeQueue.getState().clear();
  }

  return {
    file,
    pages,
    thumbnails,
    selected,
    selectedIds,
    allSelected,
    running,
    result,
    move,
    rotate,
    remove,
    toggle,
    toggleAll,
    save: () => write(pages, "tools.organize.suffix"),
    extract: () => write(pages.filter((p) => selected.has(p.id)), "tools.organize.extractSuffix"),
    reset,
  };
}
