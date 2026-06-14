import initWasm, { optimize as wasmOptimize } from "../wasm/prompt_engine_wasm.js";
import type { OptimizeRequest, OptimizeResponse } from "../shared/api-types";

chrome.runtime.onInstalled.addListener(() => {
  chrome.storage.local.set({ enabled: true, localServerPort: 43187 });
});

// The engine compiled to WebAssembly runs here, in the extension's own context,
// so prompts are optimized fully on-device. Initialized lazily on first use.
let wasmReady: Promise<void> | null = null;

function ensureWasm(): Promise<void> {
  if (!wasmReady) {
    wasmReady = initWasm(chrome.runtime.getURL("wasm/prompt_engine_wasm_bg.wasm")).then(
      () => undefined
    );
  }
  return wasmReady;
}

interface OptimizeMessage {
  type: "pmw:optimize";
  request: OptimizeRequest;
}

function isOptimizeMessage(message: unknown): message is OptimizeMessage {
  return (
    typeof message === "object" &&
    message !== null &&
    (message as { type?: unknown }).type === "pmw:optimize"
  );
}

chrome.runtime.onMessage.addListener((message, _sender, sendResponse) => {
  if (!isOptimizeMessage(message)) {
    return false;
  }

  void (async () => {
    try {
      await ensureWasm();
      const response = JSON.parse(
        wasmOptimize(JSON.stringify(message.request))
      ) as OptimizeResponse;
      sendResponse({ ok: true, response });
    } catch (error) {
      // Let the content script fall back to the local HTTP server.
      sendResponse({ ok: false, error: String(error) });
    }
  })();

  // Keep the message channel open for the async response.
  return true;
});
