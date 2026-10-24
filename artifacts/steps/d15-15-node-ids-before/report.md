# Browser session rebuild cost (#1289, B1 of #1269)

Module `06bb9bc52828f2ee4ceb759194505f6ab5192d90c33ad90c17949a54f24fa8cf` at `82053706cfd77629b3d6f928d449dbfcd6c437e2`, Node v22.23.2 (V8 12.4.254.21-node.56, --no-liftoff), uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived loadavg_above_ceiling; loadavg 0.64 0.82 1.08 1/1618 196072; affinity cpu 31.

Time of `miso_engine_web_v1_boot` on a fresh instance, 25 boots per document per round; one quantum's budget is 128 / 48000 s = 2.667 ms. Descriptive only.

| document | tracks | round | boot p50 ms | boot max ms | p50 / budget | max / budget | dispose p50 ms | peak Wasm memory MiB |
|---|---|---|---|---|---|---|---|---|
| nine_track_eq | 9 | 1 | 3.539 | 7.693 | 1.3 | 2.9 | 0.040 | 3.1 |
| sixty_four_track_console | 64 | 1 | 28.976 | 40.447 | 10.9 | 15.2 | 0.206 | 6.1 |
| sixty_four_track_app_shape | 64 | 1 | 28.244 | 36.969 | 10.6 | 13.9 | 0.213 | 6.1 |
| sixty_four_track_console_sends | 64 | 1 | 46.152 | 54.392 | 17.3 | 20.4 | 0.424 | 10.6 |
| nine_track_eq | 9 | 2 | 3.533 | 7.991 | 1.3 | 3.0 | 0.040 | 3.1 |
| sixty_four_track_console | 64 | 2 | 29.172 | 39.227 | 10.9 | 14.7 | 0.206 | 6.1 |
| sixty_four_track_app_shape | 64 | 2 | 28.386 | 37.248 | 10.6 | 14.0 | 0.216 | 6.1 |
| sixty_four_track_console_sends | 64 | 2 | 46.688 | 61.400 | 17.5 | 23.0 | 0.422 | 10.6 |
