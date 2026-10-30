import { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { expandPaths } from "@/lib/tauri/commands";
import { pickFiles, pickFolder } from "@/lib/tauri/dialogs";
import { errorMessage, toAppError } from "@/lib/tauri/errors";
import { useOpenedFiles } from "@/stores/opened-files-store";
import { useFileDrop } from "./use-file-drop";

/**
 * Tambah file lewat drag & drop dari OS, dialog file, atau satu folder sekaligus.
 * Folder dibuka isinya di Rust; format lain dilewati dengan peringatan.
 */
export function useFileInput(extensions: readonly string[], filterName: string, add: (paths: string[]) => void) {
  const { t } = useTranslation();

  async function addExpanded(paths: string[]) {
    try {
      const { accepted, rejected } = await expandPaths(paths, extensions);
      if (rejected) toast.warning(t("files.unsupported", { count: rejected }));
      if (accepted.length) add(accepted);
      else if (!rejected) toast.info(t("files.emptyFolder"));
    } catch (err) {
      toast.error(errorMessage(t, toAppError(err)));
    }
  }

  const dragging = useFileDrop((paths) => void addExpanded(paths));

  // File yang dibuka lewat OS lalu dipilihkan fitur ini (lihat OpenWithDialog)
  const handoff = useOpenedFiles((s) => s.handoff);
  useEffect(() => {
    // Cek store, bukan hanya nilai render: StrictMode menjalankan efek dua kali
    if (!handoff || useOpenedFiles.getState().handoff !== handoff) return;
    useOpenedFiles.getState().setHandoff(null);
    void addExpanded(handoff);
  }, [handoff]); // addExpanded sengaja tidak jadi dependensi: cukup bereaksi saat handoff berubah

  async function browse() {
    const paths = await pickFiles(filterName, [...extensions]);
    if (paths.length) add(paths);
  }

  async function browseFolder() {
    const folder = await pickFolder();
    if (folder) await addExpanded([folder]);
  }

  return { dragging, browse, browseFolder };
}
