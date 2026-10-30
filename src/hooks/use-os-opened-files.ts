import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { takeOpenedFiles } from "@/lib/tauri/commands";

/**
 * File yang dibuka lewat OS (drop ke ikon app, "Buka dengan").
 * Diambil saat UI siap dan setiap kali Rust memberi tahu ada file baru.
 */
export function useOsOpenedFiles() {
  const [pending, setPending] = useState<string[]>([]);

  useEffect(() => {
    const drain = async () => {
      const paths = await takeOpenedFiles();
      if (paths.length) setPending((prev) => [...prev, ...paths]);
    };
    void drain();
    const unlisten = listen("files-opened", () => void drain());
    return () => {
      unlisten.then((stop) => stop());
    };
  }, []);

  return { pending, clear: () => setPending([]) };
}
