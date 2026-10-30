import type { RefObject } from "react";
import { useTranslation } from "react-i18next";
import { fileName } from "@/lib/format";
import { pickFiles } from "@/lib/tauri/dialogs";
import { useMarkdownDoc } from "@/stores/markdown-store";
import { IMAGE_EXTENSIONS } from "@/types/files";
import {
  applyEdit,
  insertBlock,
  insertLink,
  setHeading,
  toggleLines,
  toggleWrap,
  type Edit,
  type LineKind,
  type TextState,
} from "./md-format";

/** Penanda pemisah halaman yang dikenali pdf-core. */
const PAGE_BREAK = "<!-- pagebreak -->";

/** `![nama](<path>)`: kurung sudut supaya path berspasi tetap sah; "/" juga diterima Windows. */
function imageMarkdown(path: string): string {
  const name = fileName(path).replace(/\.[^.]+$/, "");
  return `![${name}](<${path.replace(/\\/g, "/")}>)`;
}

export function useEditorActions(ref: RefObject<HTMLTextAreaElement | null>) {
  const { t } = useTranslation();
  const setContent = useMarkdownDoc((s) => s.setContent);

  /** Terapkan satu perubahan; `false` bila tidak ada yang diubah. */
  function apply(make: (s: TextState) => Edit | null): boolean {
    const el = ref.current;
    if (!el) return false;
    const edit = make({ value: el.value, start: el.selectionStart, end: el.selectionEnd });
    if (!edit) return false;
    el.focus();
    el.setSelectionRange(edit.from, edit.to);
    // execCommand menjaga riwayat undo bawaan textarea (⌘Z tetap jalan)
    const done = edit.insert
      ? document.execCommand("insertText", false, edit.insert)
      : edit.from === edit.to || document.execCommand("delete");
    if (done) {
      el.setSelectionRange(edit.selStart, edit.selEnd);
    } else {
      setContent(applyEdit(el.value, edit));
      requestAnimationFrame(() => el.setSelectionRange(edit.selStart, edit.selEnd));
    }
    return true;
  }

  const wrap = (marker: string, key: string) => () => apply((s) => toggleWrap(s, marker, t(`tools.markdown.placeholders.${key}`)));
  const lines = (kind: LineKind) => () => apply((s) => toggleLines(s, kind));
  const block = (text: string) => apply((s) => insertBlock(s, text));
  const history = (command: "undo" | "redo") => () => {
    ref.current?.focus();
    document.execCommand(command);
  };

  const columns = [1, 2, 3].map((n) => t("tools.markdown.placeholders.column", { n }));
  const table = `| ${columns.join(" | ")} |\n|---|---|---|\n|  |  |  |\n|  |  |  |`;

  return {
    apply,
    bold: wrap("**", "bold"),
    italic: wrap("_", "italic"),
    strike: wrap("~~", "strike"),
    code: wrap("`", "code"),
    heading: (level: number) => apply((s) => setHeading(s, level)),
    bullet: lines("bullet"),
    numbered: lines("numbered"),
    quote: lines("quote"),
    link: () => apply((s) => insertLink(s, t("tools.markdown.placeholders.link"))),
    table: () => block(table),
    rule: () => block("---"),
    pageBreak: () => block(PAGE_BREAK),
    insertImages: (paths: string[]) => block(paths.map(imageMarkdown).join("\n\n")),
    image: async () => {
      const paths = await pickFiles(t("tools.imageToPdf.filterName"), [...IMAGE_EXTENSIONS]);
      if (paths.length) block(paths.map(imageMarkdown).join("\n\n"));
    },
    undo: history("undo"),
    redo: history("redo"),
  };
}

export type EditorActions = ReturnType<typeof useEditorActions>;
