# Prompt Middleware

[![CI](https://github.com/ablanchard-dev/prompt-middleware/actions/workflows/ci.yml/badge.svg)](https://github.com/ablanchard-dev/prompt-middleware/actions/workflows/ci.yml)

Local-first browser middleware that optimizes user prompts before they are sent to existing LLM interfaces.

## Architecture

- `apps/extension`: browser UX only.
- `apps/local-server`: local HTTP transport and configuration.
- `apps/cli`: standalone `prompt-optimize` command-line front-end for the engine.
- `crates/prompt-engine`: pure, data-driven prompt optimization engine (templates overridable via `EngineConfig`).
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

## Quality

- `cargo test` runs unit, integration, property-based (proptest), and doc tests.
- `cargo bench` runs a Criterion benchmark of the engine (~0.7 µs per call).
- CI enforces `cargo fmt`, `clippy -D warnings`, tests, and the extension checks.

