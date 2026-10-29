import { useEffect, useRef } from "react";
import { isMac } from "@/lib/platform";

/**
 * Shortcut ⌘ (macOS) / Ctrl (Windows, Linux) + salah satu `keys`.
 * `keys` dibandingkan dengan `KeyboardEvent.key`, mis. "o", "Enter", "1".
 */
export function useHotkey(keys: readonly string[], handler: (key: string) => void, enabled = true) {
  const handlerRef = useRef(handler);
  handlerRef.current = handler;
  const keyList = keys.join("|");

  useEffect(() => {
    if (!enabled) return;
    const wanted = keyList.toLowerCase().split("|");
    function onKeyDown(e: KeyboardEvent) {
      const mod = isMac ? e.metaKey : e.ctrlKey;
      if (!mod || e.altKey || e.shiftKey || !wanted.includes(e.key.toLowerCase())) return;
      e.preventDefault();
      handlerRef.current(e.key);
    }
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [keyList, enabled]);
}
