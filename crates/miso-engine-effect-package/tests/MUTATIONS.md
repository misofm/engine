# Red-mutation log — audit #97 (`miso-engine-effect-package`)

Every gate this job lands was proven non-vacuous by applying the mutation below, running the named
command, observing the recorded failure, and reverting. A gate with no red mutation is not
evidence.

Delivery host: x86_64 with AVX2+FMA (`x86-64-v3`), rustc 1.97.1, release timings from
`cargo test --locked --release`.

---

## F1 — canonical ordering by a bounded stack index

### M-01 — sort the index with the allocating stable sort
* Mutation: in `package.rs::canonical_order`, `order.sort_unstable_by(...)` →
  `order.sort_by(...)`.
* Command: `cargo test --locked --release -p miso-engine-effect-package --test package_allocation`
* Red: `encode_at_the_frozen_artifact_cap_has_one_nested_descriptor_pass_and_no_native_allocation`
  — `left: Snapshot { allocations: 9, ..., peak_live_bytes: 8192 }` vs
  `right: Snapshot { allocations: 8, ..., peak_live_bytes: 736 }`. The extra allocation is std's
  8 KiB merge buffer; the pre-existing fixture tests (≤ 5 artifacts, under the len-20 threshold)
  stay green, so only the n = 4,096 gate sees it.

### M-02 — report the wrong member of a duplicate-key pair
* Mutation: in `canonical_order`, `.map(|pair| u32::from(pair[1]))` → `pair[0]`.
* Command: `cargo test --locked -p miso-engine-effect-package --lib`
* Red: `authoring_duplicate_source_invariant_and_native_feature_grammars` —
  `left: (Order, 0) / right: (Order, 1)`. Proves the duplicate diagnostic still names the
  smallest caller index that *repeats* an earlier key, exactly as the old prefix scan did.

### M-03 — emit the canonical index in reverse
* Mutation: in `encode_effect_package_v1`, `for &slot in &order[..len]` →
  `for &slot in order[..len].iter().rev()`.
* Command: `cargo test --locked -p miso-engine-effect-package`
* Red: 9 failures including `round_trip_layout_and_borrows`,
  `every_table_grammar_padding_order_hash_and_source_class_rejects`,
  `exact_and_one_below_limits_cover_all_five_caps` and the four selection tests. Proves the frozen
  record order (and therefore the package SHA-256 and CID) is pinned by the existing corpus.

### M-04 — restore the O(n²) `next_artifact` scan
* Mutation: reinstate `fn next_artifact` and drive the encode loop from it
  (`prior`/`filter`/`min_by`), leaving everything else in place.
* Command: `cargo test --locked --release -p miso-engine-effect-package --test package_allocation`
* Red: `encode_at_the_frozen_artifact_cap_...` —
  `required_size + encode at the artifact cap took 4.38258815s` against the 10 ms budget
  (measured 3.44 ms after the fix, a 1,274× margin over the mutation).

### M-05 — write an allocating stable sort into the package-native surface
* Mutation: same source edit as M-01.
* Command: `bash scripts/check-effect-package-v1.sh`
* Red: script exits non-zero. The tightened grep matches directly:
  `n.rs:263: order.sort_by(|a, b| key_cmp(at(*a), at(*b)).then(a.cmp(b)))` →
  `effect package V1 check failure: allocation/unsafe package-native surface`.

## F4 — the package fuzz target is actually built

### M-06 — break the package fuzz target's import
* Mutation: in `fuzz/fuzz_targets/effect_package.rs`, `verify_effect_package_v1` →
  `verify_canonical_package_v1` (the API the audit found stale).
* Command: `cargo check --locked --manifest-path fuzz/Cargo.toml --bins`, now also invoked from
  `scripts/check-effect-package-v1.sh`
* Red: `error[E0432]` — the import of
  `miso_engine_effect_package::verify_canonical_package_v1` does not resolve. (The compiler's own
  wording is paraphrased here so the #97 placeholder scan over this crate stays clean.)

## F5 — one descriptor pass in the C inspect entry

### M-07 — restore the second full verification pass
* Mutation: in `ffi.rs::miso_engine_effect_descriptor_v1_inspect`,
  `let identity = *verified.identity().as_bytes();` →
  `let identity = *effect_descriptor_identity_v1(wire_bytes, maximum_wire_bytes)...as_bytes();`
  (the shape the audit found).
* Command: `cargo test --locked -p miso-engine-effect-package --test package_allocation`
* Red: `c_inspect_performs_exactly_one_nested_descriptor_pass` —
  `left: Snapshot { allocations: 16, allocated_bytes: 2000, ... }` vs
  `right: Snapshot { allocations: 8, allocated_bytes: 1000, ... }`, exactly double.
* Oracle caveat (from the #97 plan, for whoever closes F7): this gate's oracle is the descriptor
  semantic pass's heap use. If `wire.rs` is ever made allocation-free the assertion becomes
  `0 == 0` and that job must replace the oracle with a `#[cfg(test)]` verify counter.

### M-08 — treat a null wire pointer as `Null` regardless of length
* Mutation: in the `mandatory_null` predicate, `|| (wire.is_null() && wire_len != 0)` →
  `|| wire.is_null()`.
* Command: `cargo test --locked -p miso-engine-effect-package --test package_allocation`
* Red: `c_inspect_reports_a_wire_diagnostic_for_an_empty_null_wire` — `left: 1` (`Null`) vs
  `right: 4` (`Header`, what the verifier returns for empty input).

## F3 — the state envelope binds the effect identity it names

Issue 079's rewrite of `state.rs` resolved F3 as reported:
the placeholder-ID construction and the unchecked `SHA-256(effect_id)` are gone, `bypass` decodes
only 0/1, and `quality`/`link_mode`/`sidechain_*`/`tail_kind` are range checked. No production
change was needed here; the gates below prove it and cover the one property 079 left untested.

### M-09 — stop comparing the envelope's effect-ID text to the bound descriptor
* Mutation: in `validate_effect_state_current_layout_v1`, delete
  `if state.effect_id != descriptor.id.as_str() { return Err(metadata_mismatch(1)); }`.
* Command: `cargo test --locked -p miso-engine-effect-package --test state_vectors`
* Red: `the_state_envelope_binds_the_effect_identity_it_names` — the renamed-`test.stats` envelope
  is accepted instead of yielding `Metadata` detail 1.

### M-10 — stop comparing the envelope's descriptor identity to the bound token
* Mutation: in `bind_parsed_effect_state_v1`, delete
  `if parsed.descriptor_identity != bound.identity() { return Err(unavailable(Code::Descriptor,
  3 << 16)); }`.
* Command: `cargo test --locked -p miso-engine-effect-package --test state_vectors`
* Red: 5 tests fail, including 079's own
  `independent_reference_malformed_oracle_matches_exact_diagnostics`,
  `representative_mutations_have_exact_phase_order_and_diagnostics` and
  `the_state_envelope_binds_the_effect_identity_it_names`.

### Already covered by 079 (cited, not re-added)
* Canonical `bypass`: `state_vectors.rs` `("bypass-enum", 108, 2, 108)` in
  `every_state_header_field_and_payload_class_has_an_exact_diagnostic` — wire value 2 is `Enum` at
  byte offset 108, so `encode(verify(x)) == x` cannot silently normalise a non-canonical boolean.
* Descriptor-identity mismatch: the `"descriptor-identity"` case in
  `independent_reference_malformed_oracle_matches_exact_diagnostics` and in
  `every_state_header_field_and_payload_class_has_an_exact_diagnostic` — `Descriptor`, detail
  `3 << 16`, unavailable index and offset.
* Round trip: `exact_wire_round_trip_preserves_independent_sections_and_suffixes` and
  `independent_reference_vector_binds_verifies_and_reencodes_byte_identically`.

## Issue #143 — the additive observation wire section

| # | mutation | file | test | result |
|---|---|---|---|---|
| 143-E10-a | the encoder writes the real section offset into header word 92 even for a zero-tap descriptor (drop the `observations == 0 => 0` rule in `Layout::header_observation_offset`) | `effect-package/src/wire.rs` | `cargo test -p miso-engine-effect-package` | RED — 23 of 34 tests fail, including the frozen `comprehensive-a`/`-b` wire and identity fixtures: **every** non-dynamics identity moves, which is exactly the failure the two-word header exists to prevent |
| 143-E10-b | `OBSERVATION_BYTES = 40` instead of 32 | `effect-package/src/wire.rs` | `effect-compiler` `observation_identity::every_declared_tap_costs_exactly_its_record_and_its_two_strings` | RED — `miso.compressor: the section is exactly its records plus its strings`, 56 vs 48 |
| 143-E9-a | the schema gate accepts any boolean `subscribable` instead of deriving it from the cost class | `scripts/check-parameter-metadata-v1.py` | `check-parameter-metadata-v1.py --self-test` | RED — two mutations escape: `a computed tap claims to be subscribable`, `a resident tap denies being subscribable` |

The Python reference carries its own observation mutation matrix
(`observation_mutation_matrix` in `scripts/effect-descriptor-v1-reference.py`): sixteen cases over
the tap-bearing `comprehensive-c` vector — the record's reserved word, tap order, all six
vocabularies, both float slots, inverted bounds, a `Computed` tap claiming `PerBlock` cadence, the
section offset, the section count, and the two string-pool ownership rules. Each asserts the exact
`(code, byte_offset, record_index, detail)` diagnostic and fails if the mutation is accepted.

## Issue #127 — the nudge ladder in the parameter record's reserved window

| # | mutation | file | test | result |
|---|---|---|---|---|
| 127-13 | the encoder stops writing byte 77, the ratio class | `effect-package/src/wire.rs` | `cargo test -p miso-engine-effect-package --test descriptor_v1_qualification` | RED — 3 of 8, including the frozen `comprehensive-d` wire and identity fixtures and the round trip against the static descriptor |
| 127-14 | the reserved rule for a ladder-free window is dropped (`if bytes[record + 76] == 0` leg) | `effect-package/src/wire.rs` | same | RED — `the_nudge_window_is_reserved_whether_or_not_a_ladder_is_declared` |
| 127-14c | the reserved rule for a *declared* window's two-byte tail is dropped | `effect-package/src/wire.rs` | same | RED — same test |
| 127-15 | `compare_static_descriptor` reads the declared `xs` back out of the wire instead of out of the descriptor | `effect-package/src/wire.rs` | same | RED — `a_declared_nudge_ladder_costs_no_bytes_and_moves_the_identity`: a wire whose xs rung is a different size binds |
| 127-20 | the ratio-class enum check becomes unconditional `true` | `effect-package/src/wire.rs` | same | RED — the closed vocabulary accepts `3`, and the presence-bit case panics on the `unwrap` the check exists to protect |
| 127-21 | `borrowed_semantic_errors` stops running the ladder rules | `effect-package/src/wire.rs` | same | RED — `a_wire_whose_ladder_breaks_a_rule_is_semantically_invalid` |

The Python reference carries its own nudge mutation matrix (`nudge_mutation_matrix` in
`scripts/effect-descriptor-v1-reference.py`): thirteen cases over the ladder-bearing
`comprehensive-d` vector — both reserved legs, the two vocabularies, both canonical-float rules,
and every one of the three ladder rules (zero and negative `xs`, a fractional choice count, an
absolute rung on a logarithmic mapping, a cents rung on a linear one, and an `xl` rung that crosses
the whole domain). Each asserts the exact `(code, byte_offset, record_index, detail)` diagnostic
and fails if the mutation is accepted. It also asserts the byte accounting directly:
`len(comprehensive-d) == len(comprehensive-a)`, the identities differ, and the only bytes that move
outside the string pool are inside the eight-byte windows.
