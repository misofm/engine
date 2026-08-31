# Red-mutation log — `protocol` tests

## Issue #178 — the `builtins` rack token

Driver: one mutation at a time on the committed tree,
`cargo test -p protocol --test session_edit_builtins_rack`, tree restored between rows.

| # | mutation | file | test | result |
|---|---|---|---|---|
| P3-M45 | `rack_mut`'s `RackName::Builtins` arm returns `Ok(&mut track.simd1)` instead of refusing | `protocol/src/model.rs` | `rack_addressed_edits_refuse_the_builtins_token` | RED — four rack-addressed edits start reporting `Ok`, and a `PutTrackEffect` addressed at the strip lands an effect in `simd1` |

The two positive tests in that file are what stop a refusal that refuses everything from passing:
the same edits against `RackName::Simd1` are applied, and the strip is still editable through
`SetTrackBuiltins`, which is the edit that owns it.

A third shape is worth naming and is **not** a mutation, because it is what the arm was written to
avoid: an `unreachable!()` there would abort the control thread on a well-formed wire message. The
wire decodes `RACK = 4` into `RackName::Builtins` by construction (`schema.rs`), so a rack-addressed
edit carrying it is a message a peer can legally send.

## Issue #241 — deleted session-edit opcodes

Applied on 2026-08-29, run, observed RED, and reverted.

| gate | mutation | observed red |
|---|---|---|
| `session_wire::tests::deleted_source_and_limits_opcodes_are_typed_refusals` | alias deleted `0x0006` (`SetLimits`) to the live `SetSourceContent` decoder in `SessionEditOpcode::from_raw` | decode returns `Ok(SetSourceContent { … })` where `Err(InvalidTlv)` is required; the assertion names deleted opcode `0x0006` before payload dispatch |

The same gate independently mutates the opcode TLV to deleted per-source-rate `0x0102` and source
mapping `0x0104`. Its positive neighbor round-trips the new `{content,channels,bit_depth,frames}`
payload canonically, so the refusal cannot pass by disabling source-edit decoding wholesale.
