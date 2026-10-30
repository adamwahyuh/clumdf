import { createContext, useContext } from "react";

/** Prefix i18n fitur yang sedang tampil; dipakai mis. untuk mencatat riwayat. */
export const ActiveToolContext = createContext<string>("");

export const useActiveTool = () => useContext(ActiveToolContext);
