# AI agent operating rules (repo-local)

This repo is designed to be modified by both humans and AI agents.

## Allowed
- Improve docs, comments, tests, and observability.
- Implement well-scoped tasks in ai/tasks/.
- Small refactors that reduce complexity without changing semantics.

## Not allowed
- Silent schema or topic/key changes.
- Changing security boundaries or custody assumptions.
- Introducing new dependencies without justification.

## How to work
- Prefer a single, focused PR per task.
- Add or update an ADR when behavior contracts change.
