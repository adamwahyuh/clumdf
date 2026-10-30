import type { KeyboardEvent, RefObject } from "react";
import { useTranslation } from "react-i18next";
import { isMac } from "@/lib/platform";
import { continueList, currentHeading } from "./md-format";
import type { EditorActions } from "./use-editor-actions";

interface MarkdownEditorProps {
  textareaRef: RefObject<HTMLTextAreaElement | null>;
  value: string;
  onChange: (value: string) => void;
  /** Level judul di baris kursor, untuk pilihan gaya di toolbar */
  onHeadingChange: (level: number) => void;
  actions: EditorActions;
}

/** Area tulis di atas "kertas" putih, dengan shortcut format seperti pengolah kata. */
export function MarkdownEditor({ textareaRef, value, onChange, onHeadingChange, actions }: MarkdownEditorProps) {
  const { t } = useTranslation();

  function onKeyDown(e: KeyboardEvent<HTMLTextAreaElement>) {
    const mod = isMac ? e.metaKey : e.ctrlKey;
    const shortcuts: Record<string, () => unknown> = { b: actions.bold, i: actions.italic, k: actions.link };
    if (mod && !e.shiftKey && !e.altKey && shortcuts[e.key.toLowerCase()]) {
      e.preventDefault();
      shortcuts[e.key.toLowerCase()]();
    } else if (e.key === "Enter" && !e.shiftKey && !mod && !e.nativeEvent.isComposing) {
      // Lanjutkan daftar otomatis; Shift+Enter tetap baris baru biasa
      if (actions.apply(continueList)) e.preventDefault();
    }
  }

  function reportHeading() {
    const el = textareaRef.current;
    if (el) onHeadingChange(currentHeading({ value: el.value, start: el.selectionStart, end: el.selectionEnd }));
  }

  return (
    <div className="min-h-0 flex-1 overflow-y-auto bg-muted/50 px-6 py-6">
      <div className="mx-auto flex min-h-full max-w-[760px] rounded-lg bg-card px-12 py-10 shadow-card">
        <textarea
          ref={textareaRef}
          value={value}
          onChange={(e) => {
            onChange(e.target.value);
            reportHeading();
          }}
          onSelect={reportHeading}
          onKeyDown={onKeyDown}
          aria-label={t("tools.markdown.editorLabel")}
          placeholder={t("tools.markdown.placeholder")}
          spellCheck
          autoFocus
          className="min-h-[60vh] w-full flex-1 resize-none bg-transparent text-[15px] leading-7 text-card-foreground outline-none select-text field-sizing-content placeholder:text-muted-foreground"
        />
      </div>
    </div>
  );
}
