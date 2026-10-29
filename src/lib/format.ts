const units = ["B", "KB", "MB", "GB"];

/** 1536 → "1,5 KB" (pemisah desimal mengikuti bahasa) */
export function formatBytes(bytes: number, locale: string): string {
  let value = bytes;
  let unit = 0;
  while (value >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit++;
  }
  const digits = unit === 0 || value >= 100 ? 0 : 1;
  return `${value.toLocaleString(locale, { maximumFractionDigits: digits })} ${units[unit]}`;
}

/** "/a/b/Surat.pdf" atau "C:\\a\\Surat.pdf" → "Surat.pdf" */
export function fileName(path: string): string {
  return path.split(/[\\/]/).pop() ?? path;
}

/** "/a/Surat.pdf" + "-compressed" → "/a/Surat-compressed.pdf" */
export function withSuffix(path: string, suffix: string): string {
  return path.replace(/(\.[^.\\/]+)?$/, (ext) => `${suffix}${ext}`);
}
