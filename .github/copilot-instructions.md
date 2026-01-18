# PIT Platform — AI Agent Contract (Coding Rules)

These rules are non-negotiable unless a PR explicitly says "RFC / ADR required" and includes the approved ADR link.

## 0) Prime Directive

This platform is an event-sourced system with Kafka as the immutable spine. Services must not smuggle business logic via ad-hoc topics, ad-hoc schemas, or out-of-band side effects.

## 1) Non-Negotiables (Hard Rules)

### 1.1 Kafka Payload Rule

All Kafka messages MUST be `pit.envelope.v1.EventEnvelope`.

Consumers MUST reject non-envelope messages (except temporary migration code explicitly marked `MIGRATION_ONLY`).

### 1.2 Envelope Invariants

`EventEnvelope` MUST include:

- `event_id` (globally unique)
- `ts` (event time)
- `producer` (service name + version)
- `topic` (optional but recommended)
- `key` (canonical key string)
- `tenant`, `series`, `cell` as separate fields (cell may be empty for series-scoped events)
- `payload_type` (fully-qualified protobuf message name)
- `payload` (protobuf bytes)
- (recommended) `payload_sha256` for audit/attestation friendliness

Consumers MUST validate:

- `key` matches `tenant`/`series`/`cell` fields (no cross-tenant leakage)
- `payload_type` matches the message expected for the topic/domain

### 1.3 Keying Rule (Scale Primitive)

PITs scale by key, not by topic.

Canonical key format:

- Cell-scoped: `t/{tenant}/s/{series}/c/{cell}`
- Series-scoped: `t/{tenant}/s/{series}`
- Venue: `venue/{venue}` (only for ops/health domains)

Ordering guarantees are assumed only per key, never globally.

### 1.4 Money Rule

No floats for money outside toy demos.

Production events use integer micros (e.g., `int64` micros, `ccy`).

Toy vertical-slice code may use floats only under `cmd/` and must be clearly labeled.

### 1.5 Delivery Semantics

Assume at-least-once. Every consumer must be idempotent.

Offsets are committed only after downstream side effects succeed (produce/write/store).

### 1.6 No Generated Code Committed

Do not commit generated protobuf output (`gen/**`).

Generated artifacts come from pit-proto releases (Go module, Python wheel, Rust crate).

If you see committed generated code, remove it and fix the build to pull artifacts instead.

## 2) Repo Boundaries (What Gets Edited Where)

### 2.1 pit-proto is Authoritative

All `.proto` live in the pit-proto repo.

Schema changes happen there, with:

- `buf lint`
- `buf breaking` (against main)
- version bump + release of language artifacts

### 2.2 This Repo Consumes Published Artifacts

Services import generated code from published pit-proto artifacts:

- **Go**: module dependency (recommended: `github.com/awesomelyradical/pit-proto/...`)
- **Python**: wheel/package dependency (recommended: `pit-proto`)
- **Rust**: crate dependency (recommended: `pit-proto` or `pit-proto-types`)

This repo may keep a pinned Buf module dependency to validate schemas (optional), but it is not the source of truth.

## 3) Topics, Catalog, and Drift Control

### 3.1 Topic Creation/Changes

No new topics without updating:

- `infra/topics.yaml` (catalog/intent)
- the deploy manifests (K8s topic CRs / Terraform / whatever provisions topics)

Topic naming: `pit.v{N}.{domain}.{event}`

Domain topics only (`exec`/`acct`/`risk`/`policy`/`strategy`/`runtime`/`checkpoint`/`ops`). Never "one topic per PIT".

### 3.2 Retention/Compaction Policy

Immutable audit topics: long retention (or offload), `cleanup.policy=delete`

State topics: `cleanup.policy=compact` or `compact,delete` with explicit retention windows

Never rely on Kafka "auto-create topics".

## 4) Security + Trust Boundaries

### 4.1 Transport

Kafka connections use mutual TLS.

TLS secrets are mounted from Strimzi `KafkaUser` (or equivalent). No embedded certs in images.

### 4.2 Authorization Model (Expected Shape)

Each service uses its own Kafka principal/cert.

Prefer topic-level ACLs + (later) tenant scoping enforced by envelope validation and service logic.

Never bypass envelope validation "because it's internal".

### 4.3 Attestation Readiness

Design events so an external verifier can reproduce decisions:

- stable schemas
- deterministic checkpoints
- event ranges/hashes for proofs (later)

## 5) Implementation Patterns

### 5.1 Producers

Must set `event_id`, `ts`, `producer`, `key` fields, and payload fields.

Must include schema/type in the envelope (`payload_type`).

Use idempotent producer configuration when available (Kafka idempotence).

### 5.2 Consumers

Must:

- parse envelope
- validate invariants
- decode payload based on `payload_type`
- apply logic
- write outputs
- commit offsets after success

### 5.3 Dedupe / Idempotency

Use `event_id` as the canonical dedupe key.

Preferred: store processed event_ids with TTL (Redis/RocksDB/Postgres) per consumer group.

If a consumer writes to a DB, enforce idempotency via unique constraints on `event_id`.

## 6) Language-Specific Rules

### 6.1 Rust (Hot Path)

Use `rdkafka` with SSL/TLS.

Avoid hidden proto compilation in runtime containers. Prefer pit-proto crate dependency.

Errors: use `anyhow`/`thiserror`. No panics in long-running services.

### 6.2 Go (Control Plane / Accounting)

Prefer `segmentio/kafka-go` with TLS dialer.

Manual commit after side effects.

No float money in production structs; use micros.

### 6.3 Python (Research/Analytics)

OK to be flexible and exploratory, but still obey envelope rules.

Python services should not become production-critical truth sources.

## 7) Workflows (What to Run Locally / in CI)

### 7.1 Schema Changes (in pit-proto Repo)

```bash
buf lint
buf breaking --against '.git#branch=main'
buf generate
# publish artifacts (Go module / Python wheel / Rust crate)
```

### 7.2 Service Changes (This Repo)

Update dependencies to the new pit-proto release.

Build containers; run a smoke test that produces one event and observes downstream output.

## 8) Examples (Follow These Patterns)

### 8.1 Envelope Key Fields

```
key: t/demo/s/alpha/c/0001
tenant: demo
series: alpha
cell: 0001
payload_type: pit.events.v1.TradeFillEvent
```

### 8.2 "Vertical Slice" Demo Expectations

- `tradefill-producer` emits enveloped `TradeFillEvent` to `pit.v1.exec.trade_fill`
- `nav-consumer` consumes fills, emits enveloped `NAVUpdateEvent` to `pit.v1.acct.nav_update`
- `nav-analytics` consumes nav updates and prints/plots

## 9) What Not to Do (Common Failure Modes)

- Don't add a new topic "just for this feature."
- Don't emit raw protobuf payloads to Kafka (envelope is mandatory).
- Don't commit generated code.
- Don't introduce money floats into production messages.
- Don't commit Kafka offsets before producing/writing outputs.</content>
<parameter name="filePath">/home/aric/code/pit/.github/copilot-instructions.md