# Issue #274 — the Wasm parity re-pin, and why the gate never asked

`scripts/check-protocol-wasm-parity.sh` could not fail. Its Wasm-side copy of the complete-schema
corpus digest therefore sat at `0x88a8_ee6a_6d9e_4acc` while the native pin was re-pinned twice,
and both arms reported green. This document derives the correct constant **without running the
Wasm arm**, so that the gate's first honest green is a confirmation rather than a definition.

**Headline: there was no real target divergence.** The Wasm arm computes exactly the value the
native derivation predicts. What existed was a stale duplicate of a constant, kept alive by an
inert gate, which is a different defect with the same smell.

---

## 1. Why the gate could not fail

The gate ran, for each of two builds:

```
wasm-interp --run-all-exports "$artifact"
```

under `set -euo pipefail`, and treated a zero exit status as parity. Two independent facts of
wabt 1.0.34 make that untestable, and both were confirmed against the installed interpreter:

1. **`--run-all-exports` skips exports that take parameters.** The module's only function export
   is `main`, and a Rust `main` on `wasm32-unknown-unknown` is the C entry point, Wasm signature
   `(i32, i32) -> i32` (`argc`, `argv`). `wasm-interp` runs no export with arguments, so it
   invoked *nothing*: it parsed and type-checked the module, printed no output at all, and exited
   0. The assertion in the guest never executed.
2. **A trap does not reach the exit status.** When `main` *is* invoked by name, a guest trap is
   reported as the text `main(i32:0, i32:0) => error: unreachable executed` on stdout, and
   `wasm-interp` still exits 0.

Measured on `bd260c90`, with the pin deliberately mutated to `0xdead_beef_dead_beef`:

| invocation | printed | exit |
| --- | --- | ---: |
| `wasm-interp --run-all-exports a.wasm` | *(nothing)* | 0 |
| `wasm-interp a.wasm -r main` | `Exported function 'main' expects 2 arguments, but 0 were provided` | 1 |
| `wasm-interp a.wasm -r main -a i32:0 -a i32:0` | `main(i32:0, i32:0) => error: unreachable executed` | 0 |

The whole script, with a garbage constant, printed `issue-005 Wasm golden parity: ok (scalar +
simd128)` and exited 0. Fact (1) is the reason the gate was inert; fact (2) is the reason the
obvious fix — invoking the export — would still have been inert.

## 2. What the constant should be, derived without the Wasm arm

The digest is defined by `complete_schema_hash()`: FNV-1a-64 (offset basis
`0xcbf2_9ce4_8422_2325`, prime `0x0000_0100_0000_01b3`) rolled over each frame of
`complete_schema_corpus()` as `(name.as_bytes(), bytes)`, in corpus order.

The corpus frames were dumped at four revisions (a scratch `#[test]` that writes each frame's
label and bytes; the dump reads the corpus, never a pinned constant) and the roll was recomputed
by an **independent Python implementation** of FNV-1a-64 over those dumps. No Wasm binary is
involved in any row:

| revision | frames | corpus bytes | recomputed digest |
| --- | ---: | ---: | --- |
| `29538d71` *(last revision at which the Wasm pin was true)* | 46 | 10,416 | `0x88a8_ee6a_6d9e_4acc` |
| `b454b230` `Track delay: regenerate the session corpus` | 46 | 10,480 | `0xeb7a_a549_b666_77a8` |
| `04d291dd` #241 `Implement canonical PCM source schema` | 46 | 10,176 | `0xbdeb_b0f8_1c38_ec42` |
| `bd260c90` *(`main` today)* | 46 | 10,176 | `0xbdeb_b0f8_1c38_ec42` |

Each recomputed value reproduces the native pin that `crates/protocol/tests/
conformance_corpus.rs` carried at that revision, and the 04d291dd row reproduces
`docs/derivations/241-schema-repins.md` §3 independently — including its measured corpus total of
10,176 bytes and its claim that exactly one frame moves.

**The current value is therefore `0xbdeb_b0f8_1c38_ec42`**, and the Wasm pin was stale by *two*
re-pins, not one. Issue #274 names #241; #241 is only the second half.

### 2.1 The two moves, as byte arithmetic

Exactly one of the 46 frames moves in each step: `command.session_transaction_apply`, the
all-opcode transaction. Every other frame is byte-identical across all four revisions, which is
why the frame count never moves.

BTLV framing (`docs/CONTROL_BTLV_V1.md`): a field is `id:u16, wire:u8, flags:u8, len:u32` = 8
header bytes, then the value padded to the next 8-byte boundary. A scalar field of width <= 8
therefore occupies **16** bytes.

**`b454b230`: +64 bytes** (6,280 -> 6,344). The commit adds `delay_samples` to every builtins
lane in every session document. Its wire form is `FieldSpec::req(5, Wire::U32)`
(`schema.rs:803`), one scalar field = 16 bytes, and `DualMonoBuiltins` has two lanes. Two edits
in the all-opcode fixture carry a builtins section: `UpsertTrack` (`0x0200`), which embeds a whole
`Track`, and `SetTrackBuiltins` (`0x0203`), which is that section on its own.

```
2 edits x 2 lanes x 16 bytes = 64
6280 + 64 = 6344   ->   corpus 10,416 + 64 = 10,480
```

**`04d291dd` (#241): -304 bytes** (6,344 -> 6,040). Derived edit by edit in
`docs/derivations/241-schema-repins.md` §3 — three opcodes deleted (`0x0006`, `0x0102`, `0x0104`,
-336 bytes), `0x0103 SetSourceContent` re-shaped (+64) and `0x0100 UpsertSource` re-shaped (-32):
`-336 + 64 - 32 = -304`. Re-measured here from the frame bytes at both revisions and matching.

```
6344 - 304 = 6040   ->   corpus 10,480 - 304 = 10,176
```

## 3. Was there a real divergence?

No. With the gate repaired, the scalar and `simd128` guests both compute
`0xbdeb_b0f8_1c38_ec42` — the value derived in §2 from the corpus bytes alone — and return
success. The native `conformance_corpus` test asserts the same value. Native and Wasm agree, and
have agreed throughout; what drifted was a second hand-written copy of the answer.

That copy is now gone. `COMPLETE_SCHEMA_HASH` lives once, in
`crates/protocol/src/conformance.rs`, next to the corpus it pins. The native test
asserts it; the Wasm guest computes the digest itself and compares against it. A future re-pin is
one edit, and the two arms can no longer disagree by omission — only by genuinely computing
different bytes, which is the divergence this gate exists to catch.

## 4. What the gate does now

`main` returns its verdict as a **value** rather than as a panic: `ExitCode::SUCCESS` when the
guest-computed digest equals `COMPLETE_SCHEMA_HASH`, `ExitCode::from(1)` when it does not. The
script invokes the export by name and requires the interpreter to print exactly

```
main(i32:0, i32:0) => i32:0
```

Silence is refused (that is fact (1) above), any `error:` line is refused (fact (2)), a nonzero
interpreter status is refused, and `=> i32:1` is refused. `--self-test` proves the discrimination
against real rebuilds rather than against a story: a control row on an unmutated scratch copy must
be green, and four rows must each come back red — the pre-#274 `--run-all-exports` invocation of a
*correct* artifact, the historical `0x88a8_ee6a_6d9e_4acc` pin, a garbage pin, and a guest-side
panic. `scripts/sweep.sh` runs the row as `--self-test`, so the discrimination is checked every
sweep and not only when someone remembers.
