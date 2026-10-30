import { useTranslation } from "react-i18next";
import { FileText } from "lucide-react";
import { Input } from "@/components/ui/input";
import type { PdfMetadata } from "@/lib/tauri/commands";
import { parsePdfDate } from "./pdf-date";
import type { EditableField } from "./use-edit-metadata";

const fields: EditableField[] = ["title", "author", "subject", "keywords", "creator", "producer"];

interface MetadataFormProps {
  fileName: string;
  form: PdfMetadata;
  onChange: (field: EditableField, value: string) => void;
}

/** Formulir info dokumen; kolom yang dikosongkan dihapus dari PDF. */
export function MetadataForm({ fileName, form, onChange }: MetadataFormProps) {
  const { t, i18n } = useTranslation();
  const k = (key: string) => t(`tools.editMetadata.${key}`);
  const date = (raw: string | null) =>
    parsePdfDate(raw)?.toLocaleString(i18n.language, { dateStyle: "long", timeStyle: "short" }) ?? k("unknown");

  return (
    <div className="mx-auto flex max-w-xl flex-col gap-5 rounded-xl bg-card p-6 shadow-card">
      <div className="flex items-center gap-3">
        <span className="flex size-10 shrink-0 items-center justify-center rounded-lg bg-primary/10">
          <FileText className="size-5 text-primary" />
        </span>
        <div className="min-w-0">
          <p className="truncate font-medium" title={fileName}>
            {fileName}
          </p>
          <p className="text-xs text-muted-foreground">
            {k("created")}: {date(form.creationDate)} · {k("modified")}: {date(form.modDate)}
          </p>
        </div>
      </div>
      <div className="grid grid-cols-[auto_1fr] items-center gap-x-4 gap-y-3">
        {fields.map((field) => (
          <label key={field} className="contents">
            <span className="text-right text-[13px] font-medium">{k(`fields.${field}`)}</span>
            <Input
              value={form[field] ?? ""}
              onChange={(e) => onChange(field, e.target.value)}
              placeholder={k(`placeholders.${field}`)}
              autoFocus={field === "title"}
            />
          </label>
        ))}
      </div>
      <p className="text-xs text-muted-foreground">{k("hint")}</p>
    </div>
  );
}
