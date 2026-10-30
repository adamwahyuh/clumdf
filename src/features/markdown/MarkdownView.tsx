import { useEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { dirname } from "@tauri-apps/api/path";
import { FilePlus2, FolderOpen, Save } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ActionBar } from "@/components/common/ActionBar";
import { SegmentedControl } from "@/components/common/SegmentedControl";
import { DropOverlay } from "@/components/files/DropOverlay";
import { ToolHeader } from "@/components/layout/ToolHeader";
import { useFileDrop } from "@/hooks/use-file-drop";
import { useHotkey } from "@/hooks/use-hotkey";
import { useMarkdownDoc } from "@/stores/markdown-store";
import { useSettings } from "@/stores/settings-store";
import { IMAGE_EXTENSIONS } from "@/types/files";
import { EditorToolbar } from "./EditorToolbar";
import { MarkdownEditor } from "./MarkdownEditor";
import { MarkdownSettings } from "./MarkdownSettings";
import { PdfPreview } from "./PdfPreview";
import { useEditorActions } from "./use-editor-actions";
import { MARKDOWN_EXTENSIONS, useMarkdownFile } from "./use-markdown-file";
import { useMarkdownPreview } from "./use-markdown-preview";

type Tab = "write" | "preview";
const hasExtension = (path: string, list: readonly string[]) => list.includes(path.split(".").pop()?.toLowerCase() ?? "");

export function MarkdownView() {
  const { t } = useTranslation();
  const doc = useMarkdownDoc();
  const options = useSettings((s) => s.markdown);
  const file = useMarkdownFile();
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const actions = useEditorActions(textareaRef);
  const [tab, setTab] = useState<Tab>("write");
  const [heading, setHeading] = useState(0);
  const [baseDir, setBaseDir] = useState<string | null>(null);
  // Dokumen pertama kali: isi dengan contoh supaya langsung terlihat cara pakainya
  const content = doc.content ?? t("tools.markdown.sample");
  const preview = useMarkdownPreview(content, baseDir, options, tab === "preview");

  useEffect(() => {
    let cancelled = false;
    if (doc.filePath) dirname(doc.filePath).then((dir) => !cancelled && setBaseDir(dir));
    else setBaseDir(null);
    return () => {
      cancelled = true;
    };
  }, [doc.filePath]);

  // .md yang di-drop dibuka; gambar langsung disisipkan di posisi kursor
  const dragging = useFileDrop((paths) => {
    const markdown = paths.find((p) => hasExtension(p, MARKDOWN_EXTENSIONS));
    const images = paths.filter((p) => hasExtension(p, IMAGE_EXTENSIONS));
    if (markdown) void file.openPath(markdown);
    else if (images.length) {
      setTab("write");
      requestAnimationFrame(() => actions.insertImages(images));
    }
  });
  useHotkey(["o"], file.open);
  useHotkey(["s"], () => void file.saveMarkdown());
  useHotkey(["Enter"], file.exportPdf);

  const words = content.trim() ? content.trim().split(/\s+/).length : 0;

  return (
    <>
      <ToolHeader title={t("tools.markdown.title")} description={t("tools.markdown.description")} />
      <div className="flex min-h-0 flex-1">
        <div className="relative flex min-w-0 flex-1 flex-col border-t">
          <div className="flex items-center gap-3 px-4 py-2">
            <div className="w-52">
              <SegmentedControl
                aria-label={t("tools.markdown.tabs.label")}
                value={tab}
                onChange={setTab}
                options={[
                  { value: "write", label: t("tools.markdown.tabs.write") },
                  { value: "preview", label: t("tools.markdown.tabs.preview") },
                ]}
              />
            </div>
            <p className="min-w-0 flex-1 truncate text-sm text-muted-foreground" title={doc.filePath ?? undefined}>
              {file.baseName}
              {doc.dirty && <span className="ml-1.5 text-xs">• {t("tools.markdown.unsaved")}</span>}
            </p>
            <Button variant="ghost" size="sm" onClick={file.newDoc}>
              <FilePlus2 />
              {t("tools.markdown.new")}
            </Button>
            <Button variant="ghost" size="sm" onClick={file.open}>
              <FolderOpen />
              {t("tools.markdown.open")}
            </Button>
            <Button variant="ghost" size="sm" onClick={() => void file.saveMarkdown()}>
              <Save />
              {t("tools.markdown.save")}
            </Button>
          </div>
          {tab === "write" ? (
            <>
              <EditorToolbar actions={actions} heading={heading} />
              <MarkdownEditor
                textareaRef={textareaRef}
                value={content}
                onChange={doc.setContent}
                onHeadingChange={setHeading}
                actions={actions}
              />
            </>
          ) : (
            <PdfPreview {...preview} />
          )}
          {dragging && <DropOverlay />}
        </div>
        <MarkdownSettings />
      </div>
      <ActionBar summary={t("tools.markdown.words", { count: words })}>
        <Button onClick={file.exportPdf}>{t("tools.markdown.action")}</Button>
      </ActionBar>
    </>
  );
}
