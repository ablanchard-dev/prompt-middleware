import { chatgptAdapter } from "./chatgpt/adapter";
import type { PlatformAdapter } from "./types";

export const platformRegistry: PlatformAdapter[] = [chatgptAdapter];

export function getPlatformAdapter(url = window.location.href): PlatformAdapter | null {
  const parsedUrl = new URL(url);
  return platformRegistry.find((adapter) => adapter.matchesUrl(parsedUrl)) ?? null;
}

