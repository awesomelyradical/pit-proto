# Contributing

## Non-negotiables
- Keep changes small and reviewable.
- Do not commit generated artifacts unless explicitly called out for this repo.
- Prefer schema-first and event-driven integration: change protobuf + topics/keys before writing glue.

## Local dev
- Use a Kind cluster for dev unless otherwise specified.
- Build container images locally, then load them into Kind.

## PR checklist
- Tests pass.
- Lint/formatters pass.
- Docs updated if behavior changed.
- Any event/topic/schema changes include an ADR.
