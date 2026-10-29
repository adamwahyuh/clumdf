// userAgent cukup untuk membedakan OS di webview Tauri; tidak perlu plugin os.
const ua = navigator.userAgent;

export const isMac = /Mac OS X|Macintosh/.test(ua);
export const isWindows = /Windows/.test(ua);
