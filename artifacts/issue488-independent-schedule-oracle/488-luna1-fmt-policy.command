PATH=/home/bl/.cargo/bin:$PATH cargo fmt --all -- --check
git diff --check
PATH=/home/bl/.cargo/bin:$PATH bash scripts/check-workspace-policy.sh
