import type { TFunction } from "i18next";
import { fileName } from "@/lib/format";

/** Bentuk error dari Rust: `{ code, message, path? }` */
export interface AppError {
  code: string;
  message: string;
  path?: string;
}

const knownCodes = ["encrypted", "invalid", "invalid_image", "invalid_range", "io", "no_input", "wrong_password", "not_encrypted", "empty_password"];

export function toAppError(err: unknown): AppError {
  if (typeof err === "object" && err !== null && "code" in err) return err as AppError;
  return { code: "unknown", message: String(err) };
}

/** Pesan ramah yang bisa diterjemahkan; detail teknis tetap ada di `message`. */
export function errorMessage(t: TFunction, err: AppError): string {
  const key = knownCodes.includes(err.code) ? err.code : "unknown";
  return t(`errors.${key}`, { name: err.path ? fileName(err.path) : "" });
}
