# Issue 628 attempt 2 checks

Attempt 2 stopped during the pre-install evidence setup. The wrapper was
already in `sdk/` but addressed the manifests as `sdk/package.json` and
`sdk/package-lock.json`, so the before-hash capture emitted path errors and
produced an empty `00-sdk-manifests.before.sha256`. The wrapper nevertheless
continued and ran the single permitted `npm ci --ignore-scripts`; it returned
status 0 and installed five packages.

Because the required before-hash record was not captured correctly, no SDK
package check or later check was started. No retry or second install is
authorized. The raw npm command, status, and streams, lease records, failed
before-capture path, and clean-status record are preserved. The attempt is
procedurally failed and receives no remaining-check credit.
