// Smoke test: prove the Rust engine runs through WebAssembly, callable from JS.
//
// Build the Node bindings first, then run this:
//   sh scripts/build-wasm.sh nodejs pkg
//   node scripts/wasm-smoke.mjs
//
// Exits non-zero if the WASM result does not match the expected detection.

import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import path from "node:path";

const require = createRequire(import.meta.url);
const here = path.dirname(fileURLToPath(import.meta.url));
const { optimize } = require(path.join(here, "..", "pkg", "prompt_engine_wasm.js"));

const request = {
  raw_user_input: "corrige mon code python qui plante",
  target_platform: "chatgpt",
  language: "auto",
  mode: "preview",
  user_preferences: { tone: null, detail_level: "normal" },
};

const out = JSON.parse(optimize(JSON.stringify(request)));
console.log(
  `WASM optimize() -> ${out.detected_language} / ${out.detected_domain} / ${out.detected_intent}`
);

const ok =
  out.detected_language === "fr" &&
  out.detected_domain === "code" &&
  out.detected_intent === "corriger" &&
  out.optimized_prompt.length > 0;

if (!ok) {
  console.error("WASM smoke test FAILED: unexpected result");
  process.exit(1);
}

console.log("WASM smoke test passed: the engine runs in WebAssembly with the expected result.");
