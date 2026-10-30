// Jalankan: npm test (node --test, tanpa dependency tambahan)
import { test } from "node:test";
import assert from "node:assert/strict";
import {
  applyEdit,
  continueList,
  currentHeading,
  insertBlock,
  insertLink,
  setHeading,
  toggleLines,
  toggleWrap,
  type Edit,
  type TextState,
} from "./md-format.ts";

/** Teks dengan seleksi ditandai [ ]; mis. "a [b] c" = "b" terpilih. */
function state(marked: string): TextState {
  const start = marked.indexOf("[");
  const end = marked.indexOf("]") - 1;
  return { value: marked.replace("[", "").replace("]", ""), start, end };
}

/** Hasil perubahan dengan seleksi baru ditandai [ ]. */
function result(s: TextState, edit: Edit): string {
  const v = applyEdit(s.value, edit);
  return v.slice(0, edit.selStart) + "[" + v.slice(edit.selStart, edit.selEnd) + "]" + v.slice(edit.selEnd);
}

test("bold membungkus, melepas, dan tidak ikut spasi", () => {
  assert.equal(result(state("halo [dunia]"), toggleWrap(state("halo [dunia]"), "**", "teks")), "halo **[dunia]**");
  const bold = state("halo **[dunia]**");
  assert.equal(result(bold, toggleWrap(bold, "**", "teks")), "halo [dunia]");
  const spaced = state("[kata ]lain");
  assert.equal(result(spaced, toggleWrap(spaced, "**", "teks")), "**[kata]** lain");
  const empty = state("a [] b");
  assert.equal(result(empty, toggleWrap(empty, "_", "miring")), "a _[miring]_ b");
});

test("daftar bernomor di beberapa baris lalu dilepas", () => {
  const s = state("[satu\ndua]");
  const numbered = toggleLines(s, "numbered");
  assert.equal(applyEdit(s.value, numbered), "1. satu\n2. dua");
  const again = { value: "1. satu\n2. dua", start: 0, end: 13 };
  assert.equal(applyEdit(again.value, toggleLines(again, "numbered")), "satu\ndua");
  // Ganti jenis daftar: bullet → bernomor
  const bullets = { value: "- a\n- b", start: 0, end: 7 };
  assert.equal(applyEdit(bullets.value, toggleLines(bullets, "numbered")), "1. a\n2. b");
  // Baris kosong: penanda langsung disisipkan
  const blank = state("teks\n[]");
  assert.equal(applyEdit(blank.value, toggleLines(blank, "bullet")), "teks\n- ");
});

test("gaya judul mengganti, bukan menumpuk", () => {
  const s = state("## Ju[d]ul");
  assert.equal(applyEdit(s.value, setHeading(s, 1)), "# Judul");
  assert.equal(applyEdit(s.value, setHeading(s, 0)), "Judul");
  assert.equal(currentHeading(s), 2);
});

test("Enter melanjutkan daftar dan mengakhirinya di butir kosong", () => {
  const item = state("1. satu[]");
  assert.equal(result(item, continueList(item)!), "1. satu\n2. []");
  const nested = state("- a\n  - b[]");
  assert.equal(result(nested, continueList(nested)!), "- a\n  - b\n  - []");
  const blank = state("- a\n- []");
  assert.equal(result(blank, continueList(blank)!), "- a\n[]");
  assert.equal(continueList(state("teks biasa[]")), null);
});

test("blok disisipkan di baris sendiri", () => {
  const s = state("Halo[] dunia");
  assert.equal(result(s, insertBlock(s, "---")), "Halo\n\n---[]\n\n dunia");
  const end = state("Halo\n[]");
  assert.equal(result(end, insertBlock(end, "---")), "Halo\n\n---[]");
});

test("link memilih alamatnya", () => {
  const s = state("lihat [situs]");
  assert.equal(result(s, insertLink(s, "teks")), "lihat [situs]([https://])");
});
