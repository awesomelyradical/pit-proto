# PIT Platform - AI Agent Contract (Repo Rules)

## Repository Identity
- Repository: `awesomelyradical/pit-proto`
- Local path: `/home/aric/code/pit-project/pit-proto`
- Role: Authoritative protobuf schema and artifact publishing repository for PIT platform contracts.

## Prime Directive
This repository is the source of truth for PIT protobuf schemas and generated artifacts.

## Non-Negotiables
1. All .proto changes happen here.
2. Buf checks are mandatory for schema changes.
3. Breaking changes must be prevented or explicitly versioned.
4. Generated artifacts are expected here when release process requires them.
5. Service runtime logic belongs in service repos, not this repo.

## Required Validation for Schema Changes
```bash
buf lint
buf breaking --against '.git#branch=main'
buf generate
```

## Platform Alignment
When schema changes affect topics/envelope semantics, coordinate updates in:
- `/home/aric/code/pit-project/pit/pit_platform_docs/PIT_PLATFORM_SPEC.md`
- `/home/aric/code/pit-project/pit/pit_platform_docs/TOPICS_AND_KEYS.md`
- implementing service repos

## Governance
Changes to governing docs require explicit human approval in PR review.
