# Red-mutation record for `miso-engine-core`'s realtime primitives

Master plan for issue #83, section 1.6: *every gate is proven red*. Each row below was applied to
the working tree, the named test binary was run, the result was recorded, and the mutation was
reverted in the same session. Nothing here is a claim about code that was not run.

Host: `x86_64` (AMD Ryzen 7 9700X, Zen 5), workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`, debug profile.

## Issue #143 E11 — the conflating observation cell

`tests/observation_transport.rs`, one million published windows against a concurrent reader.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 143-E11-a | drop the closing `sequence_lock` store, so the seqlock never closes | `core/src/realtime/observe.rs` | `observation_transport` | RED — 7 219 of 31 256 reads torn, and all three tests fail |
| 143-E11-b | publish the fields without first making the counter odd | `core/src/realtime/observe.rs` | `observation_transport::a_million_windows_are_read_whole_and_in_order` | RED — 12 109 of 100 384 reads torn |
| 143-E11-c | the reader ignores the counter entirely and returns whatever it loaded | `core/src/realtime/observe.rs` | `observation_transport::a_million_windows_are_read_whole_and_in_order` | RED — 190 529 of 342 298 reads torn |
| 143-E11-d | `acknowledge` stores unconditionally, so the consumed word can move backwards | `core/src/realtime/observe.rs` | `observation_transport::a_stalled_reader_resumes_on_the_newest_window_with_a_counted_gap` | RED — the consumed word regresses from `1001` to `5`, and the missed-window count becomes meaningless |

### Measured, not assumed

`MAXIMUM_READ_ATTEMPTS` was set to `4` first. Against a writer publishing continuously in a tight
loop that measured a **50% give-up rate** (130 472 absent against 128 543 whole reads), because
four six-word reads are comparable in length to the seven-word store they race. It is `64`, which
takes the measured give-up rate to a few percent under the same stress while keeping the loop
provably finite. Production publishes once per closed window — once per `window_blocks` render
blocks — so this bound is set for the stress case, not for the shipped one.
