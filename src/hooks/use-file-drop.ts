import { useEffect, useRef, useState } from "react";
import { getCurrentWebview } from "@tauri-apps/api/webview";

/**
 * Drag & drop file/folder dari OS ke seluruh jendela.
 * Mengembalikan `true` selama sesuatu sedang di-drag di atas jendela.
 */
export function useFileDrop(onDrop: (paths: string[]) => void): boolean {
  const [dragging, setDragging] = useState(false);
  // Ref supaya listener tidak perlu didaftar ulang setiap render
  const onDropRef = useRef(onDrop);
  onDropRef.current = onDrop;

  useEffect(() => {
    const unlisten = getCurrentWebview().onDragDropEvent(({ payload }) => {
      if (payload.type === "enter" || payload.type === "over") {
        setDragging(true);
      } else {
        setDragging(false);
        if (payload.type === "drop") onDropRef.current(payload.paths);
      }
    });
    return () => {
      unlisten.then((stop) => stop());
    };
  }, []);

  return dragging;
}
