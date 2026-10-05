# Browser session rebuild cost (#1289, B1 of #1269)

Module `b723ff9f4325c6cfe38913c53c29cabec5107a3a548b93f1c983a8f3c55eb48d` at `a9f09e29aff75487ead6c73b9d50fde9529d87db`, Node v22.23.2 (V8 12.4.254.21-node.56, --no-liftoff), uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived loadavg_above_ceiling; loadavg 39.22 52.44 65.74 47/2791 390463; affinity cpu 31.

Time of `miso_engine_web_v1_boot` on a fresh instance, 25 boots per document per round; one quantum's budget is 128 / 48000 s = 2.667 ms. Descriptive only.

| document | tracks | round | boot p50 ms | boot max ms | p50 / budget | max / budget | dispose p50 ms | peak Wasm memory MiB |
|---|---|---|---|---|---|---|---|---|
| nine_track_eq | 9 | 1 | 3.554 | 12.697 | 1.3 | 4.8 | 0.038 | 1.8 |
| sixty_four_track_console | 64 | 1 | 33.824 | 48.635 | 12.7 | 18.2 | 0.249 | 5.5 |
| sixty_four_track_app_shape | 64 | 1 | 33.112 | 46.306 | 12.4 | 17.4 | 0.240 | 5.5 |
| sixty_four_track_console_sends | 64 | 1 | 54.393 | 70.593 | 20.4 | 26.5 | 0.531 | 11.1 |
| nine_track_eq | 9 | 2 | 3.508 | 11.499 | 1.3 | 4.3 | 0.038 | 1.8 |
| sixty_four_track_console | 64 | 2 | 32.589 | 47.397 | 12.2 | 17.8 | 0.248 | 5.5 |
| sixty_four_track_app_shape | 64 | 2 | 31.989 | 48.897 | 12.0 | 18.3 | 0.242 | 5.5 |
| sixty_four_track_console_sends | 64 | 2 | 54.368 | 101.564 | 20.4 | 38.1 | 0.530 | 11.1 |

## Interpretation (added after verification, #1330 batch follow-ups)

This record and its pair (`d15-15-descriptor-before/`, base, loadavg 95.49; `d15-15-descriptor-after/`, branch, loadavg 39.22) are confounded by host load (≈95 -> ≈39) and cannot measure the change. The issue spec's Attempt record cites the verifier's uncommitted back-to-back pair at loadavg 23-25, which puts the change at roughly 12-20 % p50 faster.
