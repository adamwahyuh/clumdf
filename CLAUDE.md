# Berkas — Aplikasi Desktop PDF

> Dokumen ini adalah **spesifikasi proyek & panduan kerja** untuk Claude Code.
> Taruh di root proyek dengan nama `CLAUDE.md` supaya otomatis dibaca setiap sesi.
> Nama "Berkas" hanya placeholder dan boleh diganti.

---

## 1. Ringkasan

Aplikasi desktop lintas platform (**macOS, Windows, Linux**) untuk mengolah PDF:
merge, compress, gambar → PDF, dan fitur lanjutan lain. Semua proses berjalan
**100% offline** di komputer user (tidak ada upload ke server).

Target pengguna: orang Indonesia yang sering mengurus berkas digital
(lamaran kerja, CPNS, beasiswa, administrasi kampus) dan butuh alat yang
sederhana, cepat, dan aman untuk dokumen pribadi seperti KTP atau ijazah.

---

## 2. Tech Stack (sudah dikunci)

### Frontend
| Kebutuhan | Pilihan |
|---|---|
| Framework | React + TypeScript + Vite |
| Styling | Tailwind CSS v4 |
| UI kit | **shadcn/ui** (komponen di `src/components/ui`) |
| Ikon | **lucide-react** (tidak boleh campur library ikon lain) |
| Drag & drop | dnd-kit (`@dnd-kit/core`, `@dnd-kit/sortable`) |
| State | Zustand |
| Thumbnail PDF | pdf.js (`pdfjs-dist`) |
| Dua bahasa | i18next + react-i18next (Indonesia default, English) |
| Toast | sonner (via shadcn) |

### Backend
| Kebutuhan | Pilihan | Lisensi |
|---|---|---|
| Desktop shell | **Tauri v2** | MIT/Apache |
| Manipulasi PDF | `lopdf` | MIT |
| Gambar (decode/resize/encode) | `image` | MIT/Apache |
| Paralel (batch, kompresi gambar) | `rayon` | MIT/Apache |
| Error | `thiserror` | MIT/Apache |
| Render PDF → gambar (v4) | `pdfium-render` + binary PDFium | MIT/Apache + BSD |
| Password protect/unlock (v3) | `qpdf` sebagai sidecar | Apache 2.0 |
| OCR (v5) | Tesseract sebagai sidecar (data bahasa `ind`) | Apache 2.0 |

**Aturan lisensi:** jangan pakai library berlisensi **AGPL/GPL** (misalnya
Ghostscript, MuPDF) supaya app tetap bebas dijadikan open-source maupun komersial.

### Plugin Tauri
`dialog`, `fs`, `store`, `opener`. Aktifkan fitur `protocol-asset` untuk
menampilkan thumbnail file lokal via `convertFileSrc`.

---

## 3. Prinsip Kode (WAJIB)

Tujuan utama: **mudah di-maintain**. Developer lain harus bisa menemukan dan
mengubah satu fitur tanpa membaca seluruh proyek.

1. **Jangan menumpuk kode di satu file.**
   - Satu komponen React = satu file.
   - Satu fitur = satu folder (frontend) dan satu modul (Rust).
   - Patokan: file di atas ±200 baris adalah tanda untuk dipecah.
2. **Logika PDF terpisah dari Tauri.** Semua logika ada di crate
   `crates/pdf-core` yang **tidak bergantung pada Tauri**, sehingga bisa dites
   dengan `cargo test`. `src-tauri` hanya berisi command tipis yang memanggil
   `pdf-core`.
3. **UI tidak pernah memproses file.** UI hanya mengirim perintah + opsi ke Rust,
   lalu menampilkan progress dan hasil.
4. **Wrapper `invoke` ber-type.** Semua pemanggilan command lewat
   `src/lib/tauri/commands.ts`. Komponen tidak boleh memanggil `invoke()` langsung.
5. **Tidak ada teks hardcode di komponen.** Semua teks UI lewat i18next
   (`src/locales/id.json`, `src/locales/en.json`).
6. **Error punya kode stabil.** Rust mengirim `{ code, message, path? }`;
   UI memetakan `code` ke pesan ramah yang bisa diterjemahkan.
7. **Komponen generik dipakai ulang.** Contoh: `SortableFileGrid` dipakai untuk
   merge (input PDF) maupun gambar → PDF (input gambar).
8. **Penamaan:** komponen `PascalCase.tsx`, hook `use-kebab-case.ts`,
   util `kebab-case.ts`, modul Rust `snake_case.rs`.
9. **Komentar secukupnya dalam Bahasa Indonesia**, fokus pada *kenapa*, bukan *apa*.
10. **Setiap fitur baru di `pdf-core` wajib punya tes** di `crates/pdf-core/tests/`.

---

## 4. Struktur Folder

```
berkas/
├─ CLAUDE.md
├─ package.json
├─ vite.config.ts
├─ components.json                  # konfigurasi shadcn
├─ index.html
├─ src/                             # ===== FRONTEND =====
│  ├─ main.tsx
│  ├─ App.tsx
│  ├─ index.css                     # Tailwind + design tokens
│  ├─ app/
│  │  └─ tools.ts                   # registry fitur (id, ikon, judul, view) → dipakai sidebar
│  ├─ components/
│  │  ├─ ui/                        # komponen shadcn (button, select, ...)
│  │  ├─ layout/
│  │  │  ├─ AppShell.tsx            # sidebar + konten
│  │  │  ├─ Sidebar.tsx
│  │  │  ├─ SidebarItem.tsx
│  │  │  └─ ToolHeader.tsx
│  │  ├─ files/
│  │  │  ├─ DropZone.tsx            # empty state + area drop
│  │  │  ├─ SortableFileGrid.tsx    # grid generik drag & drop
│  │  │  ├─ FileCard.tsx
│  │  │  └─ FileThumbnail.tsx       # thumbnail gambar / halaman 1 PDF
│  │  └─ common/
│  │     ├─ ActionBar.tsx           # bar bawah: ringkasan + tombol utama
│  │     ├─ SettingsPanel.tsx
│  │     ├─ SettingRow.tsx
│  │     ├─ SegmentedControl.tsx    # gaya segmented control macOS
│  │     └─ ProgressOverlay.tsx
│  ├─ features/
│  │  ├─ merge/
│  │  │  ├─ MergeView.tsx
│  │  │  └─ use-merge.ts
│  │  ├─ compress/
│  │  │  ├─ CompressView.tsx
│  │  │  ├─ CompressSettings.tsx
│  │  │  ├─ CompressResult.tsx      # ukuran sebelum → sesudah
│  │  │  ├─ presets.ts
│  │  │  └─ use-compress.ts
│  │  └─ image-to-pdf/
│  │     ├─ ImageToPdfView.tsx
│  │     ├─ ImageToPdfSettings.tsx
│  │     ├─ options.ts
│  │     └─ use-image-to-pdf.ts
│  ├─ hooks/
│  │  ├─ use-file-drop.ts           # drag & drop file dari OS (event Tauri)
│  │  └─ use-job-progress.ts        # dengarkan event progress dari Rust
│  ├─ stores/
│  │  ├─ create-file-queue.ts       # factory store antrian file (dipakai tiap fitur)
│  │  └─ settings-store.ts          # pengaturan yang diingat (persist)
│  ├─ lib/
│  │  ├─ utils.ts                   # cn()
│  │  ├─ format.ts                  # format ukuran file, dll
│  │  ├─ platform.ts                # deteksi macOS/Windows/Linux
│  │  ├─ pdfjs.ts                   # setup worker + render thumbnail
│  │  └─ tauri/
│  │     ├─ commands.ts             # wrapper invoke ber-type
│  │     ├─ dialogs.ts              # pilih file / simpan
│  │     └─ errors.ts               # map kode error → pesan i18n
│  ├─ locales/
│  │  ├─ id.json
│  │  └─ en.json
│  └─ types/
│     └─ files.ts
│
├─ crates/
│  └─ pdf-core/                     # ===== LOGIKA INTI (tanpa Tauri) =====
│     ├─ Cargo.toml
│     ├─ src/
│     │  ├─ lib.rs
│     │  ├─ error.rs                # PdfError + code()
│     │  ├─ progress.rs             # trait ProgressReporter
│     │  ├─ document.rs             # open() & save() bersama
│     │  ├─ inspect.rs              # jumlah halaman, ukuran
│     │  ├─ merge.rs
│     │  ├─ compress/
│     │  │  ├─ mod.rs
│     │  │  ├─ options.rs
│     │  │  ├─ colorspace.rs
│     │  │  └─ image_stream.rs
│     │  ├─ image_to_pdf/
│     │  │  ├─ mod.rs
│     │  │  ├─ options.rs
│     │  │  ├─ layout.rs            # hitung ukuran halaman & posisi gambar (fungsi murni)
│     │  │  └─ embed.rs             # gambar → image XObject
│     │  └─ imaging/
│     │     ├─ mod.rs
│     │     ├─ load.rs              # buka gambar + rotasi EXIF
│     │     ├─ transform.rs         # resize, flatten transparansi
│     │     └─ encode.rs            # JPEG / lossless
│     └─ tests/
│        ├─ merge.rs
│        ├─ compress.rs
│        └─ image_to_pdf.rs
│
├─ src-tauri/                       # ===== SHELL TAURI (tipis) =====
│  ├─ Cargo.toml                    # depend ke ../crates/pdf-core
│  ├─ tauri.conf.json
│  ├─ capabilities/default.json
│  ├─ icons/
│  └─ src/
│     ├─ main.rs
│     ├─ lib.rs                     # registrasi plugin & command
│     ├─ error.rs                   # PdfError → JSON { code, message, path }
│     ├─ progress.rs                # ProgressReporter → emit event "job-progress"
│     └─ commands/
│        ├─ mod.rs
│        ├─ inspect.rs
│        ├─ merge.rs
│        ├─ compress.rs
│        ├─ image_to_pdf.rs
│        └─ files.rs                # tampilkan di Finder/Explorer
│
└─ .github/workflows/build.yml      # build macOS, Windows, Linux
```

---

## 5. Arsitektur & Alur Data

```
React (UI)
  │  commands.compressPdf({ input, output, options, jobId })
  ▼
src-tauri/commands/compress.rs
  │  tauri::async_runtime::spawn_blocking(...)
  ▼
pdf-core::compress::compress_pdf(input, output, &options, &reporter)
  │  reporter.report(Progress { current, total })
  ▼
src-tauri/progress.rs → app.emit("job-progress", { jobId, current, total })
  ▼
React: use-job-progress.ts → progress bar
```

- Command selalu `async` dan pekerjaan berat dijalankan di `spawn_blocking`
  supaya UI tidak freeze.
- Hasil command berupa report, misalnya
  `{ originalSize, compressedSize, imagesOptimized, alreadyOptimal }`.
- Error dikirim sebagai `{ code: "encrypted", message, path }`.
- Opsi dikirim dari UI dalam `camelCase`; struct Rust memakai
  `#[serde(rename_all = "camelCase")]`.

---

## 6. Panduan Desain (gaya Apple / macOS)

Bayangkan kamu desainer di Apple: **tenang, jelas, tidak ramai.**
Ikuti semangat Apple Human Interface Guidelines.

### Prinsip
- **Satu layar, satu tugas.** Setiap fitur punya layar sendiri dengan satu
  tombol utama yang jelas (misalnya "Gabungkan PDF").
- **Progressive disclosure.** Tampilkan pilihan paling umum; pengaturan
  lanjutan disembunyikan di bagian "Lainnya".
- **Default yang masuk akal.** User harus bisa langsung klik tombol utama
  tanpa mengubah apa pun.
- **Feedback instan.** Hover, tekan, drag, progress, dan hasil selalu terlihat.

### Layout
- **Sidebar kiri** (±220px) berisi daftar fitur dengan ikon lucide,
  dikelompokkan: *Atur* (Merge, Split, ...), *Optimasi* (Compress), *Konversi*
  (Gambar → PDF, ...).
- **Area konten** di kanan: header (judul + deskripsi singkat), grid file,
  panel pengaturan di kanan (±280px), dan **action bar** menempel di bawah.
- macOS: `titleBarStyle: "Overlay"` + `hiddenTitle: true`, sehingga tombol
  traffic light menyatu dengan sidebar. Tambahkan `data-tauri-drag-region`
  di area atas. Windows/Linux tetap memakai title bar bawaan.

### Visual
- **Font:** system font (`-apple-system, BlinkMacSystemFont, "Segoe UI", Inter, sans-serif`).
- **Grid spasi 4/8px.** Padding konten 24px, jarak antar elemen 8/12/16px.
- **Radius:** kartu 12px, tombol & input 8px.
- **Warna:** netral (abu-abu hangat) + satu warna aksen (biru sistem
  `#0A84FF`). Merah hanya untuk aksi hapus/error.
- **Bayangan halus**, bukan border tebal.
- **Dark mode** mengikuti sistem secara otomatis.
- **Animasi:** 150–250ms, easing `ease-out`. Kartu yang di-drag sedikit
  membesar (scale 1.02) dan bayangannya menguat.

### Komponen kunci
- **Empty state / DropZone:** ikon besar lembut, kalimat "Tarik file ke sini
  atau klik untuk memilih", plus info format yang didukung. Seluruh jendela
  menjadi area drop, dengan overlay saat file sedang di-drag.
- **FileCard:** thumbnail, nama file (truncate), info kecil (jumlah halaman /
  ukuran), nomor urut, tombol hapus muncul saat hover.
- **SegmentedControl** (gaya macOS) untuk pilihan 2–4 opsi seperti orientasi
  atau tingkat kompresi. Gunakan Select hanya jika opsinya banyak.
- **Hasil:** setelah selesai, tampilkan kartu hasil (centang hijau, ukuran
  sebelum → sesudah, persentase hemat) dengan tombol **"Tampilkan di Finder"**
  (Windows: Explorer) dan **"Proses lagi"**.

### Bahasa & copy
- Bahasa Indonesia yang ramah dan singkat, tanpa jargon teknis.
  Contoh: "PDF ini dikunci dengan password" (bukan "Encrypted document").
- Tombol memakai kata kerja: "Gabungkan", "Kecilkan", "Buat PDF".

### Aksesibilitas & keyboard
- Semua aksi bisa lewat keyboard, focus ring terlihat.
- Shortcut: `⌘/Ctrl + O` tambah file, `⌘/Ctrl + Enter` proses,
  `Delete` hapus file terpilih, `⌘/Ctrl + 1..9` pindah fitur.
- Kontras warna memenuhi WCAG AA.

---

## 7. Roadmap Fitur

> **Tidak termasuk:** Word/Excel → PDF dan watermark.

### v1 — MVP (kerjakan ini dulu)

**Merge PDF**
- Tambah file lewat drag & drop dari OS atau dialog.
- Thumbnail halaman pertama + jumlah halaman + ukuran.
- Atur urutan dengan drag & drop, hapus file dari daftar.
- File terkunci/rusak ditandai sebelum diproses.
- Simpan lewat dialog "Simpan sebagai".

**Compress PDF**
- Preset: **Ringan** (2400px, JPEG q80), **Seimbang** (1600px, q65, default),
  **Maksimal** (1100px, q45).
- Opsi: ubah ke hitam-putih (grayscale).
- Bisa beberapa file sekaligus (masing-masing jadi file hasil sendiri, akhiran `-compressed`).
- Tampilkan ukuran sebelum → sesudah dan persentase hemat.
- Jika hasil tidak lebih kecil, pertahankan file asli dan beri tahu
  "File ini sudah optimal".

**Gambar → PDF**
- Input: JPG, PNG, WebP, BMP, TIFF, GIF (HEIC belum).
- Atur urutan secara visual (drag & drop thumbnail).
- Pengaturan global:
  - Ukuran halaman: **Fit (sesuai gambar)**, A4, **F4/Folio (215 × 330 mm)**, Letter, Legal
  - Orientasi: Otomatis, Portrait, Landscape (nonaktif saat mode Fit)
  - Margin: Tanpa, Kecil (10 mm), Sedang (20 mm), Besar (30 mm)
  - Kualitas: Asli, Tinggi (3000px q90), Sedang (2000px q75, default), Kecil (1400px q60)
- Override per gambar (klik ikon pengaturan di kartu) — boleh menyusul di v1.1.
- Rotasi EXIF foto HP diterapkan otomatis.
- Gambar transparan diberi latar putih.

**Umum v1**
- Sidebar navigasi, dark mode, bahasa Indonesia + English.
- Ingat pengaturan terakhir (Zustand persist).

### v2 — Kelola halaman & fitur andalan
- Split PDF (per halaman, rentang `1-3, 5, 8-10`, tiap N halaman)
- Hapus / ekstrak halaman
- Rotate halaman (per halaman atau semua)
- Page organizer: grid thumbnail semua halaman, drag & drop untuk susun ulang
- **Target ukuran file** (di bawah 500 KB / 1 MB / 2 MB / custom): turunkan
  kualitas & resolusi bertahap (binary search) sampai target tercapai
- **Preset berkas** (bawaan seperti "Berkas Lamaran" / "Scan KTP", plus buatan user)

### v3 — Dokumen resmi & keamanan
- Tanda tangan: simpan gambar tanda tangan (PNG transparan) sekali, tempel dan
  geser di halaman mana pun
- Password protect / unlock (qpdf sidecar)
- Hapus metadata
- Nomor halaman (posisi & format bisa diatur)
- Header / footer

### v4 — Konversi & produktivitas
- PDF → gambar (JPG/PNG, pilih DPI)
- Mode scan dokumen: hitam-putih bersih & kontras, lalu auto-crop & luruskan
- Batch processing satu folder
- Workflow berantai (mis. Gambar → PDF → Compress dalam satu klik)
- Riwayat file yang baru diproses

### v5 — Lanjutan
- OCR (teks bisa dicari & di-copy, bahasa Indonesia)
- Integrasi klik kanan di Finder/Explorer & drop file ke ikon app

---

## 8. Catatan Teknis Penting (hindari jebakan)

**Merge**
- Sebelum memindahkan halaman ke pohon `Pages` baru, **salin atribut warisan**
  (`Resources`, `MediaBox`, `CropBox`, `Rotate`) dari node induk ke tiap
  halaman. Jika tidak, halaman bisa kosong atau ukurannya salah.
- Susun `Kids` dari urutan `get_pages()` (urutan halaman sebenarnya), bukan
  dari urutan ID objek.
- Geser nomor objek tiap dokumen dengan `renumber_objects_with(next_id)`
  agar tidak bentrok, lalu `prune_objects()` di akhir.

**Membuka PDF**
- Pakai `LoadOptions::with_max_decompressed_size(512 MB)` untuk melindungi
  dari "decompression bomb".
- PDF yang masih terenkripsi setelah dibuka → error `encrypted`
  (dukungan penuh di v3).

**Menyimpan PDF**
- Simpan dengan `save_modern` (object streams) agar file lebih kecil.
- Tulis ke file sementara lalu `rename`, supaya file tujuan tidak pernah
  setengah jadi kalau proses gagal.

**Compress**
- Proses hanya gambar yang aman: `BitsPerComponent = 8`, tanpa `Decode`
  array, bukan `ImageMask`, ruang warna Gray/RGB (termasuk `ICCBased` dengan
  `N = 1` atau `3`). Lewati CMYK, Indexed, Lab, dll.
- Jangan sentuh gambar yang dipakai sebagai `SMask`/`Mask` gambar lain.
- Ganti gambar hanya jika hasilnya lebih kecil (minimal hemat 5%).
- Proses gambar secara paralel dengan `rayon`.
- Setelah itu jalankan `doc.compress()`, `delete_zero_length_streams()`,
  dan `prune_objects()`.

**Gambar → PDF**
- Gunakan `ImageDecoder::orientation()` + `DynamicImage::apply_orientation()`
  untuk rotasi EXIF.
- Kualitas "Asli" + file JPEG tanpa rotasi: masukkan byte JPEG apa adanya
  (`DCTDecode`) tanpa re-encode. Format lain: lossless (`FlateDecode`).
- Konversi satuan: 1 mm = 72 / 25.4 pt. Mode Fit: px × 0.75 (anggap 96 DPI).
- Logika layout (ukuran halaman & posisi gambar) dibuat sebagai **fungsi
  murni** di `layout.rs` supaya mudah dites.

**Frontend**
- Thumbnail file lokal: `convertFileSrc(path)` (butuh `assetProtocol`
  aktif dengan scope yang sesuai).
- pdf.js: impor worker dengan `pdfjs-dist/build/pdf.worker.min.mjs?url`.
- Drag & drop dari OS: `getCurrentWebview().onDragDropEvent()`.
- Revoke object URL saat file dihapus dari daftar.

---

## 9. Langkah Kerja untuk Claude Code

Kerjakan **satu milestone per sesi/PR**, jalankan verifikasi di akhir
setiap milestone sebelum lanjut.

1. **Scaffold**
   - `npm create tauri-app@latest` (React + TypeScript), pasang Tailwind v4,
     inisialisasi shadcn, pasang lucide-react, Zustand, dnd-kit, i18next, pdfjs-dist.
   - Buat crate `crates/pdf-core` dan hubungkan ke `src-tauri` lewat path dependency.
   - Buat struktur folder sesuai bagian 4 (file kosong/placeholder boleh).
   - ✅ `npm run tauri dev` membuka jendela kosong.

2. **Layout & design system**
   - Design tokens di `index.css` (light + dark), AppShell, Sidebar,
     ToolHeader, ActionBar, SegmentedControl, DropZone.
   - Registry fitur di `src/app/tools.ts`.
   - ✅ Bisa pindah antar 3 layar fitur (masih kosong) dengan tampilan rapi
     di light & dark mode.

3. **pdf-core: inspect + merge** (+ tes)
   - ✅ `cargo test -p pdf-core` lulus: hasil merge punya jumlah halaman yang
     benar dan urutannya sesuai input.

4. **Fitur Merge end-to-end**
   - Command Tauri, wrapper ber-type, file queue store, SortableFileGrid,
     FileThumbnail (pdf.js), progress, kartu hasil.
   - ✅ Bisa merge 3 PDF nyata dan membuka hasilnya.

5. **pdf-core: compress** (+ tes) → **fitur Compress end-to-end**
   - ✅ PDF berisi foto besar turun ukurannya; PDF teks saja tidak membesar.

6. **pdf-core: image_to_pdf** (+ tes untuk `layout.rs`) → **fitur Gambar → PDF end-to-end**
   - ✅ 5 gambar campuran (portrait/landscape/PNG transparan/foto HP)
     menjadi PDF dengan urutan & pengaturan yang benar.

7. **Polish v1**
   - Shortcut keyboard, ingat pengaturan, pesan error ramah, terjemahan English.
   - GitHub Actions untuk build 3 OS.
   - ✅ `cargo test`, `npx tsc --noEmit`, `npm run build` lulus tanpa error.

Setelah v1 selesai, lanjutkan roadmap v2 → v5 dengan pola yang sama:
**logika + tes di pdf-core → command tipis → UI per folder fitur.**

---

## 10. Perintah yang Sering Dipakai

```bash
# Prasyarat (macOS)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
xcode-select --install

# Development
npm install
npm run tauri dev

# Tes logika PDF
cargo test -p pdf-core

# Cek tipe & build frontend
npx tsc --noEmit
npm run build

# Build installer untuk OS saat ini
npm run tauri build

# Tambah komponen shadcn
npx shadcn@latest add button select switch progress tooltip sonner
```
