# Prompt Middleware

[![CI](https://github.com/ablanchard-dev/prompt-middleware/actions/workflows/ci.yml/badge.svg)](https://github.com/ablanchard-dev/prompt-middleware/actions/workflows/ci.yml)

Local-first browser middleware that optimizes user prompts before they are sent to existing LLM interfaces.

## Architecture

- `apps/extension`: browser UX only.
- `apps/local-server`: local HTTP transport and configuration.
- `crates/prompt-engine`: pure prompt optimization engine, designed for future WASM.
- `crates/shared-types`: API contracts shared by server and engine.
- `docs`: product, architecture, security, and QA decisions.

## MVP rules

- No external LLM call.
- No automatic message sending.
- No user prompt text in logs.
- No business logic in the content script.
- No IO, network, database, or async runtime in `prompt-engine`.

