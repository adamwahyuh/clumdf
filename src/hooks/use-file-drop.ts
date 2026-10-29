import { useEffect, useRef, useState } from "react";
import { getCurrentWebview } from "@tauri-apps/api/webview";

/**
 * Drag & drop file dari OS ke seluruh jendela.
 * Mengembalikan `true` selama file sedang di-drag di atas jendela.
 */
export function useFileDrop(
  extensions: readonly string[],
  onDrop: (paths: string[], rejectedCount: number) => void,
): boolean {
  const [dragging, setDragging] = useState(false);
  // Ref supaya listener tidak perlu didaftar ulang setiap render
  const onDropRef = useRef(onDrop);
  onDropRef.current = onDrop;

  useEffect(() => {
    const unlisten = getCurrentWebview().onDragDropEvent(({ payload }) => {
      if (payload.type === "enter" || payload.type === "over") {
        setDragging(true);
      } else if (payload.type === "leave") {
        setDragging(false);
      } else {
        setDragging(false);
        const accepted = payload.paths.filter((p) =>
          extensions.some((ext) => p.toLowerCase().endsWith(`.${ext}`)),
        );
        onDropRef.current(accepted, payload.paths.length - accepted.length);
      }
    });
    return () => {
      unlisten.then((stop) => stop());
    };
  }, [extensions]);

  return dragging;
}
