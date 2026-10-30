import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { withSuffix } from "@/lib/format";
import { readPdfMetadata, writePdfMetadata, type PdfMetadata } from "@/lib/tauri/commands";
import { pickSavePath } from "@/lib/tauri/dialogs";
import { errorMessage, toAppError } from "@/lib/tauri/errors";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";

// Satu PDF; antrian dipakai ulang demi inspeksi & tanda file terkunci/rusak.
export const useEditMetadataQueue = createFileQueue(inspectPdfFile);

export type EditableField = "title" | "author" | "subject" | "keywords" | "creator" | "producer";

export function useEditMetadata() {
  const { t } = useTranslation();
  const file = useEditMetadataQueue((s) => s.files[0]);
  const ready = file?.status === "ready";
  const [original, setOriginal] = useState<PdfMetadata | null>(null);
  const [form, setForm] = useState<PdfMetadata | null>(null);
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<{ output: string } | null>(null);

  // Baca ulang setiap kali file berganti
  useEffect(() => {
    setOriginal(null);
    setForm(null);
    if (!ready) return;
    let cancelled = false;
    readPdfMetadata(file.path).then(
      (metadata) => {
        if (cancelled) return;
        setOriginal(metadata);
        setForm(metadata);
      },
      (err) => toast.error(errorMessage(t, toAppError(err))),
    );
    return () => {
      cancelled = true;
    };
  }, [file?.id, ready]); // sengaja hanya saat file/statusnya berubah

  const changed = !!form && JSON.stringify(form) !== JSON.stringify(original);
  const setField = (field: EditableField, value: string) => setForm((f) => f && { ...f, [field]: value });

  async function save() {
    if (!ready || !form || running) return;
    const output = await pickSavePath(withSuffix(file.path, "-metadata"));
    if (!output) return;
    setRunning(true);
    try {
      await writePdfMetadata({ input: file.path, output, metadata: form });
      setResult({ output });
    } catch (err) {
      toast.error(errorMessage(t, toAppError(err)));
    } finally {
      setRunning(false);
    }
  }

  function reset() {
    setResult(null);
    useEditMetadataQueue.getState().clear();
  }

  return { file, form, changed, setField, running, result, save, reset };
}
