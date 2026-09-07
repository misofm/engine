# Issue 431 coordinator command capture

This file transcribes the original external executor records for the sole Issue 431 capture. The
source is the durable Codex session record
`rollout-2026-09-07T10-57-12-01a07b83-d50f-7ad0-9e90-bef0ef5a03e2.jsonl`. The identifiers below
name the original `CommandExecution` records. Nothing here was rerun to create this file.

All four commands ran with `/bin/bash -lc` in
`/home/bl/misofm/engine-431-full-chain-capture`. The executor recorded no explicit per-command
environment overrides. It did not emit the complete inherited process environment, so those
historical values are unavailable and are not reconstructed here. The committed READY seal records
the actual build tools, executable hashes, versions, target, target features and release profile;
the committed disposition and accepted rows preserve the benchmark child environment and
controlled-load note. The preflight also rejected incompatible Rust/Cargo build override families
before building.

## Preflight

- Timestamp: `2026-09-07T12:03:38.675Z`
- Executor ID: `exec-b44a551e-70bd-4fbf-8c3d-2cd36eea210c`
- Process ID: `53289`
- Command: `bash scripts/preflight-builtins-current-benchmark.sh`
- Status: completed, exit 0
- Stderr: empty
- Stdout, exactly as recorded by the executor:

```text
   Compiling bytemuck v1.25.2
   Compiling engine v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/engine)
   Compiling version_check v0.9.5
   Compiling cfg-if v1.0.4
   Compiling proc-macro2 v1.0.107
   Compiling lexical-util v1.0.7
   Compiling unicode-ident v1.0.24
   Compiling quote v1.0.47
   Compiling libc v0.2.189
   Compiling typenum v1.20.1
   Compiling syn v1.0.109
   Compiling once_cell v1.21.4
   Compiling autocfg v1.5.1
   Compiling smallvec v1.15.2
   Compiling cpufeatures v0.3.0
   Compiling utf8-decode v1.0.1
   Compiling decoded-char v0.1.1
   Compiling locspan v0.8.2
   Compiling semver v1.0.28
   Compiling bitflags v2.13.1
   Compiling dsp-reference v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/dsp-reference)
   Compiling safe_arch v1.2.0
   Compiling smallstr v0.3.1
   Compiling proc-macro-error-attr v1.0.4
   Compiling ahash v0.7.8
   Compiling proc-macro-error v1.0.4
   Compiling rustc_version v0.4.1
   Compiling indexmap v1.9.3
   Compiling flatbuffers v25.12.19
   Compiling wide v1.6.1
   Compiling lexical-write-integer v1.0.6
   Compiling lexical-parse-integer v1.0.6
   Compiling hybrid-array v0.4.14
   Compiling lexical-parse-float v1.0.6
   Compiling lexical-write-float v1.0.6
   Compiling getrandom v0.2.17
   Compiling lexical-core v1.0.6
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling lexical v7.0.5
   Compiling json-number v0.4.10
   Compiling hashbrown v0.12.3
   Compiling digest v0.11.3
   Compiling sha2 v0.11.0
   Compiling locspan-derive v0.6.0
   Compiling bench-support v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/tools/bench-support)
   Compiling json-syntax v0.12.5
   Compiling session v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/session)
   Compiling protocol v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/protocol)
   Compiling lane v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/lane)
   Compiling math v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/math)
   Compiling effect-contract v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/effect-contract)
   Compiling effect-runtime v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/effect-runtime)
   Compiling rack v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/rack)
   Compiling builtins v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/builtins)
   Compiling transient-shaper v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/transient-shaper)
   Compiling compressor v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/compressor)
   Compiling delay v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/delay)
   Compiling gate-expander v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/gate-expander)
   Compiling multiband-compressor v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/multiband-compressor)
   Compiling parametric-eq v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/parametric-eq)
   Compiling soft-clip v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/soft-clip)
   Compiling true-peak-limiter v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/true-peak-limiter)
   Compiling conformance v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/conformance)
   Compiling graph v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/graph)
   Compiling rack-compiler v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/rack-compiler)
   Compiling builtins-compiler v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/builtins-compiler)
   Compiling effect-package v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/effect-package)
   Compiling effect-compiler v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/effect-compiler)
   Compiling graph-compiler v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/crates/graph-compiler)
   Compiling console-workload v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/tools/console-workload)
   Compiling bench v0.1.0 (/home/bl/misofm/engine-431-full-chain-capture/tools/bench)
    Finished `release` profile [optimized + debuginfo] target(s) in 48.15s
Issue-431 current benchmark preflight: READY (preflight/runner/workload/timed=1/0/0/0)
```

## Rejected readiness sample

- Timestamp: `2026-09-07T12:04:21.908Z`
- Executor ID: `exec-4e77adf4-0822-48a5-b617-33439a9324e8`
- Process ID: `81883`
- Status: failed, exit 3, deliberately rejected because binary age was below 60 seconds
- Stderr: empty
- Command:

```bash
source scripts/check-bench-preconditions.sh
online=$(cat /sys/devices/system/cpu/online)
bench_cpu=$(bench_highest_cpu "$online")
now=$(date +%s)
mtime=$(stat -c %Y target/issue431-prepared/bench)
age=$((now-mtime))
loadavg=$(cat /proc/loadavg)
load_one=$(bench_loadavg_one_minute "$loadavg")
siblings_text=$(cat "/sys/devices/system/cpu/cpu$bench_cpu/topology/thread_siblings_list")
siblings=$(bench_other_siblings "$bench_cpu" "$siblings_text")
printf 'online=%s cpu=%s age=%s load_one=%s siblings=%s\n' "$online" "$bench_cpu" "$age" "$load_one" "$siblings"
taskset -c "$bench_cpu" true
bench_within_ceiling "$load_one" "$MISO_ENGINE_BENCH_LOADAVG_CEILING"
if [[ -n "$siblings" ]]; then
  stat_before=$(cat /proc/stat)
  sleep "$MISO_ENGINE_BENCH_SIBLING_SAMPLE_SECONDS"
  stat_after=$(cat /proc/stat)
  for sibling in $siblings; do
    busy=$(bench_cpu_busy_percent "$stat_before" "$stat_after" "$sibling")
    printf 'sibling_%s_busy=%s\n' "$sibling" "$busy"
    bench_within_ceiling "$busy" "$MISO_ENGINE_BENCH_SIBLING_BUSY_CEILING"
  done
fi
if ((age < MISO_ENGINE_BENCH_COOLDOWN_SECONDS)); then exit 3; fi
ps -eo comm,args | rg 'cargo|rustc|wasm-opt|chromium|firefox' || true
```

The original tool result preserved this complete output (the lower-level command event retained
only its final line):

```text
online=0-31 cpu=31 age=43 load_one=0.47 siblings=15
sibling_15_busy=0.00
```

## Accepted readiness sample

- Timestamp: `2026-09-07T12:05:10.107Z`
- Executor ID: `exec-45e7584f-e01c-4533-95e8-c527ca1830fb`
- Process ID: `55804`
- Status: completed, exit 0
- Stderr: empty
- Command:

```bash
sleep 20
source scripts/check-bench-preconditions.sh
online=$(cat /sys/devices/system/cpu/online)
bench_cpu=$(bench_highest_cpu "$online")
now=$(date +%s)
mtime=$(stat -c %Y target/issue431-prepared/bench)
age=$((now-mtime))
loadavg=$(cat /proc/loadavg)
load_one=$(bench_loadavg_one_minute "$loadavg")
siblings_text=$(cat "/sys/devices/system/cpu/cpu$bench_cpu/topology/thread_siblings_list")
siblings=$(bench_other_siblings "$bench_cpu" "$siblings_text")
printf 'online=%s cpu=%s age=%s load_one=%s siblings=%s\n' "$online" "$bench_cpu" "$age" "$load_one" "$siblings"
taskset -c "$bench_cpu" true
((age >= MISO_ENGINE_BENCH_COOLDOWN_SECONDS))
bench_within_ceiling "$load_one" "$MISO_ENGINE_BENCH_LOADAVG_CEILING"
if [[ -n "$siblings" ]]; then
  stat_before=$(cat /proc/stat)
  sleep "$MISO_ENGINE_BENCH_SIBLING_SAMPLE_SECONDS"
  stat_after=$(cat /proc/stat)
  for sibling in $siblings; do
    busy=$(bench_cpu_busy_percent "$stat_before" "$stat_after" "$sibling")
    printf 'sibling_%s_busy=%s\n' "$sibling" "$busy"
    bench_within_ceiling "$busy" "$MISO_ENGINE_BENCH_SIBLING_BUSY_CEILING"
  done
fi
if ps -eo comm= | rg -x 'cargo|rustc|wasm-opt|chromium|firefox'; then exit 4; fi
```

Stdout:

```text
online=0-31 cpu=31 age=91 load_one=0.26 siblings=15
sibling_15_busy=0.00
```

## Sole runner invocation

- Timestamp: `2026-09-07T12:05:22.255Z`
- Executor ID: `exec-e4580ee1-5ac0-4d7a-a7b7-6909b9a40ce2`
- Process ID: `50656`
- Command: `bash scripts/run-builtins-current-benchmark.sh`
- Status: completed, exit 0
- Stderr: empty
- Stdout:

```text
/home/bl/misofm/engine-431-full-chain-capture/artifacts/issue431-full-chain/builtins-benchmark.jsonl
```
