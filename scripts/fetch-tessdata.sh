#!/usr/bin/env bash
# Unduh data bahasa OCR (tessdata_fast, lisensi Apache 2.0) ke src-tauri/tessdata/.
# Dibundel bersama app; program tesseract-nya sendiri memakai yang terpasang di sistem.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/src-tauri/tessdata"
# Dikunci ke tag rilis supaya isi file tidak berubah diam-diam
REV="${TESSDATA_REV:-4.1.0}"

mkdir -p "$DEST"
for lang in ind eng; do
  curl -fsSL -o "$DEST/$lang.traineddata" "https://github.com/tesseract-ocr/tessdata_fast/raw/$REV/$lang.traineddata"
done
echo "Data bahasa OCR ($REV) siap di $DEST"
