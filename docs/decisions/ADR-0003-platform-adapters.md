# ADR-0003 - Platform Adapters

## Decision

Browser-specific DOM logic is isolated in `apps/extension/src/platforms/*`.

## Consequences

- No platform-specific URL checks outside the platform registry.
- ChatGPT, Claude, Gemini, and DeepSeek can evolve independently.

