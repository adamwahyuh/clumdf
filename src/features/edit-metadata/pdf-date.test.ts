import { test } from "node:test";
import assert from "node:assert/strict";
import { parsePdfDate } from "./pdf-date.ts";

test("tanggal PDF dengan zona waktu", () => {
  assert.equal(parsePdfDate("D:20240131120000+07'00'")?.toISOString(), "2024-01-31T05:00:00.000Z");
  assert.equal(parsePdfDate("D:20240131120000Z")?.toISOString(), "2024-01-31T12:00:00.000Z");
});

test("tanggal tidak lengkap atau rusak", () => {
  assert.equal(parsePdfDate("D:2024")?.getFullYear(), 2024);
  assert.equal(parsePdfDate("bukan tanggal"), null);
  assert.equal(parsePdfDate(null), null);
});
