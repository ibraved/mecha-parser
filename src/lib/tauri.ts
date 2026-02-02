export function isTauri(): boolean {
  // Tauri v2 injects __TAURI__ into window in production/dev via tauri.
  return typeof window !== "undefined" && "__TAURI__" in window;
}

