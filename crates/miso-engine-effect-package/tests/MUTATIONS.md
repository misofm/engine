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
* Red: `error[E0432]: unresolved import
  miso_engine_effect_package::verify_canonical_package_v1`.
