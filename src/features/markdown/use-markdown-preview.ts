import { useEffect, useState } from "react";
import { markdownPreview, type MarkdownOptions } from "@/lib/tauri/commands";
import { errorMessage, toAppError } from "@/lib/tauri/errors";
import { useTranslation } from "react-i18next";

const DEBOUNCE_MS = 500;

/**
 * PDF pratinjau dibuat ulang (oleh Rust) setiap kali isi/pengaturan berubah,
 * tetapi hanya selama tab Pratinjau terbuka dan setelah jeda mengetik.
 */
export function useMarkdownPreview(markdown: string, baseDir: string | null, options: MarkdownOptions, active: boolean) {
  const { t } = useTranslation();
  const [preview, setPreview] = useState<{ path: string; version: number } | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const optionsKey = JSON.stringify(options);

  useEffect(() => {
    if (!active) return;
    let cancelled = false;
    const timer = setTimeout(async () => {
      setLoading(true);
      try {
        const path = await markdownPreview({ markdown, baseDir, options });
        if (!cancelled) {
          setPreview({ path, version: Date.now() });
          setError(null);
        }
      } catch (err) {
        if (!cancelled) setError(errorMessage(t, toAppError(err)));
      } finally {
        if (!cancelled) setLoading(false);
      }
    }, DEBOUNCE_MS);
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, [markdown, baseDir, optionsKey, active]); // optionsKey: bandingkan isi pengaturan, bukan identitas objek

  return { preview, loading, error };
}
