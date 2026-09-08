# Issue 628 remaining checks

Execution stopped at the first authorized check. The lease was verified before
the SDK check: `owner.txt` SHA-256 was
`086714a9b12f97dde9a9f4818244e7b3bd9b67c8a7fb33a40feed70f9720ecc8`, with the
exact issue, owner, authorized head, and scope recorded in `01-sdk.lease.*`.

The sole SDK package/generated-surface command was:

`bash scripts/sdk-package.sh check /tmp/issue627-postpin-attempt3-output-20260908`

It returned status 2 because `sdk/node_modules` is missing. No installation was
attempted, and no matrix, formatting, diff-hygiene, workspace-policy, or
effect-runtime command ran. The raw command, status, and streams are retained.
