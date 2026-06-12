export type PlatformId = "chatgpt" | "claude" | "gemini" | "deepseek";

export interface PromptInput {
  element: HTMLElement;
}

export interface PlatformCapabilities {
  supportsRichText: boolean;
}

export interface PlatformAdapter {
  id: PlatformId;
  matchesUrl(url: URL): boolean;
  detectInput(): PromptInput | null;
  readInput(input: PromptInput): string;
  replaceInput(input: PromptInput, value: string): void;
  findAnchorElement(): HTMLElement | null;
  getCapabilities(): PlatformCapabilities;
}

