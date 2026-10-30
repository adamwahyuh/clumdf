import { invoke } from "@tauri-apps/api/core";

// Satu-satunya tempat yang memanggil invoke(); komponen memakai fungsi di bawah.

export interface PdfInfo {
  pageCount: number;
  fileSize: number;
}

export interface MergeReport {
  pageCount: number;
  outputSize: number;
}

export function inspectPdf(path: string): Promise<PdfInfo> {
  return invoke("inspect_pdf", { path });
}

export function mergePdfs(args: { inputs: string[]; output: string; jobId: string }): Promise<MergeReport> {
  return invoke("merge_pdfs", args);
}

export interface CompressOptions {
  maxDimension: number;
  jpegQuality: number;
  grayscale: boolean;
  /** Target ukuran (byte); jika diisi, resolusi & kualitas dipilih otomatis */
  targetSize: number | null;
}

export interface CompressReport {
  originalSize: number;
  compressedSize: number;
  imagesOptimized: number;
  alreadyOptimal: boolean;
  /** null jika tanpa target */
  targetMet: boolean | null;
}

export function compressPdf(args: {
  input: string;
  output: string;
  options: CompressOptions;
  jobId: string;
}): Promise<CompressReport> {
  return invoke("compress_pdf", args);
}

export type PageSize = "fit" | "a4" | "f4" | "letter" | "legal";
export type PageOrientation = "auto" | "portrait" | "landscape";
export type Margin = "none" | "small" | "medium" | "large";
export type ImageQuality = "original" | "high" | "medium" | "low";

export interface ImageToPdfOptions {
  pageSize: PageSize;
  orientation: PageOrientation;
  margin: Margin;
  quality: ImageQuality;
  /** Mode scan dokumen: potong ke kertas, luruskan, hitam-putih bersih */
  scan: boolean;
  /** Workflow berantai: langsung kecilkan PDF hasil */
  compress?: CompressOptions | null;
}

export interface ImageToPdfReport {
  pageCount: number;
  outputSize: number;
  /** null jika tidak dikecilkan dengan target */
  targetMet: boolean | null;
}

export function imagesToPdf(args: {
  inputs: string[];
  output: string;
  options: ImageToPdfOptions;
  jobId: string;
}): Promise<ImageToPdfReport> {
  return invoke("images_to_pdf", args);
}

export type SplitMode = { mode: "eachPage" } | { mode: "ranges"; ranges: string } | { mode: "everyN"; n: number };

export interface SplitReport {
  files: string[];
}

export function splitPdf(args: { input: string; outputDir: string; mode: SplitMode; jobId: string }): Promise<SplitReport> {
  return invoke("split_pdf", args);
}

/** Satu halaman di hasil: nomor halaman asli (mulai 1) + rotasi tambahan (kelipatan 90). */
export interface PageEdit {
  page: number;
  rotate: number;
}

export interface PagesReport {
  pageCount: number;
  outputSize: number;
}

export function editPages(args: { input: string; output: string; pages: PageEdit[] }): Promise<PagesReport> {
  return invoke("edit_pages", args);
}

export interface StripReport {
  outputSize: number;
  /** 0 = file memang sudah bersih */
  removed: number;
}

export function stripMetadata(args: { input: string; output: string }): Promise<StripReport> {
  return invoke("strip_metadata", args);
}

export interface PasswordReport {
  outputSize: number;
}

export function protectPdf(args: { input: string; output: string; password: string }): Promise<PasswordReport> {
  return invoke("protect_pdf", args);
}

export function unlockPdf(args: { input: string; output: string; password: string }): Promise<PasswordReport> {
  return invoke("unlock_pdf", args);
}

export type StampPosition = "topLeft" | "topCenter" | "topRight" | "bottomLeft" | "bottomCenter" | "bottomRight";

export interface StampOptions {
  /** Teks dengan penanda {page} dan {total} */
  stamps: { text: string; position: StampPosition }[];
  fontSize: number;
  marginMm: number;
  firstNumber: number;
  skipFirst: boolean;
}

export interface StampReport {
  pageCount: number;
  outputSize: number;
}

export function stampText(args: { input: string; output: string; options: StampOptions }): Promise<StampReport> {
  return invoke("stamp_text", args);
}

export interface SignatureInfo {
  path: string;
  width: number;
  height: number;
}

/** Posisi sebagai pecahan (0–1) halaman yang terlihat, asal kiri-atas. */
export interface Placement {
  page: number;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface SignReport {
  signedPages: number;
  outputSize: number;
}

export function importSignature(source: string): Promise<SignatureInfo> {
  return invoke("import_signature", { source });
}

export function placeSignature(args: {
  input: string;
  output: string;
  signature: string;
  placements: Placement[];
}): Promise<SignReport> {
  return invoke("place_signature", args);
}

export interface ExpandedPaths {
  accepted: string[];
  rejected: number;
}

/** Buka folder (tidak rekursif, urutan alami) dan saring berdasarkan ekstensi. */
export function expandPaths(paths: string[], extensions: readonly string[]): Promise<ExpandedPaths> {
  return invoke("expand_paths", { paths, extensions });
}

export interface RenderOptions {
  format: "jpg" | "png";
  /** 72–600 */
  dpi: number;
}

export interface RenderReport {
  files: string[];
}

export function pdfToImages(args: {
  input: string;
  outputDir: string;
  options: RenderOptions;
  jobId: string;
}): Promise<RenderReport> {
  return invoke("pdf_to_images", args);
}

/** Ambil (dan kosongkan) file yang dibuka lewat OS: ikon Dock, "Buka dengan", argumen. */
export function takeOpenedFiles(): Promise<string[]> {
  return invoke("take_opened_files");
}

export interface OcrReport {
  pagesRecognized: number;
  /** Halaman yang dilewati karena sudah punya teks */
  pagesSkipped: number;
  words: number;
  outputSize: number;
}

/** Lokasi tesseract di sistem, atau null bila belum terpasang. */
export function ocrStatus(): Promise<string | null> {
  return invoke("ocr_status");
}

export function ocrPdf(args: { input: string; output: string; jobId: string }): Promise<OcrReport> {
  return invoke("ocr_pdf", args);
}

export interface MarkdownOptions {
  pageSize: Exclude<PageSize, "fit">;
  font: "sans" | "serif";
  fontSize: number;
  marginMm: number;
  pageNumbers: boolean;
}

export interface MarkdownReport {
  pageCount: number;
  outputSize: number;
}

export function markdownToPdf(args: {
  markdown: string;
  baseDir: string | null;
  output: string;
  options: MarkdownOptions;
}): Promise<MarkdownReport> {
  return invoke("markdown_to_pdf", args);
}

/** Buat PDF pratinjau di folder cache app; mengembalikan path-nya. */
export function markdownPreview(args: { markdown: string; baseDir: string | null; options: MarkdownOptions }): Promise<string> {
  return invoke("markdown_preview", args);
}

export function readTextFile(path: string): Promise<string> {
  return invoke("read_text_file", { path });
}

export function writeTextFile(path: string, content: string): Promise<void> {
  return invoke("write_text_file", { path, content });
}

export interface PdfMetadata {
  title: string | null;
  author: string | null;
  subject: string | null;
  keywords: string | null;
  creator: string | null;
  producer: string | null;
  /** Hanya dibaca; format PDF mentah, mis. "D:20240131120000+07'00'" */
  creationDate: string | null;
  modDate: string | null;
}

export function readPdfMetadata(path: string): Promise<PdfMetadata> {
  return invoke("read_pdf_metadata", { path });
}

export function writePdfMetadata(args: { input: string; output: string; metadata: PdfMetadata }): Promise<{ outputSize: number }> {
  return invoke("write_pdf_metadata", args);
}
