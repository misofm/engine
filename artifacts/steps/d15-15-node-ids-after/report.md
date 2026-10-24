# Browser session rebuild cost (#1289, B1 of #1269)

Module `31b970c33dfaa8737bba199e13ff1a3854aa18fa74eec8b3e4b3144216ee048d` at `80980574512a95ea7d6b74c6f52d906e8746b402`, Node v22.23.2 (V8 12.4.254.21-node.56, --no-liftoff), uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived loadavg_above_ceiling; loadavg 1.01 0.90 1.11 2/1692 196687; affinity cpu 31.

Time of `miso_engine_web_v1_boot` on a fresh instance, 25 boots per document per round; one quantum's budget is 128 / 48000 s = 2.667 ms. Descriptive only.

| document | tracks | round | boot p50 ms | boot max ms | p50 / budget | max / budget | dispose p50 ms | peak Wasm memory MiB |
|---|---|---|---|---|---|---|---|---|
| nine_track_eq | 9 | 1 | 3.184 | 11.742 | 1.2 | 4.4 | 0.040 | 3.1 |
| sixty_four_track_console | 64 | 1 | 25.396 | 26.112 | 9.5 | 9.8 | 0.187 | 6.0 |
| sixty_four_track_app_shape | 64 | 1 | 24.591 | 38.156 | 9.2 | 14.3 | 0.201 | 5.9 |
| sixty_four_track_console_sends | 64 | 1 | 40.169 | 48.937 | 15.1 | 18.4 | 0.396 | 10.6 |
| nine_track_eq | 9 | 2 | 3.179 | 11.529 | 1.2 | 4.3 | 0.040 | 3.1 |
| sixty_four_track_console | 64 | 2 | 25.278 | 27.325 | 9.5 | 10.2 | 0.191 | 6.0 |
| sixty_four_track_app_shape | 64 | 2 | 24.460 | 33.862 | 9.2 | 12.7 | 0.198 | 5.9 |
| sixty_four_track_console_sends | 64 | 2 | 39.939 | 48.123 | 15.0 | 18.0 | 0.399 | 10.6 |
