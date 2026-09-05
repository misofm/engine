#!/usr/bin/env bash
set -euo pipefail
root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"
temp="$(mktemp -d)"; trap 'rm -rf -- "$temp"' EXIT
fixture="$temp/fixture"
mkdir -p "$fixture/dsp-research/listening"
notes=(filters dynamics loudness oversampling true-peak delay nonlinear-antialiasing multirate-crossovers simd-numerics console-daw-architecture)
headings=("Scope and engineering question" "Adopted decisions" "Definitions and assumptions" "Algorithm and equations" "Coefficients and update rules" "Numerical and stability limits" "Latency and tail" "Units, mappings, automation and smoothing" "Denormal, signed-zero and NaN policy" "Fixtures" "Objective tests and tolerances" "Benchmark plan" "Listening protocol or evidence" "Primary and official sources" "Rejected alternatives and tradeoffs" "Known gaps and follow-up")
write_note() {
    local file=$1 heading
    : >"$file"
    for heading in "${headings[@]}"; do
        printf '## %s\n' "$heading" >>"$file"
        if [[ "$heading" == 'Primary and official sources' ]]; then
            printf -- '- [DUMMY-A]\n- [DUMMY-B]\n' >>"$file"
        else
            printf 'structural fixture content\n' >>"$file"
        fi
    done
}
populate() {
    rm -rf "$fixture"; mkdir -p "$fixture/dsp-research/listening"
    for note in "${notes[@]}"; do write_note "$fixture/dsp-research/$note.md"; done
    printf 'DiGiCo SSL Lawo Avid Logic\n' >>"$fixture/dsp-research/console-daw-architecture.md"
    printf 'support\n' >"$fixture/dsp-research/README.md"
    printf 'support\n' >"$fixture/dsp-research/CITATION_POLICY.md"
    write_note "$fixture/dsp-research/NOTE_TEMPLATE.md"
    printf -- '- `[DUMMY-A]`\n- `[DUMMY-B]`\n- `[OUTSIDE-KEY]`\n' >"$fixture/dsp-research/BIBLIOGRAPHY.md"
    printf 'support\n' >"$fixture/dsp-research/listening/TEMPLATE.md"
    printf 'Evidence kind: synthetic format example\nSound-quality claim: none\nSYNTHETIC-NOT-A-HUMAN\n' >"$fixture/dsp-research/listening/FORMAT_EXAMPLE.md"
}
check() { bash "$root/scripts/check-dsp-research.sh" "$1"; }
expect_failure() {
    local label=$1 expected=$2 output rc
    output="$(check "$fixture" 2>&1)" && rc=0 || rc=$?
    [[ "$rc" -ne 0 && "$output" == *"$expected"* ]] || {
        printf 'research mutation escaped (%s): %s\n' "$label" "$output" >&2; exit 1;
    }
}
populate; check "$fixture" >/dev/null
rm "$fixture/dsp-research/filters.md"; expect_failure missing-note 'missing research note'
populate; rm "$fixture/dsp-research/listening/TEMPLATE.md"; expect_failure missing-support 'missing research artifact'
populate; sed -i '/^## Fixtures$/{n;d;}' "$fixture/dsp-research/filters.md"; expect_failure empty-section 'empty heading Fixtures'
populate; sed -i '/^- \[DUMMY-B\]$/d' "$fixture/dsp-research/filters.md"; expect_failure insufficient-primary 'fewer than two sources'
populate; sed -i 's/DUMMY-B/DUMMY-A/' "$fixture/dsp-research/filters.md"; expect_failure duplicate-primary 'fewer than two sources'
populate; printf '\n[UNRESOLVED-OUTSIDE]\n' >>"$fixture/dsp-research/filters.md"; expect_failure unresolved-outside 'bibliography key UNRESOLVED-OUTSIDE'
populate; sed -i '/Sound-quality claim: none/d' "$fixture/dsp-research/listening/FORMAT_EXAMPLE.md"; expect_failure listening 'listening literal Sound-quality claim: none'
populate

real_rg="$(command -v rg)"
mkdir -p "$temp/rg-fault"
cat >"$temp/rg-fault/rg" <<EOF
#!/usr/bin/env bash
matched=0
case "\$RESEARCH_SELECTOR" in
    PRIMARY_RG) [[ "\$*" == *"/filters-primary"* ]] && matched=1 ;;
    WHOLE_RG) [[ "\$*" == *"-o"* && "\$*" == *"dsp-research/filters.md"* ]] && matched=1 ;;
    *) [[ "\$*" == *"\$RESEARCH_SELECTOR"* ]] && matched=1 ;;
esac
if [[ "\$matched" == 1 ]]; then
    [[ "\$RESEARCH_MODE" == partial ]] && printf '%s\n' "\$RESEARCH_PARTIAL"
    exit 7
fi
exec "$real_rg" "\$@"
EOF
chmod +x "$temp/rg-fault/rg"
assert_rg_fault() {
    local checker=$1 selector=$2 partial=$3 diagnostic=$4 mode=$5 output rc
    output="$(RESEARCH_SELECTOR="$selector" RESEARCH_PARTIAL="$partial" RESEARCH_MODE="$mode" PATH="$temp/rg-fault:$PATH" bash "$checker" "$fixture" 2>&1)" && rc=0 || rc=$?
    [[ "$rc" -ne 0 && "$output" == *"$diagnostic"* ]] || {
        printf 'research selective fault escaped (%s/%s): %s\n' "$selector" "$mode" "$output" >&2; return 1;
    }
    [[ "$mode" != partial || "$output" == *"$partial"* ]] || return 1
}
while IFS='|' read -r selector partial diagnostic; do
    assert_rg_fault "$root/scripts/check-dsp-research.sh" "$selector" "$partial" "$diagnostic" error
    assert_rg_fault "$root/scripts/check-dsp-research.sh" "$selector" "$partial" "$diagnostic" partial
done <<'EOF'
^## Scope and engineering question$|1:## Scope and engineering question|heading Scope and engineering question in dsp-research/filters.md search failed (rg exit 7)
PRIMARY_RG|[DUMMY-A]|Primary key extraction failed in dsp-research/filters.md (rg exit 7)
WHOLE_RG|[DUMMY-A]|whole-note key extraction failed in dsp-research/filters.md (rg exit 7)
- `[DUMMY-A]`|1:- `[DUMMY-A]`|bibliography key DUMMY-A search failed (rg exit 7)
DiGiCo|1:DiGiCo|console/DAW literal DiGiCo search failed (rg exit 7)
dsp-research/NOTE_TEMPLATE.md|1:## Scope and engineering question|note template heading Scope and engineering question search failed (rg exit 7)
Sound-quality claim: none|1:Sound-quality claim: none|listening literal Sound-quality claim: none search failed (rg exit 7)
EOF

real_awk="$(command -v awk)"
mkdir -p "$temp/awk-fault"
cat >"$temp/awk-fault/awk" <<EOF
#!/usr/bin/env bash
matched=0
case "\$RESEARCH_AWK_SELECTOR" in
    SECTION_AWK) [[ "\$*" == *"-v heading="* ]] && matched=1 ;;
    PRIMARY_AWK) [[ "\$*" == *"Primary and official sources"* && "\$*" != *"-v heading="* ]] && matched=1 ;;
esac
if [[ "\$matched" == 1 ]]; then printf 'awk partial\n'; exit 7; fi
exec "$real_awk" "\$@"
EOF
chmod +x "$temp/awk-fault/awk"
for row in 'SECTION_AWK|section content check failed' 'PRIMARY_AWK|Primary section extraction failed'; do
    selector=${row%%|*}; diagnostic=${row#*|}
    output="$(RESEARCH_AWK_SELECTOR="$selector" PATH="$temp/awk-fault:$PATH" check "$fixture" 2>&1)" && rc=0 || rc=$?
    [[ "$rc" -ne 0 && "$output" == *'awk partial'* && "$output" == *"$diagnostic"* ]] || {
        printf 'research awk fault escaped (%s): %s\n' "$selector" "$output" >&2; exit 1;
    }
done

# Same-assertion actual counter for the whole-note producer.
assert_rg_fault "$root/scripts/check-dsp-research.sh" WHOLE_RG '[DUMMY-A]' 'whole-note key extraction failed in dsp-research/filters.md (rg exit 7)' partial
mkdir -p "$temp/mutant-scripts/lib"
cp "$root/scripts/check-dsp-research.sh" "$temp/mutant-scripts/"
cp "$root/scripts/lib/gate.sh" "$temp/mutant-scripts/lib/"
mutant="$temp/mutant-scripts/check-dsp-research.sh"
[[ "$(grep -Fc '[[ "$rc" == 1 ]] || fail "whole-note key extraction failed' "$mutant")" == 1 ]] || exit 1
sed -i '/whole-note key extraction failed/ s/\[\[ "$rc" == 1 \]\]/[[ "$rc" == 1 || "$rc" == 7 ]]/' "$mutant"
grep -Fq '[[ "$rc" == 1 || "$rc" == 7 ]]' "$mutant" || exit 1
if counter_output="$(assert_rg_fault "$mutant" WHOLE_RG '[DUMMY-A]' 'whole-note key extraction failed in dsp-research/filters.md (rg exit 7)' partial 2>&1)"; then
    printf 'whole-note same-assertion counter-mutant escaped\n' >&2; exit 1
fi

# Same-assertion actual counter for the final listening consumer.
cp "$root/scripts/check-dsp-research.sh" "$mutant"
[[ "$(grep -Fc 'gate_scan_required "listening literal $literal"' "$mutant")" == 1 ]] || exit 1
sed -i '/gate_scan_required "listening literal \$literal"/ s/|| exit \$?/|| true/' "$mutant"
grep -F 'gate_scan_required "listening literal $literal"' "$mutant" | grep -Fq '|| true' || exit 1
if counter_output="$(assert_rg_fault "$mutant" 'Sound-quality claim: none' '1:Sound-quality claim: none' 'listening literal Sound-quality claim: none search failed (rg exit 7)' partial 2>&1)"; then
    printf 'listening same-assertion counter-mutant escaped\n' >&2; exit 1
fi
check "$fixture" >/dev/null
printf 'dsp research mutations: ok\n'
