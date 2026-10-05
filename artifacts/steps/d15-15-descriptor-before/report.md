# Browser session rebuild cost (#1289, B1 of #1269)

Module `a9a518625f4c0621c00a515be5a1c50b257601e9a7e492dbac45dbbcb5637b55` at `3ade8e969581154dcf3492889eceeda5dbffccc1`, Node v22.23.2 (V8 12.4.254.21-node.56, --no-liftoff), uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived loadavg_above_ceiling; loadavg 95.49 98.36 75.27 77/3644 130129; affinity cpu 31.

Time of `miso_engine_web_v1_boot` on a fresh instance, 25 boots per document per round; one quantum's budget is 128 / 48000 s = 2.667 ms. Descriptive only.

| document | tracks | round | boot p50 ms | boot max ms | p50 / budget | max / budget | dispose p50 ms | peak Wasm memory MiB |
|---|---|---|---|---|---|---|---|---|
| nine_track_eq | 9 | 1 | 4.665 | 27.821 | 1.7 | 10.4 | 0.040 | 1.8 |
| sixty_four_track_console | 64 | 1 | 46.559 | 88.664 | 17.5 | 33.2 | 0.299 | 5.4 |
| sixty_four_track_app_shape | 64 | 1 | 44.947 | 96.899 | 16.9 | 36.3 | 0.306 | 5.5 |
| sixty_four_track_console_sends | 64 | 1 | 80.701 | 202.326 | 30.3 | 75.9 | 0.628 | 11.1 |
| nine_track_eq | 9 | 2 | 5.119 | 24.546 | 1.9 | 9.2 | 0.042 | 1.8 |
| sixty_four_track_console | 64 | 2 | 50.437 | 103.721 | 18.9 | 38.9 | 0.302 | 5.4 |
| sixty_four_track_app_shape | 64 | 2 | 45.674 | 96.751 | 17.1 | 36.3 | 0.295 | 5.5 |
| sixty_four_track_console_sends | 64 | 2 | 74.795 | 142.156 | 28.0 | 53.3 | 0.616 | 11.1 |
