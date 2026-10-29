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
}

export interface CompressReport {
  originalSize: number;
  compressedSize: number;
  imagesOptimized: number;
  alreadyOptimal: boolean;
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
}

export interface ImageToPdfReport {
  pageCount: number;
  outputSize: number;
}

export function imagesToPdf(args: {
  inputs: string[];
  output: string;
  options: ImageToPdfOptions;
  jobId: string;
}): Promise<ImageToPdfReport> {
  return invoke("images_to_pdf", args);
}
