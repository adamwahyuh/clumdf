import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { pickFiles } from "@/lib/tauri/dialogs";
import { useFileDrop } from "./use-file-drop";

/** Tambah file lewat drag & drop dari OS atau dialog; format lain dilewati dengan peringatan. */
export function useFileInput(extensions: readonly string[], filterName: string, add: (paths: string[]) => void) {
  const { t } = useTranslation();

  const dragging = useFileDrop(extensions, (paths, rejected) => {
    if (rejected) toast.warning(t("files.unsupported", { count: rejected }));
    if (paths.length) add(paths);
  });

  async function browse() {
    const paths = await pickFiles(filterName, [...extensions]);
    if (paths.length) add(paths);
  }

  return { dragging, browse };
}
