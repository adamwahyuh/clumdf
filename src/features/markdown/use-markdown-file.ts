import { useTranslation } from "react-i18next";
import { dirname } from "@tauri-apps/api/path";
import { ask, save } from "@tauri-apps/plugin-dialog";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { toast } from "sonner";
import { fileName } from "@/lib/format";
import { isMac, isWindows } from "@/lib/platform";
import { markdownToPdf, readTextFile, writeTextFile } from "@/lib/tauri/commands";
import { pickFiles, pickSavePath } from "@/lib/tauri/dialogs";
import { errorMessage, toAppError } from "@/lib/tauri/errors";
import { useHistory } from "@/stores/history-store";
import { useMarkdownDoc } from "@/stores/markdown-store";
import { useSettings } from "@/stores/settings-store";

export const MARKDOWN_EXTENSIONS = ["md", "markdown", "txt"] as const;

/** Buat baru, buka, simpan .md, dan simpan sebagai PDF. */
export function useMarkdownFile() {
  const { t } = useTranslation();
  const doc = useMarkdownDoc();
  const options = useSettings((s) => s.markdown);
  const fail = (err: unknown) => toast.error(errorMessage(t, toAppError(err)));
  const baseName = doc.filePath ? fileName(doc.filePath).replace(/\.[^.]+$/, "") : t("tools.markdown.untitled");

  /** Minta konfirmasi bila ada perubahan yang belum disimpan. */
  async function canDiscard() {
    if (!doc.dirty || !doc.content?.trim()) return true;
    return ask(t("tools.markdown.discardMessage"), { title: t("tools.markdown.discardTitle"), kind: "warning" });
  }

  async function newDoc() {
    if (await canDiscard()) doc.load("", null);
  }

  async function openPath(path: string) {
    if (!(await canDiscard())) return;
    try {
      doc.load(await readTextFile(path), path);
    } catch (err) {
      fail(err);
    }
  }

  async function open() {
    const [path] = await pickFiles("Markdown", [...MARKDOWN_EXTENSIONS]);
    if (path) await openPath(path);
  }

  async function saveMarkdown(saveAs = false) {
    const path =
      doc.filePath && !saveAs
        ? doc.filePath
        : await save({ defaultPath: `${baseName}.md`, filters: [{ name: "Markdown", extensions: ["md"] }] });
    if (!path) return;
    try {
      await writeTextFile(path, doc.content ?? "");
      doc.markSaved(path);
      toast.success(t("tools.markdown.saved", { name: fileName(path) }));
    } catch (err) {
      fail(err);
    }
  }

  async function exportPdf() {
    const output = await pickSavePath(`${baseName}.pdf`);
    if (!output) return;
    try {
      const baseDir = doc.filePath ? await dirname(doc.filePath) : null;
      await markdownToPdf({ markdown: doc.content ?? "", baseDir, output, options });
      useHistory.getState().add({ toolKey: "tools.markdown", outputs: [output] });
      const reveal = t(isMac ? "result.revealMac" : isWindows ? "result.revealWindows" : "result.revealOther");
      toast.success(t("tools.markdown.exported", { name: fileName(output) }), {
        action: { label: reveal, onClick: () => void revealItemInDir(output) },
      });
    } catch (err) {
      fail(err);
    }
  }

  return { baseName, newDoc, open, openPath, saveMarkdown, exportPdf };
}
