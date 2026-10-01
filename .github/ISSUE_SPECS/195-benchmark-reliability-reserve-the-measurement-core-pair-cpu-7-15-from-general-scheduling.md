Owner-approved plan (2026-08-27) to stop the loadavg-refusal serialization tax: the standing fixture measures on cpu 15 (SMT sibling cpu 7 must be idle), but nothing prevents builds, agents, and unrelated processes from landing on that pair, so the runners spend hours refusing on the global-loadavg precondition.

## Tier 1 — runtime cgroup cpuset (no reboot; EXECUTING NOW)

Restrict the default systemd slices so ordinary processes cannot touch the bench pair:

```
systemctl set-property --runtime system.slice AllowedCPUs=0-6,8-14
systemctl set-property --runtime user.slice   AllowedCPUs=0-6,8-14
```

Because cgroup cpusets override taskset, the CPU-exclusive wrapper (the flock that already serializes all heavy work) is amended to run its command in a transient scope with `AllowedCPUs=0-15`: mutex-held work (builds AND benchmarks) keeps all cores — safe because it is serialized by construction — while everything outside the mutex is confined to the 14 general cores. Bench runners keep their own taskset to 15 and all existing preconditions (clock-drift refusal stays: shared L3 / memory bandwidth / package-power boost still leak from busy cores).

To persist across reboots, drop the same properties into `/etc/systemd/system.conf.d/` or unit drop-ins (done together with tier 2).

Follow-up (separate ceremony): relax the runners' global loadavg-0.50 precondition to a core-local check (runnable tasks on cpu 15), since global load can no longer perturb the pinned core.

## Tier 2 — kernel isolation (gold standard; NEXT REBOOT)

Kernel cmdline: `isolcpus=7,15 nohz_full=7,15 rcu_nocbs=7,15` — removes the pair from the scheduler and evicts kernel threads and timer ticks. The cpu-exclusive scope keeps working unchanged (affinity into isolated cores is exactly how they are used).

## Non-goals

- Governor stays `powersave`: every sealed baseline was captured under it; switching re-baselines the record family and is a separate owner decision at a round boundary.
