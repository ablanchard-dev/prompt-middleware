import { defineConfig } from "vitest/config";

// Unit tests run under Vitest; the Playwright end-to-end suite in tests/e2e
// is run separately via `npm run test:e2e` and must not be collected here.
export default defineConfig({
  test: {
    include: ["tests/unit/**/*.spec.ts"]
  }
});
