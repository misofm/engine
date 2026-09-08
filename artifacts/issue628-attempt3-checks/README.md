# Issue 628 final attempt 3

Stopped at the required lease preflight. The retained lease owner file is not
the exact owner/scope requested for this execution: it contains an additional
`attempt=3-final` line and uses
`scope=sdk-installed-state-preflight,sdk-check,matrix-check,cargo-fmt,diff-hygiene,workspace-policy,effect-runtime-policy`.
The requested exact scope was
`preflight, sdk-check,matrix-check,cargo-fmt,diff-hygiene,workspace-policy,effect-runtime-policy`.

No SDK manifest/node_modules/output preflight, SDK check, matrix check, fmt,
diff, workspace, or effect-runtime command ran. No install or retry was made.
