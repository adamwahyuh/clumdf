# Logo Clumdf

- `clumdf.svg` — sumber logo (gaya keluarga Clumsee, dokumen PDF di tengah).
- `clumdf-1024.png` — hasil render 1024 px transparan, sumber semua ikon app.

Membuat ulang setelah mengubah SVG (macOS, butuh Google Chrome):

```bash
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless=new --hide-scrollbars \
  --user-data-dir="$(mktemp -d)" --default-background-color=00000000 --window-size=1024,1024 \
  --screenshot="$PWD/assets/logo/clumdf-1024.png" "file://$PWD/assets/logo/clumdf.svg"
npx tauri icon assets/logo/clumdf-1024.png
rm -rf src-tauri/icons/android src-tauri/icons/ios   # app ini hanya desktop
```
