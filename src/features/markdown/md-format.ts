// Operasi format Markdown sebagai fungsi murni: teks + seleksi → satu perubahan.
// Editor menerapkannya lewat execCommand supaya undo/redo bawaan tetap jalan.

export interface TextState {
  value: string;
  start: number;
  end: number;
}

/** Ganti `value[from..to]` dengan `insert`, lalu pilih `selStart..selEnd` (posisi di teks hasil). */
export interface Edit {
  from: number;
  to: number;
  insert: string;
  selStart: number;
  selEnd: number;
}

export type LineKind = "bullet" | "numbered" | "quote";

const LINE_PREFIX = /^(- |\* |\d+\. |> ?)/;
const PREFIXES: Record<LineKind, RegExp> = { bullet: /^[-*] /, numbered: /^\d+\. /, quote: /^> ?/ };

/** Tebal/miring/coret/kode: bungkus seleksi, atau lepas bila sudah dibungkus. */
export function toggleWrap(s: TextState, marker: string, placeholder: string): Edit {
  // Spasi di tepi tidak ikut dibungkus: "**kata **" bukan Markdown yang sah
  let { start, end } = s;
  while (start < end && /\s/.test(s.value[start])) start++;
  while (end > start && /\s/.test(s.value[end - 1])) end--;
  const selected = s.value.slice(start, end);
  const m = marker.length;

  if (selected && s.value.slice(start - m, start) === marker && s.value.slice(end, end + m) === marker) {
    return { from: start - m, to: end + m, insert: selected, selStart: start - m, selEnd: end - m };
  }
  if (selected.length >= 2 * m && selected.startsWith(marker) && selected.endsWith(marker)) {
    const inner = selected.slice(m, -m);
    return { from: start, to: end, insert: inner, selStart: start, selEnd: start + inner.length };
  }
  const text = selected || placeholder;
  return { from: start, to: end, insert: marker + text + marker, selStart: start + m, selEnd: start + m + text.length };
}

/** Rentang baris penuh yang tersentuh seleksi. */
function lineRange(s: TextState): { from: number; to: number } {
  const from = s.value.lastIndexOf("\n", s.start - 1) + 1;
  // Seleksi yang berakhir tepat setelah "\n" tidak menyentuh baris berikutnya
  const endAt = s.end > s.start && s.value[s.end - 1] === "\n" ? s.end - 1 : s.end;
  const next = s.value.indexOf("\n", endAt);
  return { from, to: next === -1 ? s.value.length : next };
}

function replaceLines(s: TextState, map: (lines: string[]) => string[]): Edit {
  const { from, to } = lineRange(s);
  const insert = map(s.value.slice(from, to).split("\n")).join("\n");
  return { from, to, insert, selStart: from, selEnd: from + insert.length };
}

/** Daftar bullet/bernomor/kutipan untuk baris terpilih; dilepas bila semua sudah berformat itu. */
export function toggleLines(s: TextState, kind: LineKind): Edit {
  return replaceLines(s, (lines) => {
    const filled = lines.filter((l) => l.trim());
    const remove = filled.length > 0 && filled.every((l) => PREFIXES[kind].test(l));
    let n = 0;
    return lines.map((line) => {
      if (remove) return line.replace(PREFIXES[kind], "");
      if (!line.trim() && filled.length) return line;
      const body = line.replace(LINE_PREFIX, "");
      n++;
      return kind === "bullet" ? `- ${body}` : kind === "numbered" ? `${n}. ${body}` : `> ${body}`;
    });
  });
}

/** Gaya paragraf: 0 = teks biasa, 1–3 = judul. */
export function setHeading(s: TextState, level: number): Edit {
  return replaceLines(s, (lines) =>
    lines.map((line) => {
      const body = line.replace(/^#{1,6}\s+/, "");
      return level > 0 ? `${"#".repeat(level)} ${body}` : body;
    }),
  );
}

/** Level judul di baris kursor (0 = teks biasa). */
export function currentHeading(s: TextState): number {
  const from = s.value.lastIndexOf("\n", s.start - 1) + 1;
  return /^(#{1,6})\s/.exec(s.value.slice(from))?.[1].length ?? 0;
}

/** Enter di dalam daftar: lanjutkan daftar, atau akhiri bila butirnya kosong. `null` = Enter biasa. */
export function continueList(s: TextState): Edit | null {
  if (s.start !== s.end) return null;
  const from = s.value.lastIndexOf("\n", s.start - 1) + 1;
  const line = s.value.slice(from, s.start);
  const match = /^(\s*)(- |\* |(\d+)\. |> )(.*)$/.exec(line);
  if (!match) return null;
  const [, indent, prefix, number, body] = match;
  if (!body.trim()) {
    // Butir kosong: hapus penanda, keluar dari daftar
    return { from, to: s.start, insert: "", selStart: from, selEnd: from };
  }
  const next = `\n${indent}${number ? `${Number(number) + 1}. ` : prefix}`;
  return { from: s.start, to: s.start, insert: next, selStart: s.start + next.length, selEnd: s.start + next.length };
}

/** Sisipkan blok (tabel, garis, gambar, …) di baris sendiri, dipisah baris kosong. */
export function insertBlock(s: TextState, block: string): Edit {
  const before = s.value.slice(0, s.start);
  const after = s.value.slice(s.end);
  const pre = !before || before.endsWith("\n\n") ? "" : before.endsWith("\n") ? "\n" : "\n\n";
  const post = !after || after.startsWith("\n\n") ? "" : after.startsWith("\n") ? "\n" : "\n\n";
  const cursor = s.start + pre.length + block.length;
  return { from: s.start, to: s.end, insert: pre + block + post, selStart: cursor, selEnd: cursor };
}

/** Link dari seleksi (atau teks contoh); alamatnya langsung terpilih untuk diganti. */
export function insertLink(s: TextState, placeholder: string): Edit {
  const text = s.value.slice(s.start, s.end) || placeholder;
  const url = "https://";
  const insert = `[${text}](${url})`;
  const urlStart = s.start + text.length + 3;
  return { from: s.start, to: s.end, insert, selStart: urlStart, selEnd: urlStart + url.length };
}

/** Terapkan perubahan pada string (untuk tes & cadangan bila execCommand tidak tersedia). */
export function applyEdit(value: string, edit: Edit): string {
  return value.slice(0, edit.from) + edit.insert + value.slice(edit.to);
}
