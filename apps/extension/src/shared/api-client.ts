import type { OptimizeRequest, OptimizeResponse } from "./api-types";

export async function optimizePrompt(
  request: OptimizeRequest,
  port = 43187
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

