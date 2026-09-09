# Qualify the frozen continuous-mapping extraction

GitHub: https://github.com/misofm/engine/issues/670

Parent: #560 (lane B, CP8)

Supersedes stopped #669 for qualification only. #669 exhausted three test and
evidence attempts without SOURCE PASS. Its production extraction is nevertheless
semantically correct per three Astra LOW reviews: `effect-contract` owns pure
`continuous_mapping_admissible`, and the typed and borrowed validators call it
at the former duplicated mapping/minimum condition without moving surrounding
validation. This successor qualifies those frozen source bytes with independently
valid public fixtures. It is a new bounded workflow after the required rescope,
not a fourth #669 attempt.

Sol HIGH coordinates documentation, checkpoints, GitHub synchronization, and
artifact qualification/pinning. The sole implementation executor is Luna HIGH
`/root/issue583_luna_impl`. Per current user routing, Astra LOW performs every
scope, source, exact-head, integration, artifact-applicability, and delivery
verification that would otherwise use Sol HIGH/XHIGH. #668 and #669 are closed;
#670 is the sole active audit child at opening.

## Frozen base and ownership

The isolated successor branch starts at #669 disposition
`c139a0f0788a1e3b1b023ed14dce214b1aa79020`, whose merge-base with main is
`7d16d9c9752c9ac2d31e69008fe075df86ce3c26`. Frozen production hashes are:

- `crates/effect-contract/src/lib.rs`:
  `dfe5ba9b47e65998e997f7485e6b8bccbab666cc02c2e1b0f5a6036d0509d8b1`
- `crates/effect-package/src/wire.rs`:
  `a9284433c1ee5a802cd99bbcdd911d48d8c099c6627774f397541a69d81a57e0`

Only inline `#[cfg(test)]` modules in those two source files, mechanical
formatting of those tests, and this numbered spec/evidence may change. The helper,
both production callers, imports needed by production, descriptors, diagnostics,
wire format, formulas, dependencies, lockfile, other tests, artifacts, pins,
workflows, and policies are frozen. Preserve #669's three worktree branches,
temporary evidence/targets, and exact failed patches unchanged.

## Fixture contract

Use one literal nine-minimum sequence for every mapping: negative finite `-1.0`,
negative zero, positive zero, smallest positive subnormal `f32::from_bits(1)`,
smallest positive normal `f32::MIN_POSITIVE`, ordinary positive `1.0`, NaN,
negative infinity, and positive infinity. Every numeric binding is explicitly
`f32`. The full matrix is four mappings by nine minima, exactly 36 cases.

For canonical finite minima at or below zero, use maximum `1.0` and default
`0.5`. For ordinary positive `1.0`, use maximum `2.0` and default `1.5`. The two
tiny positive values may use maximum `1.0` and default `0.5`; they deliberately
reach the independent decimal-lattice rule. Nonfinite spellings are parser
controls and keep otherwise fixed finite maximum/default fields. Derive the
declared lattice for the case's unit/domain/mapping exactly as production encoding
does; do not reuse a lattice derived for another mapping.

The typed projection must build an otherwise-valid public `EffectDescriptor`:
one required dual-mono main input, one required dual-mono main output, Normal
quality rows at 44.1/48/88.2/96 kHz with valid state/latency/tail/scratch, valid
link modes, and exactly the subject parameter. Call public `validate_descriptor`.
For each case assert the exact `parameters`/`Parameter` diagnostic presence or
absence. Fully lattice-valid admissible cases must return `Ok(())`; the two tiny
admissible minima must have no Parameter diagnostic and the exact coupled Lattice
diagnostic. Invalid mapping cases must carry the exact Parameter diagnostic; any
coupled Lattice diagnostic must be stated literally rather than ignored.

The borrowed projection must call only public
`verify_effect_descriptor_wire`; private semantic views/predicates are forbidden.
Assert exact public outcomes for all 36 cases:

- negative zero and all nonfinite minima: `Code::Float`, byte offset
  `HEADER_BYTES + 36`, record index 0;
- canonical finite mapping-law failures: `Code::Semantic`, byte offset
  `HEADER_BYTES + 4`, record index 0;
- the two tiny values with Linear, Logarithmic, or Exponential:
  `Code::Semantic`, byte offset `HEADER_BYTES + 72`, record index 0, proving the
  mapping law passed before the lattice law rejected their decimal spelling;
- lattice-valid Linear/Exponential cases at `-1.0`, positive zero, and `1.0`,
  plus Logarithmic at `1.0`: successful verification with exact input bytes.

Add a combined nonfinite-minimum/mapping-failure control proving the earlier
Float diagnostic. For each successful case, independently compute SHA-256 over
`IDENTITY_DOMAIN`, the little-endian `u64` input length, and exact input bytes,
then compare with `verified.identity().as_bytes()`. Do not use
`effect_descriptor_identity` to compute expected bytes. Retain and explicitly
exercise the existing phase-order, overflow/tie-break, typed/borrowed differential,
roundtrip/identity, and binding-mismatch tests named in #669.

## Attempt 1 execution contract

Before any edit, Luna must record exact clean HEAD/upstream, merge-base/live
main, cwd, `rustc --version`, `cargo --version`, frozen production hashes, and
the explicit unset/value state of `CARGO_TARGET_DIR`, `RUSTFLAGS`,
`RUSTDOCFLAGS`, `CARGO_ENCODED_RUSTFLAGS`, `CC`, `CFLAGS`, and
`SOURCE_DATE_EPOCH`. Verify every #669 path is preserved. These five #670 paths
must be absent and non-symlinks:

- `/tmp/issue670-attempt1-evidence`
- `/tmp/issue670-attempt1-target`
- `/tmp/issue670-attempt1-manifest-record.txt`
- `/tmp/issue670-attempt1-manifest-verify.stdout`
- `/tmp/issue670-attempt1-manifest-verify.status`

Create only the evidence and target directories. Prove harmless success status
0 and harmless failure status 1, persist and independently read back both. After
test edits and mechanical formatting, run these commands exactly once in order,
exporting `CARGO_TARGET_DIR=/tmp/issue670-attempt1-target` for every Cargo command:

1. `cargo test --locked -p effect-contract`
2. `cargo test --locked -p effect-package`
3. `cargo test --locked --release -p effect-contract`
4. `cargo test --locked --release -p effect-package`
5. `cargo clippy --locked -p effect-contract -p effect-package --all-targets --all-features -- -D warnings`
6. `cargo fmt --all -- --check`
7. `bash scripts/check-effect-runtime-policy.sh`
8. `bash scripts/check-effect-package-v1.sh`
9. `bash scripts/check-effect-descriptor-v1.sh`
10. `bash scripts/check-workspace-policy.sh`
11. `git diff --check`

Capture every exact command, separate stdout/stderr, and numeric status under the
evidence directory. Stop on the first unexpected status without correction,
retry, or later gate. If all pass, finalize every file in the evidence directory,
write its self-excluding `SHA256SUMS`, and place all manifest command/completion,
verification output, and numeric status only in the three sibling paths. Current
`sha256sum -c` must pass every entry. Commit no temporary evidence, target output,
`.ll`, `.s`, or full compiler stream.

Implementation remains unauthorized until the clean pushed brief and synchronized
#559/#560/#670 bodies receive Astra LOW scope PASS. One Luna attempt is initially
available; subsequent attempts follow the repository's three-attempt rule. After
SOURCE PASS, root owns artifact applicability and any required qualification/pin
successor. No artifact, pin, PR, or merge work is included in this brief.

## Astra LOW scope review — PASS

Astra LOW returned **SCOPE PASS** at exact clean feature HEAD/upstream
`a68e418232d9512a86df4cfd2901ee0ec33a66a3`, main merge-base
`7d16d9c9752c9ac2d31e69008fe075df86ce3c26`, synchronized #670/#559/#560
bodies, and all five fresh paths absent including symlinks. Independent source
inspection confirmed precision-1 dB Linear/Exponential accept the proposed
ordinary values; logarithmic ratio 1.02/precision 3 accepts `1.0/2.0/1.5`; and
both tiny positive minima retain the intended later-lattice rejection.

Only Luna HIGH `/root/issue583_luna_impl` is authorized for one qualification-
only attempt limited to inline tests and formatting with production frozen. The
literal typed diagnostic sets and all 36 public borrowed outcomes are mandatory.
Run the eleven gates once under the evidence contract and stop on the first
unexpected failure without correction or retry. Preserve all #669 history and
temporary records. No artifact, pin, PR, or merge work is authorized.
