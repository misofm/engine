# Issue map: submix strips, sends and VCA groups

Filed on 2026-10-02 by *Record the submix, send and VCA ruling* (#1197), under decision 13
(`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`). The left column is the plan's slice
label, which `DESIGN.md`, the `VERIFY`, `REVISION` and `APPLIED` documents in this folder, and the
plan files they cite as `issues/<label>-*.md`, use. The plan's `issues/` folder is not committed
except for the unfiled VCA drafts; each filed spec is `.github/ISSUE_SPECS/<number>-*.md`, and its
GitHub issue body matches it. Five specs (#1203, #1205, #1208, #1210, #1212) were renamed in
#1197's attempt 2 to the slug of their GitHub titles; their plan files keep the plan's names.
A closed issue's spec leaves `.github/ISSUE_SPECS/` at the batch after it closes; the rows of
#1197 and #1198 (closed by PR #1230) therefore link the spec at `5abde384`, and the rows of
#1199-#1205 (closed by PR #1231) link it at `b6b1bdf4`, where each last stood.

The umbrella's app-facing documents -- `APP-SDK.md`, the session migration script and, later,
`APP-LIVE.md` -- live in [`docs/handoffs/submix-strips-and-sends/`](../submix-strips-and-sends/),
the name decision D6 of #1205 froze; this folder keeps the design record, the plan, the drafts and
the verdicts (`verdicts/`).

| Slice | Issue | Title | Spec | Plan file |
|---|---|---|---|---|
| umbrella | #1196 | Submix strips and live aux sends | `.github/ISSUE_SPECS/1196-submix-strips-and-live-aux-sends.md` | (written from `DESIGN.md`; no plan file) |
| 00 | #1197 | Record the submix, send and VCA ruling | [`1197-record-the-submix-send-and-vca-ruling.md`](https://github.com/misofm/engine/blob/5abde3840f1af276760584b84cfb40fde64554a4/.github/ISSUE_SPECS/1197-record-the-submix-send-and-vca-ruling.md) (retired, closed) | `00-record-the-submix-send-and-vca-ruling.md` |
| 01 | #1198 | Iterate session strips, not tracks, wherever strip semantics apply | [`1198-iterate-session-strips-not-tracks.md`](https://github.com/misofm/engine/blob/5abde3840f1af276760584b84cfb40fde64554a4/.github/ISSUE_SPECS/1198-iterate-session-strips-not-tracks.md) (retired, closed) | `01-iterate-session-strips-not-tracks.md` |
| 02 | #1199 | Declare the submix strip in the session grammar and wire | [`1199-declare-the-submix-strip-in-the-session-grammar-and-wire.md`](https://github.com/misofm/engine/blob/b6b1bdf4bbc2d2640335457e3dd47339516a0501/.github/ISSUE_SPECS/1199-declare-the-submix-strip-in-the-session-grammar-and-wire.md) (retired, closed) | `02-declare-the-submix-strip-in-the-session-grammar-and-wire.md` |
| 03 | #1200 | Render a submix strip on its summed input | [`1200-render-a-submix-strip-on-its-summed-input.md`](https://github.com/misofm/engine/blob/b6b1bdf4bbc2d2640335457e3dd47339516a0501/.github/ISSUE_SPECS/1200-render-a-submix-strip-on-its-summed-input.md) (retired, closed) | `03-render-a-submix-strip-on-its-summed-input.md` |
| 04 | #1201 | Delay a submix strip's summed input | [`1201-delay-a-submix-strips-summed-input.md`](https://github.com/misofm/engine/blob/b6b1bdf4bbc2d2640335457e3dd47339516a0501/.github/ISSUE_SPECS/1201-delay-a-submix-strips-summed-input.md) (retired, closed) | `04-delay-a-submix-strips-summed-input.md` |
| 05 | #1202 | Carry every console slot on every submix strip | [`1202-carry-every-console-slot-on-every-submix-strip.md`](https://github.com/misofm/engine/blob/b6b1bdf4bbc2d2640335457e3dd47339516a0501/.github/ISSUE_SPECS/1202-carry-every-console-slot-on-every-submix-strip.md) (retired, closed) | `05-carry-every-console-slot-on-every-submix-strip.md` |
| 06 | #1203 | Tap a submix strip at any of the seven send points | [`1203-tap-a-submix-strip-at-any-of-the-seven-send-points.md`](https://github.com/misofm/engine/blob/b6b1bdf4bbc2d2640335457e3dd47339516a0501/.github/ISSUE_SPECS/1203-tap-a-submix-strip-at-any-of-the-seven-send-points.md) (retired, closed) | `06-tap-a-submix-strip-at-any-send-point.md` |
| 07 | #1204 | Address submix strips in session edits | [`1204-address-submix-strips-in-session-edits.md`](https://github.com/misofm/engine/blob/b6b1bdf4bbc2d2640335457e3dd47339516a0501/.github/ISSUE_SPECS/1204-address-submix-strips-in-session-edits.md) (retired, closed) | `07-address-submix-strips-in-session-edits.md` |
| 08 | #1205 | Build submix strips and bus taps in the SDK and teach agents to author them | [`1205-build-submix-strips-and-bus-taps-in-the-sdk-and-teach-agents-to-author-them.md`](https://github.com/misofm/engine/blob/b6b1bdf4bbc2d2640335457e3dd47339516a0501/.github/ISSUE_SPECS/1205-build-submix-strips-and-bus-taps-in-the-sdk-and-teach-agents-to-author-them.md) (retired, closed) | `08-build-submix-strips-in-the-sdk-and-teach-agents-to-author-them.md` |
| 09 | #1206 | Count and cap submix strips in host preparation and the C ABI | `.github/ISSUE_SPECS/1206-count-and-cap-submix-strips-in-host-preparation-and-the-c-abi.md` | `09-count-and-cap-submix-strips-in-host-preparation-and-the-c-abi.md` |
| 10 | #1207 | List every strip in the live-control handles and file bus effects in the browser | `.github/ISSUE_SPECS/1207-list-every-strip-in-the-live-control-handles-and-file-bus-effects-in-the-browser.md` | `10-list-every-strip-in-the-live-control-handles-and-file-bus-effects-in-the-browser.md` |
| 11 | #1208 | Meter any boundary of a submix strip and designate a master strip in host-core | `.github/ISSUE_SPECS/1208-meter-any-boundary-of-a-submix-strip-and-designate-a-master-strip-in-host-core.md` | `11-meter-and-designate-submix-strips-in-host-core.md` |
| 12 | #1209 | Carry submix strips in the browser meter frame | `.github/ISSUE_SPECS/1209-carry-submix-strips-in-the-browser-meter-frame.md` | `12-carry-submix-strips-in-the-browser-meter-frame.md` |
| 13 | #1210 | Name submix strips in the browser session map and the SDK measurement | `.github/ISSUE_SPECS/1210-name-submix-strips-in-the-browser-session-map-and-the-sdk-measurement.md` | `13-name-submix-strips-in-the-browser-session-map-and-sdk-measurement.md` |
| 14 | #1211 | Give every strip one mute owner and live-control producers in host-core | `.github/ISSUE_SPECS/1211-give-every-strip-one-mute-owner-and-live-control-producers-in-host-core.md` | `14-give-every-strip-one-mute-owner-and-live-control-producers-in-host-core.md` |
| 15 | #1212 | Add the notSoloable command reason to every vocabulary spelling | `.github/ISSUE_SPECS/1212-add-the-notsoloable-command-reason-to-every-vocabulary-spelling.md` | `15-add-the-not-soloable-command-reason.md` |
| 16 | #1213 | Address submix strips in browser live commands | `.github/ISSUE_SPECS/1213-address-submix-strips-in-browser-live-commands.md` | `16-address-submix-strips-in-browser-live-commands.md` |
| 17 | #1214 | Drive submix strips from the SDK live controls | `.github/ISSUE_SPECS/1214-drive-submix-strips-from-the-sdk-live-controls.md` | `17-drive-submix-strips-from-the-sdk-live-controls.md` |
| 18a | #1215 | Gate every route's coefficients through one function | `.github/ISSUE_SPECS/1215-gate-every-routes-coefficients-through-one-function.md` | `18a-gate-every-routes-coefficients-through-one-function.md` |
| 18b | #1216 | Mute a route in the session | `.github/ISSUE_SPECS/1216-mute-a-route-in-the-session.md` | `18b-mute-a-route-in-the-session.md` |
| 19 | #1217 | Skip an inactive route in its destination's sum | `.github/ISSUE_SPECS/1217-skip-an-inactive-route-in-its-destinations-sum.md` | `19-skip-an-inactive-route-in-its-destinations-sum.md` |
| 20 | #1218 | Let a route into a submix follow its source strip's mute in the session | `.github/ISSUE_SPECS/1218-let-a-route-into-a-submix-follow-its-source-strips-mute-in-the-session.md` | `20-let-a-route-into-a-submix-follow-its-source-strips-mute-in-the-session.md` |
| 21 | #1219 | Ramp a send's coefficients with the indexed ramp kernel | `.github/ISSUE_SPECS/1219-ramp-a-sends-coefficients-with-the-indexed-ramp-kernel.md` | `21-ramp-a-sends-coefficients-with-the-indexed-ramp-kernel.md` |
| 22 | #1220 | Ramp live send coefficients on the render plane | `.github/ISSUE_SPECS/1220-ramp-live-send-coefficients-on-the-render-plane.md` | `22-ramp-live-send-coefficients-on-the-render-plane.md` |
| 23 | #1221 | Produce live send records from host-core | `.github/ISSUE_SPECS/1221-produce-live-send-records-from-host-core.md` | `23-produce-live-send-records-from-host-core.md` |
| 24 | #1222 | Admit live send commands in the browser | `.github/ISSUE_SPECS/1222-admit-live-send-commands-in-the-browser.md` | `24-admit-live-send-commands-in-the-browser.md` |
| 25 | #1223 | Enumerate sends and drive them from the SDK | `.github/ISSUE_SPECS/1223-enumerate-sends-and-drive-them-from-the-sdk.md` | `25-enumerate-sends-and-drive-them-from-the-sdk.md` |
| 26 | #1224 | Let a send follow its source strip's mute live in the browser | `.github/ISSUE_SPECS/1224-let-a-send-follow-its-source-strips-mute-live-in-the-browser.md` | `26-let-a-send-follow-its-source-strips-mute-live-in-the-browser.md` |
| 27 | #1225 | Deliver value-only send and submix-strip edits to the running C ABI plan | `.github/ISSUE_SPECS/1225-deliver-value-only-send-and-submix-strip-edits-to-the-running-c-abi-plan.md` | `27-deliver-value-only-send-and-submix-strip-edits-to-the-running-c-abi-plan.md` |
| 28 | #1226 | Let C ABI sends follow their source strip's mute live | `.github/ISSUE_SPECS/1226-let-c-abi-sends-follow-their-source-strips-mute-live.md` | `28-let-c-abi-sends-follow-their-source-strips-mute-live.md` |
| BM1 | #1227 | Add a bus-and-send row to the native console benchmark | `.github/ISSUE_SPECS/1227-add-a-bus-and-send-row-to-the-native-console-benchmark.md` | `BM1-add-a-bus-and-send-row-to-the-native-console-benchmark.md` |
| BM2 | #1228 | Add the bus-and-send session to the browser mixing benchmark | `.github/ISSUE_SPECS/1228-add-the-bus-and-send-session-to-the-browser-mixing-benchmark.md` | `BM2-add-the-bus-and-send-session-to-the-browser-mixing-benchmark.md` |
| BM3 | #1229 | Record the bus-and-send baseline and its route-work profile | `.github/ISSUE_SPECS/1229-record-the-bus-and-send-baseline-and-its-route-work-profile.md` | `BM3-record-the-bus-and-send-baseline-and-its-route-work-profile.md` |

BM1-BM3 (#1227-#1229) are standalone successors, not children of #1196.

## Not filed

| Draft | Title | Where |
|---|---|---|
| V0 | VCA groups | `issues/V0-vca-groups-umbrella.md` (this folder) |
| V1 | Declare VCA groups in the session | `issues/V1-declare-vca-groups-in-the-session.md` (this folder) |
| V2 | Apply VCA offsets and mutes at preparation | `issues/V2-apply-vca-offsets-and-mutes-at-preparation.md` (this folder) |
| V3 | Ride VCA groups live in the browser | `issues/V3-ride-vca-groups-live-in-the-browser.md` (this folder) |
| V4 | Enumerate VCA groups and drive them from the SDK | `issues/V4-enumerate-vca-groups-and-drive-them-from-the-sdk.md` (this folder) |
| V5 | Deliver value-only VCA edits to the running C ABI plan | `issues/V5-deliver-value-only-vca-edits-to-the-running-c-abi-plan.md` (this folder) |

The root files the VCA umbrella when batch K3 closes (#1224's Sol PASS and the K3 push), and
re-verifies the drafts' anchors then. V4 closes the VCA batch V1-V4, so its draft carries the
deliverable that removes the decision-13 qualifier from `AGENTS.md`'s VCA sentence (#1197 D5); the
filed V4 must keep it. Deferred item O11 waits on owner question Q2; nothing is filed
for it.

The probe sources that `DESIGN.md` 3.2 and `VERIFY-1.md` name under `scratchpad/` were never committed.
