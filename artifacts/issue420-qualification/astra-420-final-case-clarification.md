# #420 final-attempt case clarification

This is a concrete implementation aid for the already issued attempt2 FAIL, not another verdict, new gate or authorization to change production. Keep the existing old-width primitive oracle, f32/Simd4/Simd8 instantiations and lengths `1, max(W-1,1), W, W+1, 3W+1,128`. A handful of separately named nine-contributor families is sufficient; do not build a Cartesian corpus.

**Use the existing canonical FP guard around BOTH old and DUT execution in this test:** `lane::fpenv::CanonicalFpEnv::enter()`, retaining its guard until comparison completes. This is the actual render policy, not a newly assumed flush convention. Current fpenv.rs explicitly pins round-to-nearest-even with FTZ/DAZ disabled; Wasm has full subnormal arithmetic. D7's separate recursive-state flush does not belong in sum2/sum_into or general reduction. Do not mutate raw control words, add unsafe, accept zero because the ambient host happened to enable FTZ, or compare outputs produced under different environments. The existing guard restores the caller's thread environment on scope exit.

For each output frame use one of these rows as its ordered contributor values; pad to nine contributors as stated:

| Family | Ordered values for one frame | Required OLD-output assertion before DUT bit equality |
|---|---|---|
| Finite arithmetic/order | Alternate frames between `[2.0,-0.5,0.25,+0.0…]` and `[16777216.0,1.0,-16777216.0,+0.0…]` | First pattern exactly1.75, second exactly+0.0; all outputs finite. Keep existing explicit wrong-association/subtotal tests as the independent wrong-result proof. |
| Negative zero | All nine inputs `-0.0` | Every output exactly `0x80000000`, not numeric equality with zero. |
| Small normal | `[f32::MIN_POSITIVE,f32::MIN_POSITIVE,+0.0…]` | Every output exactly `0x01000000`, nonzero finite normal; this also detects inappropriate blanket state-style flushing. |
| Subnormal | `[f32::from_bits(1),f32::from_bits(1),+0.0…]` | Every output exactly bits2, nonzero subnormal under the canonical environment. No tolerance or category-only comparison. |
| Infinity without invalid cancellation | Alternate frames `[+infinity,1.0,+0.0…]` and `[-infinity,-1.0,+0.0…]` | Exact signed infinity according to the frame; no NaN, and no opposite-signed infinity in the same sum. |
| NaN separately | `[f32::from_bits(0x7fc04201),+0.0…]` | Old result is NaN, then DUT bits equal that SAME-width old result. Do not assert one universal NaN payload across architectures or unrelated execution arms. The existing richer NaN rotation may also be retained but is not needed for the other categories. |

The frame1 shape naturally exercises only one member of an alternating family; larger existing shapes cover both. If convenient swap which finite pattern begins a run so the ordering pattern is also directly tested at frame1; the existing dedicated frame1 ordering test already proves that host path, so do not expand this into another matrix.

Generate buffers from these rows, execute the unchanged old sum2 followed by seven sum_into steps, assert the category/exact value specified, then compare every DUT bit. Runtime-loaded values and existing lane kernels avoid treating a hand-computed scalar expression as the only oracle. Both vector body and scalar tail remain exercised at each width. Nine contributors already meet this targeted repair; existing finite65/129 tests and16-input subgroup witness remain intact.

The only other completion from the attempt2 report is a concise honest workload-link record: name the console fixture's64 tracks and64 ordered post-matrix routes to main-out, the PlumbingOnly transformation's preserved routing, the track-count assertion/unbanked compiler arm and the representative private64-input lowering test. Checked read-only source/fixture evidence is sufficient; no public runtime inspection API or additional workload is required. Do not describe the synthetic lowering test as if it directly instantiated SessionRuntime.

Retain accepted ownership/sentinel/allocation/source tests. Run the proportional affected debug/release proof after the single coherent revision; root owns checkpoint and final Astra review. No source, timing, artifact or runner work is authorized by this clarification. Read-only current FP policy and old kernel inspection only; no tests/Cargo/timing or repository/GitHub mutation performed.
