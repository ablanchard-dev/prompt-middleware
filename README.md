# Prompt Middleware

[![CI](https://github.com/ablanchard-dev/prompt-middleware/actions/workflows/ci.yml/badge.svg)](https://github.com/ablanchard-dev/prompt-middleware/actions/workflows/ci.yml)

Local-first browser middleware that optimizes user prompts before they are sent to existing LLM interfaces.

## Architecture

- `apps/extension`: browser UX only.
- `apps/local-server`: local HTTP transport and configuration.
- `apps/cli`: standalone `prompt-optimize` command-line front-end for the engine.
- `crates/prompt-engine`: pure, data-driven prompt optimization engine (templates overridable via `EngineConfig`). The classifier recognizes several domains; Code, Business, and Learning have dedicated templates and the rest fall back to the general template.
- `crates/prompt-engine-wasm`: WebAssembly bindings so the engine runs in-browser, with no server.
- `crates/shared-types`: API contracts shared by server and engine.
- `docs`: product, architecture, security, and QA decisions.

## CLI

The engine runs without the browser. Build with `cargo build --release` and call it
directly:

```sh
prompt-optimize "corrige mon code python qui plante"
echo "write unit tests for my parser" | prompt-optimize --lang en --json
```

## MVP rules

- No external LLM call.
- No automatic message sending.
- No user prompt text in logs.
- No business logic in the content script.
- No IO, network, database, or async runtime in `prompt-engine`.

## WebAssembly

The engine compiles to WebAssembly, so it can run in the browser with no server
(nothing leaves the machine). Build the bindings and run the smoke test that
calls the engine through WASM:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
sh scripts/build-wasm.sh nodejs pkg
node scripts/wasm-smoke.mjs        # -> fr / code / corriger
```

Use `sh scripts/build-wasm.sh web pkg` to produce a browser bundle. The
extension's service worker loads this WASM and optimizes prompts on-device,
falling back to the local server only if the WASM module is unavailable
(`npm --prefix apps/extension run build` bundles it into `dist/`).

## Quality

- `cargo test` runs unit, integration, property-based (proptest), and doc tests.
- `cargo bench` runs a Criterion benchmark of the engine (~0.7 µs per call).
- CI enforces `cargo fmt`, `clippy -D warnings`, tests, and the extension checks.

