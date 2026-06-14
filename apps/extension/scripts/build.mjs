import { build } from "esbuild";
import { cp, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { execFileSync } from "node:child_process";
import path from "node:path";

const root = process.cwd(); // apps/extension
const repoRoot = path.resolve(root, "..", "..");
const dist = path.join(root, "dist");
const wasmSrc = path.join(root, "src/wasm");

await rm(dist, { recursive: true, force: true });
await mkdir(dist, { recursive: true });

// 1. Build the engine to WebAssembly and generate the JS glue into src/wasm/.
//    Requires: rustup target add wasm32-unknown-unknown; cargo install wasm-bindgen-cli.
const wasmArtifact = path.join(
  repoRoot,
  "target/wasm32-unknown-unknown/release/prompt_engine_wasm.wasm"
);
execFileSync(
  "cargo",
  ["build", "--release", "--target", "wasm32-unknown-unknown", "-p", "prompt-engine-wasm"],
  { cwd: repoRoot, stdio: "inherit" }
);
execFileSync(
  "wasm-bindgen",
  ["--target", "web", "--no-typescript", "--out-dir", wasmSrc, wasmArtifact],
  { stdio: "inherit" }
);

// 2. Content script, popup, and options are classic scripts (IIFE).
await build({
  entryPoints: {
    "content/index": "src/content/index.ts",
    "popup/popup": "src/popup/popup.ts",
    "options/options": "src/options/options.ts"
  },
  bundle: true,
  format: "iife",
  target: "chrome120",
  outdir: "dist",
  sourcemap: false,
  logLevel: "info"
});

// 3. The service worker is an ES module (it imports the wasm glue).
await build({
  entryPoints: { "background/service-worker": "src/background/service-worker.ts" },
  bundle: true,
  format: "esm",
  target: "chrome120",
  outdir: "dist",
  sourcemap: false,
  logLevel: "info"
});

// 4. Static assets and the wasm binary.
await mkdir(path.join(dist, "styles"), { recursive: true });
await mkdir(path.join(dist, "popup"), { recursive: true });
await mkdir(path.join(dist, "options"), { recursive: true });
await mkdir(path.join(dist, "wasm"), { recursive: true });

await cp("styles/content.css", path.join(dist, "styles/content.css"));
await cp("src/popup/popup.html", path.join(dist, "popup/popup.html"));
await cp("src/options/options.html", path.join(dist, "options/options.html"));
await cp(
  path.join(wasmSrc, "prompt_engine_wasm_bg.wasm"),
  path.join(dist, "wasm/prompt_engine_wasm_bg.wasm")
);

// 5. Manifest.
const manifest = JSON.parse(await readFile("src/manifest.base.json", "utf8"));
manifest.content_scripts[0].js = ["content/index.js"];
manifest.content_scripts[0].css = ["styles/content.css"];
manifest.background.service_worker = "background/service-worker.js";
manifest.action.default_popup = "popup/popup.html";
manifest.options_page = "options/options.html";

await writeFile(path.join(dist, "manifest.json"), `${JSON.stringify(manifest, null, 2)}\n`);
