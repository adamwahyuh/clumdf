import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import {
  Bold,
  Code,
  Image,
  Italic,
  Link,
  List,
  ListOrdered,
  Minus,
  Redo2,
  SeparatorHorizontal,
  Strikethrough,
  Table,
  TextQuote,
  Undo2,
  type LucideIcon,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { isMac } from "@/lib/platform";
import type { EditorActions } from "./use-editor-actions";

interface EditorToolbarProps {
  actions: EditorActions;
  /** Level judul di baris kursor: 0 = teks biasa */
  heading: number;
}

const mod = isMac ? "⌘" : "Ctrl+";

function Tool({ icon: Icon, label, shortcut, onClick }: { icon: LucideIcon; label: string; shortcut?: string; onClick: () => void }) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        {/* onMouseDown: jangan curi fokus dari editor supaya seleksi tetap ada */}
        <Button variant="ghost" size="icon-sm" aria-label={label} onMouseDown={(e) => e.preventDefault()} onClick={onClick}>
          <Icon />
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        {label}
        {shortcut && <span className="ml-2 opacity-60">{shortcut}</span>}
      </TooltipContent>
    </Tooltip>
  );
}

const Group = ({ children }: { children: ReactNode }) => (
  <div className="flex items-center gap-0.5 border-r pr-1.5 mr-1.5 last:mr-0 last:border-r-0 last:pr-0">{children}</div>
);

/** Toolbar gaya pengolah kata; setiap tombol menyisipkan sintaks Markdown yang sesuai. */
export function EditorToolbar({ actions: a, heading }: EditorToolbarProps) {
  const { t } = useTranslation();
  const k = (key: string) => t(`tools.markdown.toolbar.${key}`);

  return (
    <div role="toolbar" aria-label={k("label")} className="flex flex-wrap items-center gap-y-1 border-b bg-background px-4 py-1.5">
      <Group>
        <Tool icon={Undo2} label={k("undo")} shortcut={`${mod}Z`} onClick={a.undo} />
        <Tool icon={Redo2} label={k("redo")} shortcut={isMac ? "⇧⌘Z" : "Ctrl+Y"} onClick={a.redo} />
      </Group>
      <Group>
        <Select value={String(heading > 3 ? 3 : heading)} onValueChange={(v) => a.heading(Number(v))}>
          <SelectTrigger size="sm" className="w-36" aria-label={k("style")} onMouseDown={(e) => e.stopPropagation()}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {[0, 1, 2, 3].map((level) => (
              <SelectItem key={level} value={String(level)}>
                {level ? t("tools.markdown.toolbar.heading", { level }) : k("normal")}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </Group>
      <Group>
        <Tool icon={Bold} label={k("bold")} shortcut={`${mod}B`} onClick={a.bold} />
        <Tool icon={Italic} label={k("italic")} shortcut={`${mod}I`} onClick={a.italic} />
        <Tool icon={Strikethrough} label={k("strike")} onClick={a.strike} />
        <Tool icon={Code} label={k("code")} onClick={a.code} />
      </Group>
      <Group>
        <Tool icon={List} label={k("bullet")} onClick={a.bullet} />
        <Tool icon={ListOrdered} label={k("numbered")} onClick={a.numbered} />
        <Tool icon={TextQuote} label={k("quote")} onClick={a.quote} />
      </Group>
      <Group>
        <Tool icon={Link} label={k("link")} shortcut={`${mod}K`} onClick={a.link} />
        <Tool icon={Image} label={k("image")} onClick={a.image} />
        <Tool icon={Table} label={k("table")} onClick={a.table} />
        <Tool icon={Minus} label={k("rule")} onClick={a.rule} />
        <Tool icon={SeparatorHorizontal} label={k("pageBreak")} onClick={a.pageBreak} />
      </Group>
    </div>
  );
}
