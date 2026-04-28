# Agent Tasks — pit-proto (schema source of truth)

> **Read first**: `pit/docs/PROGRAM_PLAN.md` §4 (Agent operating rules)

## Repo role
Authoritative protobuf schema repository. All `.proto` files and generated artifacts originate here.
Service runtime logic does NOT belong in this repo.

## Active workstreams
WS-A (contract convergence), WS-B (CI integrity)

---

## Ready queue

### T-001: Release pit-proto v0.3.0 with envelope field expansion
- **Status**: ready
- **Workstream**: WS-A → M1
- **Files in scope**: `proto/pit/envelope/v1/envelope.proto`, `buf.yaml`, `buf.gen.yaml`, release tooling
- **Out of scope**: any service repo (lib-* repos handle their own bumps)
- **Acceptance criteria**:
  - `buf lint` passes.
  - `buf breaking --against '.git#branch=main'` passes (additions are non-breaking).
  - `buf generate` produces clean output.
  - Tag `v0.3.0` pushed; release artifacts published.
  - Release notes link `pit/docs/adr/0002-envelope-field-expansion.md`.
- **Validation**:
  ```
  buf lint && buf breaking --against '.git#branch=main' && buf generate
  ```
- **Reference**: `pit/docs/adr/0002-envelope-field-expansion.md`

### T-002: Add `buf breaking` as a required CI check
- **Status**: completed 2026-04-28 — workflow `.github/workflows/buf-breaking.yaml` added
- **Workstream**: WS-B → M2
- **Files in scope**: `.github/workflows/**`
- **Acceptance criteria**:
  - PRs run `buf breaking --against 'https://github.com/${GITHUB_REPOSITORY}.git#branch=main'`. ✅
  - Pinned to `buf` v1.62.1 via `bufbuild/buf-setup-action@v1`.
  - Zero soft-fail bypasses. ✅
  - **Follow-up (human action required)**: enable the `buf-checks` job as a required status check in branch protection.

### T-003: Audit envelope schema consumers for field-number assumptions
- **Status**: ready
- **Workstream**: WS-A → M1
- **Acceptance criteria**:
  - Confirm no consumer code in `pit-lib-{go,python,rust}` hardcodes field numbers; all access is
    by generated accessor.
  - Document finding in this `AGENT_TASKS.md` under "Recently completed".
- **Note**: read-only audit task; if violations are found, file blocking tasks in the offending
  `pit-lib-*` repo, do NOT fix from this repo.

---

## Blocked / waiting
None.

## Recently completed
- 2026-04-27 — Envelope fields 12–22 added to `proto/pit/envelope/v1/envelope.proto` (ADR-0002)

## Repo-specific gotchas for agents
- **Do not** add service runtime code here. This repo is schemas + generated artifacts only.
- **Do not** introduce breaking changes without a major version bump and explicit ADR.
- All schema changes ripple to `pit-lib-{go,python,rust}` and downstream services. Coordinate via
  `pit/docs/PROGRAM_PLAN.md` WS-A sequence.
- `gen/**` is committed here (this is the only repo where that's allowed).
