# Agent Quick Start

Last updated: 2026-02-12

## Pre-flight Checks
```bash
cd /home/aric/code/pit-project/pit-proto
pwd
# expected: /home/aric/code/pit-project/pit-proto

git remote -v
# expected remote name: pit-proto
```

## Read Before Coding
- `.github/copilot-instructions.md`
- `CONTRIBUTING.md`
- Platform contracts in `/home/aric/code/pit-project/pit/pit_platform_docs/`

## Working Rules
- Use focused branches: `workstream-<letter>/<description>`
- Keep changes scoped and reviewable
- Include concrete test/lint output in PR
- Flag spec drift explicitly if docs and implementation are temporarily misaligned

## Repository Reminder
- GitHub repo: `awesomelyradical/pit-proto`
- Role: Authoritative protobuf schema and artifact publishing repository for PIT platform contracts.
