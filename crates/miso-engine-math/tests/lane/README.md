# Lane-layer tests, pending job 83a

`m1_exhaustive.rs` is gate M1 for `src/lane_math.rs`. Both are complete but not wired into the
build: they need the `Lane` trait from `crates/miso-engine-lane`, which job 83a of issue #83 owns
and which had not been pushed when 83b was delivered. Cargo only auto-discovers integration tests
directly under `tests/`, so keeping this one a directory down leaves the workspace gates green
instead of failing on a dependency that does not exist yet.

To wire it up, in one commit:

1. add `lane = ["dep:miso-engine-lane"]` to `[features]` and
   `miso-engine-lane = { workspace = true, optional = true }` to `[dependencies]` in
   `crates/miso-engine-math/Cargo.toml`, plus the matching `workspace.dependencies` entry in the
   root manifest;
2. in `src/lib.rs`, add

   ```rust
   #[cfg(feature = "lane")]
   mod lane_math;
   #[cfg(feature = "lane")]
   pub use lane_math::{exp2_lane, log2_lane};
   ```

   and restore the intra-doc links in the crate header;
3. move `m1_exhaustive.rs` up to `tests/`, and declare it in `Cargo.toml` as

   ```toml
   [[test]]
   name = "m1_exhaustive"
   required-features = ["lane"]
   ```

4. run `cargo test --locked --release -p miso-engine-math --features lane --test m1_exhaustive -- --ignored`
   and confirm the printed maxima against the numbers recorded in the test's module documentation
   (1.4615 ulp for `exp2_lane`, 1.4667 ulp for `log2_lane`).

Gate M2 (lane identity: the same bits at width 1, 4 and 8) is not written here at all. It needs
83a's backend dispatch entry point to instantiate the generic function at `F32x4`/`F32x8`, and
those types are deliberately not nameable outside the lane crate, so the shape of that test is
83a's to fix. `exp2_lane`/`log2_lane` use nothing but `Lane` trait operations, so 83a's own G1
per-op identity gate already covers every operation they are built from.
