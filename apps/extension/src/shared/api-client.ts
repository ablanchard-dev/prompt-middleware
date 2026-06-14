import type { OptimizeRequest, OptimizeResponse } from "./api-types";

export async function optimizePrompt(
  request: OptimizeRequest,
  port = 43187
): Promise<OptimizeResponse> {
  const fromWasm = await optimizeViaWasm(request);
  if (fromWasm) {
    return fromWasm;
  }
  return optimizeViaServer(request, port);
}

// Run the engine compiled to WebAssembly in the service worker. Returns null on
// any failure so the caller transparently falls back to the local server.
async function optimizeViaWasm(request: OptimizeRequest): Promise<OptimizeResponse | null> {
  if (typeof chrome === "undefined" || !chrome.runtime?.sendMessage) {
    return null;
  }
  try {
    const reply = (await chrome.runtime.sendMessage({ type: "pmw:optimize", request })) as
      | { ok: true; response: OptimizeResponse }
      | { ok: false; error: string }
      | undefined;
    return reply && reply.ok ? reply.response : null;
  } catch {
    return null;
  }
}

async function optimizeViaServer(
  request: OptimizeRequest,
  port: number
): Promise<OptimizeResponse> {
  const response = await fetch(`http://127.0.0.1:${port}/optimize`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request)
  });

  if (!response.ok) {
    throw new Error(`Local optimizer failed with status ${response.status}`);
  }

  return response.json() as Promise<OptimizeResponse>;
}
