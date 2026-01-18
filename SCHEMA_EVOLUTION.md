# Schema evolution rules

## Principles
- Protobuf is the wire contract. Changes must be compatible with existing consumers.
- Prefer additive changes.

## Allowed without major version bump
- Add new fields with new field numbers.
- Add new message types.
- Add new enum values (append-only).

## Requires a major version bump (or new topic family)
- Removing or renaming fields.
- Changing field types or semantics.
- Renumbering fields.

## Buf gates
- buf lint must pass.
- buf breaking must pass against main.
