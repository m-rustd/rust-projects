// const invoke = window.__TAURI__.invoke;
export async function invokeSetWindowDecorations(decorations) {
  if (window.__TAURI__) {
    const invoke = window.__TAURI__.invoke;
    return await invoke("set_window_decorations", { decorations });
  }
}