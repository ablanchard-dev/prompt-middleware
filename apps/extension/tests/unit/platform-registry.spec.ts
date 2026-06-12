import { describe, expect, it } from "vitest";

import { getPlatformAdapter } from "../../src/platforms/registry";

describe("platform registry", () => {
  it("detects ChatGPT URLs", () => {
    expect(getPlatformAdapter("https://chatgpt.com/")?.id).toBe("chatgpt");
    expect(getPlatformAdapter("https://chat.openai.com/")?.id).toBe("chatgpt");
  });

  it("returns null for unsupported platforms", () => {
    expect(getPlatformAdapter("https://example.com/")).toBeNull();
  });
});

