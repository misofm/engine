# Red-mutation log — `miso-engine-protocol` tests

## Issue #178 — the `builtins` rack token

Driver: one mutation at a time on the committed tree,
`cargo test -p miso-engine-protocol --test session_edit_builtins_rack`, tree restored between rows.

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
