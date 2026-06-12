# ADR-0002 - Pure Prompt Engine

## Decision

`crates/prompt-engine` must remain pure, synchronous, and portable to WASM.

## Forbidden in prompt-engine

- HTTP clients.
- Axum or server frameworks.
- Tokio or async runtime dependencies.
- SQLite or filesystem runtime access.
- Environment variable reads.

