# Browser session rebuild cost (#1289, B1 of #1269)

Module `30d075d3ce6382f21235675996184c675753acf6d451e11d7d676a3d50aaeff4` at `0b477c585237033c3289bb76bbf5c9a18bc8c9d2`, Node v22.23.2 (V8 12.4.254.21-node.56, --no-liftoff), uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived loadavg_above_ceiling; loadavg 28.07 24.74 16.14 20/2281 2870557; affinity cpu 31.

Time of `miso_engine_web_v1_boot` on a fresh instance, 25 boots per document per round; one quantum's budget is 128 / 48000 s = 2.667 ms. Descriptive only.

| document | tracks | round | boot p50 ms | boot max ms | p50 / budget | max / budget | dispose p50 ms | peak Wasm memory MiB |
|---|---|---|---|---|---|---|---|---|
| nine_track_eq | 9 | 1 | 2.695 | 11.385 | 1.0 | 4.3 | 0.031 | 1.8 |
| sixty_four_track_console | 64 | 1 | 23.916 | 33.470 | 9.0 | 12.6 | 0.220 | 5.4 |
| sixty_four_track_app_shape | 64 | 1 | 23.424 | 32.394 | 8.8 | 12.1 | 0.217 | 5.5 |
| sixty_four_track_console_sends | 64 | 1 | 39.120 | 49.115 | 14.7 | 18.4 | 0.465 | 11.1 |
| nine_track_eq | 9 | 2 | 2.684 | 11.920 | 1.0 | 4.5 | 0.030 | 1.8 |
| sixty_four_track_console | 64 | 2 | 23.916 | 37.236 | 9.0 | 14.0 | 0.205 | 5.4 |
| sixty_four_track_app_shape | 64 | 2 | 23.523 | 32.990 | 8.8 | 12.4 | 0.208 | 5.5 |
| sixty_four_track_console_sends | 64 | 2 | 39.169 | 48.515 | 14.7 | 18.2 | 0.453 | 11.1 |
