# Canonical JSON migration inventory

Issue #338 froze this inventory from baseline `51468d5d`. Classification is semantic: a `.toml`
suffix alone does not make a file a Session V1 document, and truthful historical records are not
rewritten.

## Live contract and implementation names

These are migration targets: the `session` parser/model/writer/diagnostic/compiled snapshot;
host-core, WebAssembly and C/native document entry points; `sessionTomlBytes` and C compile-limit
source names; protocol snapshot field 3 and `canonical_toml_chunk`; builtins plan seals; SDK
`toToml()` and boot paths; enginectl, validators and native runners; generated ABI declarations;
active architecture/control/session/runner/SDK documentation; policy and packaging scripts; and
`.claude/skills/author-session/SKILL.md`.

The session-authority tranche migrates `crates/session/**`, `AGENTS.md`,
`docs/SESSION_SCHEMA_V1.md`, and establishes the three JSON authority fixtures
`canonical-minimal.json`, `canonical.json`, and `parametric-eq-nine-track.json`. Their retired TOML
copies remain temporarily because unmigrated host/tool consumers still include them directly;
they are not inputs accepted by the session crate and are deleted with those consumers in the
subsequent tranches. Remaining live contract names belong to the host/protocol and SDK/tools
tranches.

## Current session fixtures

Baseline contains 35 live session-document TOMLs: 14 under `fixtures/session/v1`, ten builtins
benchmark sessions, five native-runner sessions, and six host-web browser/qualification sessions.
Each is a migration target. Within `fixtures/session/v1`, the three authority fixtures above gain
their sole authoritative JSON form in tranche 1 while their old paths remain temporary consumer
inputs as inventoried above. The remaining observation/console fixtures stay temporarily TOML only
until their direct consumers migrate in the later fixture tranche; none is accepted by the new
session authority.

## Generic configuration

Cargo manifests/configuration, rustfmt configuration, workflow configuration and unrelated tool
inputs written in TOML are not session documents and do not migrate. In particular, `Cargo.toml`
and dependency references to a file named `Cargo.toml` remain TOML.

## Immutable historical evidence

Accepted `.github/ISSUE_SPECS/`, archived benchmark/evidence records, derivations and old ruling
prose may retain accurate TOML names and byte identities. New live inputs cannot use those names.
The final policy allowlist must enumerate any historical match still visible to a live-name scan;
the category is not a blanket directory exemption.

## Reproducible audit

The migration audit uses:

```sh
rg -n -i 'toml|SessionToml|parse_session_toml|canonical_session_toml|canonical_toml|sessionTomlBytes|toToml|maximum_toml_bytes' \
  AGENTS.md crates hosts sdk tools sidecars fixtures docs scripts .claude .github
find fixtures/session/v1 fixtures/builtins/v1/benchmark fixtures/native-pcm-runner \
  hosts/host-web/tests/browser-v1 hosts/host-web/qualification -type f -name '*.toml' -print
```

Reviewers classify additions into the four sections above. The policy gate introduced in the final
migration tranche turns unclassified live contract/current-fixture matches red while preserving
Cargo/configuration TOML and the explicit historical allowlist.
