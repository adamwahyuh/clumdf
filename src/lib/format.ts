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

/** "5 menit yang lalu" / "kemarin", mengikuti bahasa */
export function formatRelativeTime(at: number, locale: string): string {
  const seconds = Math.round((at - Date.now()) / 1000);
  const units: [Intl.RelativeTimeFormatUnit, number][] = [
    ["day", 86400],
    ["hour", 3600],
    ["minute", 60],
  ];
  const rtf = new Intl.RelativeTimeFormat(locale, { numeric: "auto" });
  for (const [unit, size] of units) {
    if (Math.abs(seconds) >= size) return rtf.format(Math.round(seconds / size), unit);
  }
  return rtf.format(0, "second");
}
