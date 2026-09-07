First coherent tranche complete; paused for root checkpoint.

- Changed only [crates/true-peak-limiter/src/lib.rs](/home/bl/misofm/engine-limiter-stationary-dispatch/crates/true-peak-limiter/src/lib.rs)
- Source SHA-256: `5b25b0337de2106c58d48edd71dac0332eedce61322fa4ee0988e729defce28d`
- Git blob: `5b21199dc5df1a477407e7daa1613646702c5de0`
- Focused test: `cargo test --locked -p true-peak-limiter --lib stationary_dispatch_matches_runtime_oracle_and_observes_selected_body`
- Result: `1 passed`, 29 filtered, status `0`
- Coverage: W8 dual/mono uniform/per-lane and scalar dual/uniform; PCM/full-state/ramp and mono right-plane witness.
- Target: `/tmp/issue539-luna2-target`

Raw captures:

- Final green: `/tmp/issue539-luna2/focused-debug-3.{command.json,stdout,stderr,status}`
- Earlier compile failure and corrected green preserved under `focused-debug-1` and `focused-debug-2`.

No commit, push, GitHub, issue, or agent updates performed.