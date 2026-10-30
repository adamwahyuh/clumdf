/** Tanggal PDF "D:YYYYMMDDHHmmSS+07'00'" → Date (bagian yang tidak ada dianggap awal). */
export function parsePdfDate(raw: string | null): Date | null {
  const m = /^(?:D:)?(\d{4})(\d{2})?(\d{2})?(\d{2})?(\d{2})?(\d{2})?(Z|[+-]\d{2}'?\d{2}'?)?/.exec(raw?.trim() ?? "");
  if (!m) return null;
  const [, y, mo = "01", d = "01", h = "00", mi = "00", s = "00", tz] = m;
  // Tanpa zona waktu: anggap waktu lokal, seperti penampil PDF pada umumnya
  const zone = !tz ? "" : tz === "Z" ? "Z" : `${tz.slice(0, 3)}:${tz.replace(/'/g, "").slice(3, 5)}`;
  const date = new Date(`${y}-${mo}-${d}T${h}:${mi}:${s}${zone}`);
  return Number.isNaN(date.getTime()) ? null : date;
}
