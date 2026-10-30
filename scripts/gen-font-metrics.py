#!/usr/bin/env python3
"""Hasilkan crates/pdf-core/src/fonts/metrics.rs dari metrik font standar PDF di pdf.js.

Lebar per karakter WinAnsiEncoding (0-255) dalam 1/1000 em, untuk font base-14
(tidak perlu ditanam di PDF). Jalankan ulang hanya bila pdfjs-dist diperbarui:
    python3 scripts/gen-font-metrics.py
"""
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WORKER = ROOT / "node_modules/pdfjs-dist/build/pdf.worker.mjs"
OUT = ROOT / "crates/pdf-core/src/fonts/metrics.rs"
FONTS = {
    "HELVETICA": "Helvetica",
    "HELVETICA_BOLD": "Helvetica-Bold",
    "HELVETICA_OBLIQUE": "Helvetica-Oblique",
    "HELVETICA_BOLD_OBLIQUE": "Helvetica-BoldOblique",
    "TIMES_ROMAN": "Times-Roman",
    "TIMES_BOLD": "Times-Bold",
    "TIMES_ITALIC": "Times-Italic",
    "TIMES_BOLD_ITALIC": "Times-BoldItalic",
}

source = WORKER.read_text()
encoding = json.loads(re.search(r"const WinAnsiEncoding = (\[.*?\]);", source).group(1))
assert len(encoding) == 256


def table(name: str) -> dict:
    key = re.escape(f'["{name}"]') if "-" in name else re.escape(f".{name}")
    start = re.search(rf"  t{key} = getLookupTableFactory\(function \(t\) {{", source).end()
    body = source[start : source.index("});", start)]
    pairs = re.findall(r't(?:\.(\w+)|\["(\w+)"\]) = (\d+);', body)
    return {(dotted or quoted): int(width) for dotted, quoted, width in pairs}


lines = [
    "//! Dihasilkan oleh scripts/gen-font-metrics.py dari metrik pdf.js — jangan diedit manual.",
    "//! Lebar karakter WinAnsiEncoding (indeks = byte) dalam 1/1000 em.",
    "",
]
for const, name in FONTS.items():
    widths = table(name)
    row = [widths.get(glyph, 0) if glyph else 0 for glyph in encoding]
    missing = [g for g in encoding[32:] if g and g not in widths]
    assert not missing, f"{name}: glyph tanpa lebar {missing}"
    lines.append(f"pub const {const}: [u16; 256] = {row};")
OUT.write_text("\n".join(lines) + "\n")
print(f"{OUT.relative_to(ROOT)}: {len(FONTS)} font")
