#!/usr/bin/env bash
set -euo pipefail
root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"
temp="$(mktemp -d)"; trap 'rm -rf -- "$temp"' EXIT
cp -R "$root/dsp-research" "$temp/"
check() { bash "$root/scripts/check-dsp-research.sh" "$1"; }
expect_failure() { local label=$1; if check "$temp" >/dev/null 2>&1; then printf 'research mutation escaped: %s\n' "$label" >&2; exit 1; fi; }
check "$temp" >/dev/null
rm "$temp/dsp-research/filters.md"; expect_failure missing-note
cp "$root/dsp-research/filters.md" "$temp/dsp-research/filters.md"
sed -i '/^## Fixtures$/,/^## Objective tests/ { /^## Objective tests/!d; }' "$temp/dsp-research/filters.md"; expect_failure empty-section
cp "$root/dsp-research/filters.md" "$temp/dsp-research/filters.md"
sed -i 's/^\[RBJ-EQ\]/[RBJ-EQ]/' "$temp/dsp-research/filters.md"
printf '\n[RBJ-EQ]\n' >>"$temp/dsp-research/filters.md"; expect_failure duplicate-primary
cp "$root/dsp-research/filters.md" "$temp/dsp-research/filters.md"
printf '\n## Extra\n- [UNRESOLVED-OUTSIDE]\n' >>"$temp/dsp-research/filters.md"; expect_failure unresolved-outside-primary
cp "$root/dsp-research/dsp-research/filters.md" "$temp/dsp-research/filters.md" 2>/dev/null || cp "$root/dsp-research/filters.md" "$temp/dsp-research/filters.md"
sed -i '/Sound-quality claim: none/d' "$temp/dsp-research/listening/FORMAT_EXAMPLE.md"; expect_failure final-listening-literal

# Counter-control: mutate the actual whole-note producer to swallow a real partial-output error.
cp -R "$root/dsp-research" "$temp/mutant-research"
mkdir "$temp/mutant-bin"
real_rg="$(command -v rg)"
cat >"$temp/mutant-bin/rg" <<EOF
#!/usr/bin/env bash
if [[ "\$*" == *'-o \\[[A-Z0-9]'* ]]; then exec "$real_rg" "\$@"; fi
if [[ "\$*" == *'dsp-research/filters.md'* ]]; then "$real_rg" "\$@"; printf 'valid partial output\n' >&2; exit 7; fi
exec "$real_rg" "\$@"
EOF
chmod +x "$temp/mutant-bin/rg"
mutant="$temp/mutant-check.sh"; cp "$root/scripts/check-dsp-research.sh" "$mutant"
sed -i 's/if all_keys="$(rg -o/if all_keys="$(rg -o/; s/ | sort -u)"; then/ | sort -u)" || true; then/' "$mutant"
grep -q 'sort -u)" || true; then' "$mutant" || { printf 'whole-note mutant replacement missing\n' >&2; exit 1; }
if PATH="$temp/mutant-bin:$PATH" bash "$mutant" "$temp" >/dev/null 2>&1; then
    printf 'whole-note producer counter-mutant escaped\n' >&2; exit 1
fi
printf 'dsp research mutations: ok\n'
