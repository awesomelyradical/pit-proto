# Contributing

## Scope
Authoritative protobuf schema and artifact publishing repository for PIT platform contracts.

## Non-negotiables
- Keep changes small and reviewable.
- Schema changes require Buf validation and compatibility checks.
- Generated artifacts may be updated here as part of schema release workflow.

## Required Commands (Schema Changes)
```bash
buf lint
buf breaking --against '.git#branch=main'
buf generate
```

## PR Checklist
- Buf checks pass.
- Generated artifacts are consistent with updated schemas.
- Downstream impact is documented (service/library repos).
- ADR added/updated for contract-level behavior changes.
- Governing document changes explicitly request human approval.
