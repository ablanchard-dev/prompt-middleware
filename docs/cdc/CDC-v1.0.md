# Prompt Middleware CDC v1.0

## Non-negotiable MVP principles

- Local-first by default.
- The V1 engine uses only local rules, heuristics, scoring, and templates.
- No external LLM dependency in the MVP.
- No automatic send in the MVP.
- Preview is mandatory before replacement.
- No data leaves the machine without explicit consent.

## Clean architecture

```text
extension = browser user interface
local-server = local HTTP transport and config
prompt-engine = pure business intelligence
shared-types = API contracts
docs = decisions and specifications
tests = validation by layer
```

Rule: a feature that works but violates these boundaries is not done.

## Definition of Done

- Code is implemented in the right layer.
- Architecture boundaries are respected.
- Code is formatted.
- Relevant tests are added or updated.
- Tests pass.
- No logs contain user prompt text.
- Errors are typed and handled.
- User behavior is validated.
- Minimal documentation is updated when needed.
- Ticket acceptance criteria are validated.

