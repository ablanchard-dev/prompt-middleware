# ADR-0001 - Local-first MVP

## Decision

The MVP runs locally by default. The browser extension calls a Rust server bound to `127.0.0.1`.

## Consequences

- User prompt text is not sent to a remote service.
- Installation is more complex than a pure extension.
- Future WASM support is planned to reduce this friction.

