import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { withSuffix } from "@/lib/format";
import { importSignature, placeSignature, type Placement, type SignReport } from "@/lib/tauri/commands";
import { pickFiles, pickSavePath } from "@/lib/tauri/dialogs";
import { errorMessage, toAppError } from "@/lib/tauri/errors";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";
import { useSettings } from "@/stores/settings-store";
import { IMAGE_EXTENSIONS } from "@/types/files";

// Tanda tangan ditempel ke satu PDF; antrian dipakai ulang demi inspeksi & tanda error.
export const useSignatureQueue = createFileQueue(inspectPdfFile);

export interface PlacedSignature extends Placement {
  id: string;
}

export function useSignature() {
  const { t } = useTranslation();
  const file = useSignatureQueue((s) => s.files[0]);
  const { signature, setSignature } = useSettings();
  const [page, setPage] = useState(1);
  const [placements, setPlacements] = useState<PlacedSignature[]>([]);
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<{ report: SignReport; output: string } | null>(null);

  // Posisi tanda tangan milik file lama tidak berlaku untuk file baru
  useEffect(() => {
    setPage(1);
    setPlacements([]);
  }, [file?.id]);

  async function chooseSignature() {
    const [source] = await pickFiles(t("tools.signature.imageFilter"), [...IMAGE_EXTENSIONS]);
    if (!source) return;
    try {
      setSignature(await importSignature(source));
    } catch (err) {
      toast.error(errorMessage(t, toAppError(err)));
    }
  }

  /** Tempel di halaman aktif, kanan-bawah; `pageAspect` = lebar/tinggi halaman terlihat. */
  function addOnPage(pageAspect: number) {
    if (!signature) return;
    const width = 0.3;
    const height = width * (signature.height / signature.width) * pageAspect;
    setPlacements((ps) => [
      ...ps,
      { id: crypto.randomUUID(), page, width, height, x: 1 - width - 0.08, y: Math.max(0, 1 - height - 0.1) },
    ]);
  }

  const update = (next: PlacedSignature) => setPlacements((ps) => ps.map((p) => (p.id === next.id ? next : p)));
  const remove = (id: string) => setPlacements((ps) => ps.filter((p) => p.id !== id));

  const canSave = file?.status === "ready" && !!signature && placements.length > 0 && !running;

  async function save() {
    if (!canSave) return;
    const output = await pickSavePath(withSuffix(file.path, "-signed"));
    if (!output) return;
    setRunning(true);
    try {
      const report = await placeSignature({
        input: file.path,
        output,
        signature: signature.path,
        placements: placements.map(({ page, x, y, width, height }) => ({ page, x, y, width, height })),
      });
      setResult({ report, output });
    } catch (err) {
      toast.error(errorMessage(t, toAppError(err)));
    } finally {
      setRunning(false);
    }
  }

  function reset() {
    setResult(null);
    useSignatureQueue.getState().clear();
  }

  return {
    file,
    signature,
    page,
    setPage,
    placements,
    running,
    result,
    canSave,
    chooseSignature,
    addOnPage,
    update,
    remove,
    save,
    reset,
  };
}
