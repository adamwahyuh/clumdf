#!/usr/bin/env bash
# Unduh binary PDFium (github.com/bblanchon/pdfium-binaries, lisensi BSD/Apache)
# ke src-tauri/pdfium/ untuk OS ini. Wajib sebelum `npm run tauri dev/build`
# dan sebelum tes render pdf-core (PDFIUM_DIR=src-tauri/pdfium).
set -euo pipefail

# Dikunci ke versi API yang dipakai pdfium-render (fitur pdfium_7881)
VERSION="${PDFIUM_VERSION:-7881}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/src-tauri/pdfium"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

fetch() {
  local target="$1"
  mkdir -p "$TMP/$target"
  curl -fsSL "https://github.com/bblanchon/pdfium-binaries/releases/download/chromium/$VERSION/pdfium-$target.tgz" \
    | tar -xz -C "$TMP/$target"
}

mkdir -p "$DEST"
case "$(uname -s)-$(uname -m)" in
  Darwin-*)
    # Satu dylib universal supaya build macOS universal (Intel + Apple Silicon) jalan
    fetch mac-arm64
    fetch mac-x64
    lipo -create "$TMP/mac-arm64/lib/libpdfium.dylib" "$TMP/mac-x64/lib/libpdfium.dylib" -output "$DEST/libpdfium.dylib"
    cp "$TMP/mac-arm64/LICENSE" "$DEST/LICENSE"
    ;;
  Linux-x86_64)
    fetch linux-x64
    cp "$TMP/linux-x64/lib/libpdfium.so" "$TMP/linux-x64/LICENSE" "$DEST/"
    ;;
  Linux-aarch64)
    fetch linux-arm64
    cp "$TMP/linux-arm64/lib/libpdfium.so" "$TMP/linux-arm64/LICENSE" "$DEST/"
    ;;
  MINGW*|MSYS*|CYGWIN*)
    fetch win-x64
    cp "$TMP/win-x64/bin/pdfium.dll" "$TMP/win-x64/LICENSE" "$DEST/"
    ;;
  *)
    echo "OS tidak dikenal: $(uname -s)-$(uname -m)" >&2
    exit 1
    ;;
esac
echo "PDFium $VERSION siap di $DEST"
