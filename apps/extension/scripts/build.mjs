import { build } from "esbuild";
import { cp, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";

const root = process.cwd();
const dist = path.join(root, "dist");

await rm(dist, { recursive: true, force: true });
await mkdir(dist, { recursive: true });

await build({
  entryPoints: {
    "content/index": "src/content/index.ts",
    "background/service-worker": "src/background/service-worker.ts",
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

await mkdir(path.join(dist, "styles"), { recursive: true });
await mkdir(path.join(dist, "popup"), { recursive: true });
await mkdir(path.join(dist, "options"), { recursive: true });

await cp("styles/content.css", path.join(dist, "styles/content.css"));
await cp("src/popup/popup.html", path.join(dist, "popup/popup.html"));
await cp("src/options/options.html", path.join(dist, "options/options.html"));

const manifest = JSON.parse(await readFile("src/manifest.base.json", "utf8"));
manifest.content_scripts[0].js = ["content/index.js"];
manifest.content_scripts[0].css = ["styles/content.css"];
manifest.background.service_worker = "background/service-worker.js";
manifest.action.default_popup = "popup/popup.html";
manifest.options_page = "options/options.html";

await writeFile(path.join(dist, "manifest.json"), `${JSON.stringify(manifest, null, 2)}\n`);
