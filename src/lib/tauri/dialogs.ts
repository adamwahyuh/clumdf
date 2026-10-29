import { open, save } from "@tauri-apps/plugin-dialog";

/** Pilih beberapa file; kosong jika dibatalkan. */
export async function pickFiles(filterName: string, extensions: string[]): Promise<string[]> {
  const result = await open({ multiple: true, directory: false, filters: [{ name: filterName, extensions }] });
  return result ?? [];
}

/** Dialog "Simpan sebagai" untuk PDF; null jika dibatalkan. */
export async function pickSavePath(defaultPath: string): Promise<string | null> {
  return save({ defaultPath, filters: [{ name: "PDF", extensions: ["pdf"] }] });
}

/** Pilih folder tujuan; null jika dibatalkan. */
export async function pickFolder(defaultPath?: string): Promise<string | null> {
  return open({ directory: true, multiple: false, defaultPath });
}
