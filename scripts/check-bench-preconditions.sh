#!/usr/bin/env bash
# Measurement-admissibility preconditions for the one-shot benchmark runners (#144 item 13,
# delivered by #163 phase 0a).
#
# # Why this file exists
#
# Before it, `scripts/run-console-benchmark.sh` controlled nothing about the machine it measured.
# It read `/proc/loadavg` once and copied the numbers into a record field literally spelled
# `"not-controlled; pre-run loadavg ..."`. That is a *note*, and a note is not a precondition: it
# makes an inadmissible measurement fully admissible and merely well documented. The sealed
# issue-149 records all carry it, and there is no way to tell from any of them whether the host was
# quiet, whether the process stayed on one core, or whether another tenant was on the sibling
# hyperthread for half the run.
#
# The rule this file implements instead: a run that cannot be controlled **refuses**, and names
# which control it could not obtain. An operator on a machine where control is genuinely
# impossible -- a shared CI box, a container without `taskset`, a kernel without cpufreq -- sets
# `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`, and the run then proceeds *and says so in every record
# it writes*, so a controlled and an uncontrolled number can never be silently compared.
#
# # Why the predicates live here and not inline in the runner
#
# A precondition that has never been shown to reject anything is decoration, exactly like a
# validator that has never been shown to reject anything. Every decision below is a pure function
# of text -- `/proc/loadavg` contents, `/sys/.../online` contents, two `/proc/stat` samples -- so
# the self-test at the bottom can drive each one to both verdicts without a machine in a
# particular state. The runner reads the real files and calls these.
#
# Sourced, this file defines functions and nothing else. Executed, it runs the self-test.
set -euo pipefail

# ---------------------------------------------------------------------------------------------
# The frozen ceilings.
#
# Deliberately constants and not environment variables. A ceiling an operator can raise is a
# ceiling that gets raised on the day the measurement matters, and the escape hatch already exists
# for the honest case -- it just makes the record say `uncontrolled`, which is the point.
# ---------------------------------------------------------------------------------------------

# One-minute load average the host must be under. Absolute, not per-core: the workload is pinned to
# one core, and a runnable task anywhere on the machine can still take the shared last-level cache,
# the memory controller and the package's turbo budget.
readonly MISO_ENGINE_BENCH_LOADAVG_CEILING=0.50
# Seconds a freshly linked binary must age before it is timed. A release build saturates every core
# and leaves the package hot and the governor ramped; the first measured block after one is
# measuring the build.
readonly MISO_ENGINE_BENCH_COOLDOWN_SECONDS=60
# Percentage of an interval an SMT sibling of the pinned core may be busy.
readonly MISO_ENGINE_BENCH_SIBLING_BUSY_CEILING=5
# Interval, in seconds, over which sibling busyness is sampled.
readonly MISO_ENGINE_BENCH_SIBLING_SAMPLE_SECONDS=0.2

# ---------------------------------------------------------------------------------------------
# Pure predicates.
# ---------------------------------------------------------------------------------------------

# The one-minute load average from `/proc/loadavg` text.
bench_loadavg_one_minute() {
    local text=$1 value
    value=$(awk '{print $1}' <<<"$text")
    [[ "$value" =~ ^[0-9]+(\.[0-9]+)?$ ]] || return 1
    printf '%s' "$value"
}

# True when `value <= ceiling`. Both are decimal; bash cannot compare them, so awk does.
bench_within_ceiling() {
    local value=$1 ceiling=$2
    [[ "$value" =~ ^[0-9]+(\.[0-9]+)?$ ]] || return 1
    awk -v value="$value" -v ceiling="$ceiling" 'BEGIN { exit !(value <= ceiling) }'
}

# Expands a Linux CPU list (`0-3,8,10-11`) to a space-separated ascending list.
bench_expand_cpu_list() {
    local text=$1 part low high cpu
    local -a out=() parts=()
    [[ "$text" =~ ^[0-9]+([-,][0-9]+)*$ ]] || return 1
    # Split on commas without touching IFS: `${out[*]}` below joins on IFS, so a `local IFS=,`
    # here would silently make every caller receive a comma-joined list instead of a space-joined
    # one -- which is exactly what the first version of this function did.
    read -ra parts <<<"${text//,/ }"
    for part in "${parts[@]}"; do
        if [[ "$part" == *-* ]]; then
            low=${part%%-*}
            high=${part##*-}
            (( low <= high )) || return 1
            for (( cpu = low; cpu <= high; cpu++ )); do out+=("$cpu"); done
        else
            out+=("$part")
        fi
    done
    printf '%s' "${out[*]}"
}

# The highest CPU in an online-CPU list.
#
# Highest rather than lowest on purpose: CPU 0 is where a default Linux install lands timer
# interrupts, RCU callbacks and unbound workqueues, so it is the one core guaranteed to have
# another tenant.
bench_highest_cpu() {
    local expanded
    expanded=$(bench_expand_cpu_list "$1") || return 1
    printf '%s' "${expanded##* }"
}

# The busy percentage of one CPU between two `/proc/stat` snapshots.
#
# `/proc/stat`'s per-CPU line is `cpuN user nice system idle iowait irq softirq steal ...`. Busy is
# everything that is not `idle` or `iowait`; `steal` counts as busy because a hypervisor taking the
# core is exactly the contention this check exists to find.
bench_cpu_busy_percent() {
    local before=$1 after=$2 cpu=$3
    awk -v cpu="cpu$cpu" '
        function total(  i, sum) { sum = 0; for (i = 2; i <= NF; i++) sum += $i; return sum }
        FNR == NR { if ($1 == cpu) { first_total = total(); first_idle = $5 + $6; seen_first = 1 } next }
        $1 == cpu { second_total = total(); second_idle = $5 + $6; seen_second = 1 }
        END {
            if (!seen_first || !seen_second) exit 1
            delta_total = second_total - first_total
            delta_idle = second_idle - first_idle
            if (delta_total <= 0) exit 1
            printf "%.2f", 100 * (delta_total - delta_idle) / delta_total
        }
    ' <(printf '%s\n' "$before") <(printf '%s\n' "$after")
}

# The SMT siblings of `cpu`, excluding `cpu` itself, from a `thread_siblings_list` text.
bench_other_siblings() {
    local cpu=$1 text=$2 expanded sibling out=()
    expanded=$(bench_expand_cpu_list "$text") || return 1
    for sibling in $expanded; do
        [[ "$sibling" == "$cpu" ]] || out+=("$sibling")
    done
    printf '%s' "${out[*]-}"
}

# ---------------------------------------------------------------------------------------------
# Self-test: every predicate driven to both verdicts.
# ---------------------------------------------------------------------------------------------

bench_preconditions_self_test() {
    local failures=0
    expect() {
        local label=$1 condition=$2
        if [[ "$condition" != ok ]]; then
            printf 'bench precondition self-test FAILED: %s\n' "$label" >&2
            failures=$((failures + 1))
        fi
    }
    yes_no() { if "$@" >/dev/null 2>&1; then printf ok; else printf no; fi; }
    equals() { if [[ "$1" == "$2" ]]; then printf ok; else printf 'no (%s != %s)' "$1" "$2"; fi; }

    expect 'loadavg parses the one-minute column' \
        "$(equals "$(bench_loadavg_one_minute '0.07 0.31 0.42 1/512 90210')" 0.07)"
    expect 'loadavg refuses text that is not a load average' \
        "$(yes_no bench_loadavg_one_minute 'not a loadavg' | sed 's/^ok$/no/;s/^no$/ok/')"

    expect 'a quiet host is admitted' "$(yes_no bench_within_ceiling 0.07 0.50)"
    expect 'the ceiling itself is admitted' "$(yes_no bench_within_ceiling 0.50 0.50)"
    expect 'a busy host is refused' \
        "$(yes_no bench_within_ceiling 0.51 0.50 | sed 's/^ok$/no/;s/^no$/ok/')"
    expect 'a very busy host is refused' \
        "$(yes_no bench_within_ceiling 12.3 0.50 | sed 's/^ok$/no/;s/^no$/ok/')"
    expect 'a non-numeric load is refused rather than silently admitted' \
        "$(yes_no bench_within_ceiling 'nan' 0.50 | sed 's/^ok$/no/;s/^no$/ok/')"

    expect 'a single cpu list expands' "$(equals "$(bench_expand_cpu_list '3')" '3')"
    expect 'a range expands' "$(equals "$(bench_expand_cpu_list '0-3')" '0 1 2 3')"
    expect 'a mixed list expands' \
        "$(equals "$(bench_expand_cpu_list '0-1,4,6-7')" '0 1 4 6 7')"
    expect 'a malformed list is refused' \
        "$(yes_no bench_expand_cpu_list 'cpu0-3' | sed 's/^ok$/no/;s/^no$/ok/')"
    expect 'an inverted range is refused' \
        "$(yes_no bench_expand_cpu_list '7-3' | sed 's/^ok$/no/;s/^no$/ok/')"

    expect 'the highest cpu of a range' "$(equals "$(bench_highest_cpu '0-15')" '15')"
    expect 'the highest cpu of a mixed list' "$(equals "$(bench_highest_cpu '0-3,12')" '12')"

    local quiet_before quiet_after busy_after
    quiet_before='cpu  1 0 1 100 0 0 0 0 0 0
cpu7 10 0 5 1000 0 0 0 0 0 0'
    quiet_after='cpu  1 0 1 200 0 0 0 0 0 0
cpu7 10 0 5 1100 0 0 0 0 0 0'
    busy_after='cpu  1 0 1 200 0 0 0 0 0 0
cpu7 100 0 5 1000 0 0 0 0 0 0'
    expect 'an idle sibling reports zero busy' \
        "$(equals "$(bench_cpu_busy_percent "$quiet_before" "$quiet_after" 7)" '0.00')"
    expect 'a saturated sibling reports one hundred' \
        "$(equals "$(bench_cpu_busy_percent "$quiet_before" "$busy_after" 7)" '100.00')"
    expect 'an idle sibling passes the busy ceiling' \
        "$(yes_no bench_within_ceiling \
            "$(bench_cpu_busy_percent "$quiet_before" "$quiet_after" 7)" 5)"
    expect 'a saturated sibling fails the busy ceiling' \
        "$(yes_no bench_within_ceiling \
            "$(bench_cpu_busy_percent "$quiet_before" "$busy_after" 7)" 5 |
            sed 's/^ok$/no/;s/^no$/ok/')"
    expect 'stolen time counts as busy' \
        "$(equals "$(bench_cpu_busy_percent \
            'cpu3 0 0 0 1000 0 0 0 0' 'cpu3 0 0 0 1050 0 0 0 50' 3)" '50.00')"
    expect 'a cpu absent from the snapshots is refused' \
        "$(yes_no bench_cpu_busy_percent "$quiet_before" "$quiet_after" 99 |
            sed 's/^ok$/no/;s/^no$/ok/')"
    expect 'two identical snapshots are refused rather than reported as idle' \
        "$(yes_no bench_cpu_busy_percent "$quiet_before" "$quiet_before" 7 |
            sed 's/^ok$/no/;s/^no$/ok/')"

    expect 'a hyperthreaded core names its sibling' \
        "$(equals "$(bench_other_siblings 7 '7,23')" '23')"
    expect 'a core with no sibling names none' "$(equals "$(bench_other_siblings 7 '7')" '')"
    expect 'a sibling range excludes the pinned cpu' \
        "$(equals "$(bench_other_siblings 6 '6-7')" '7')"

    if [[ "$failures" != 0 ]]; then
        printf 'bench preconditions: %s FAILED case(s)\n' "$failures" >&2
        return 1
    fi
    printf 'bench preconditions: ok (loadavg ceiling %s, cooldown %ss, sibling busy ceiling %s%%)\n' \
        "$MISO_ENGINE_BENCH_LOADAVG_CEILING" "$MISO_ENGINE_BENCH_COOLDOWN_SECONDS" \
        "$MISO_ENGINE_BENCH_SIBLING_BUSY_CEILING"
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    [[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
    bench_preconditions_self_test
fi
