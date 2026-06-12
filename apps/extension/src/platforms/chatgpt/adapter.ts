import { chatgptSelectors } from "./selectors";
import type { PlatformAdapter, PromptInput } from "../types";

export const chatgptAdapter: PlatformAdapter = {
  id: "chatgpt",

  matchesUrl(url: URL): boolean {
    return url.hostname === "chatgpt.com" || url.hostname === "chat.openai.com";
  },

  detectInput(): PromptInput | null {
    const candidates = chatgptSelectors.promptInputs.flatMap((selector) =>
      Array.from(document.querySelectorAll<HTMLElement>(selector))
    );

    const element = candidates.find((candidate) => {
      const rect = candidate.getBoundingClientRect();
      const isVisible = rect.width > 100 && rect.height > 20;
      const isEditable =
        candidate instanceof HTMLTextAreaElement ||
        candidate.getAttribute("contenteditable") === "true" ||
        candidate.getAttribute("role") === "textbox";

      return isVisible && isEditable;
    });

    return element ? { element } : null;
  },

  readInput(input: PromptInput): string {
    if (input.element instanceof HTMLTextAreaElement) {
      return input.element.value;
    }

    return input.element.textContent ?? "";
  },

  replaceInput(input: PromptInput, value: string): void {
    if (input.element instanceof HTMLTextAreaElement) {
      input.element.value = value;
      input.element.dispatchEvent(new Event("input", { bubbles: true }));
      return;
    }

    input.element.textContent = value;
    input.element.dispatchEvent(new InputEvent("input", { bubbles: true, inputType: "insertText", data: value }));
  },

  findAnchorElement(): HTMLElement | null {
    return this.detectInput()?.element ?? null;
  },

  getCapabilities() {
    return { supportsRichText: true };
  }
};

