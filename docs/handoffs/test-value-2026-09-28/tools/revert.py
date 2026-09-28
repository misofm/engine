#!/usr/bin/env python3
"""Re-inject the four recorded historical bugs into a scratch tree (never the repo).

usage: revert.py <tree> <bug> apply|restore
bugs: 966 (graph-compiler bank level check deleted, MUTATIONS 966-M1),
      970 (arm_mono_collapse without gathers_track_input, 970 spec M1 / 966-M970),
      994 (knee_coefficients `if true`, 994-R1/C1),
      1015 (old stationary leg (c) in both stationary lists, 1015-M1).
"""
import os, shutil, sys

tree, bug, action = sys.argv[1], sys.argv[2], sys.argv[3]

EDITS = {
    '966': ('crates/graph-compiler/src/banks.rs', [(
        """    let first_level = member_level(&members[0]);
    if first_level.is_none()
        || members
            .iter()
            .any(|member| member_level(member) != first_level)
    {
        return Ok(None);
    }
    Ok(Some(members))""",
        """    let _ = &member_level;
    Ok(Some(members))""")]),
    '970': ('crates/graph/src/runtime.rs', [(
        """            let armed = identity.banking.gathers_track_input()
                && !tracks.is_empty()""",
        """            let armed = !tracks.is_empty()""")]),
    '994': ('crates/effect-runtime/src/dynamics.rs', [(
        """        if inv_two_knee.is_finite() {
            return (0.5 * knee_db, inv_two_knee);""",
        """        if true {
            return (0.5 * knee_db, inv_two_knee);""")]),
    '1015': ('crates/parametric-eq/src/lib.rs', [
        # re-add the pre-#1015 predicate next to the flush-shaped one
        ("""#[inline(always)]
fn section_state_is_flush_shaped<L: Lane>(section: &Section<L>) -> bool {""",
         """fn lane_is_finite_without_negative_zero<L: Lane>(value: L) -> bool {
    let mut words = [0_u32; MAX_LANES];
    value.store_bits(&mut words[..L::WIDTH]);
    words[..L::WIDTH]
        .iter()
        .all(|word| *word != NEGATIVE_ZERO_BITS && (*word & MAGNITUDE_MASK) < NON_FINITE_MAGNITUDE)
}

fn section_state_is_finite_without_negative_zero<L: Lane>(section: &Section<L>) -> bool {
    lane_is_finite_without_negative_zero::<L>(section.state.ic1)
        && lane_is_finite_without_negative_zero::<L>(section.state.ic2)
}

#[inline(always)]
fn section_state_is_flush_shaped<L: Lane>(section: &Section<L>) -> bool {"""),
    ]),
}

path = os.path.join(tree, EDITS[bug][0])
backup = path + '.orig-' + bug
if action == 'restore':
    shutil.copyfile(backup, path)
    os.remove(backup)
    print('restored', path)
    sys.exit(0)
shutil.copyfile(path, backup)
text = open(path).read()
for old, new in EDITS[bug][1]:
    assert text.count(old) == 1, (bug, old[:60], text.count(old))
    text = text.replace(old, new)
if bug == '1015':
    lines = text.split('\n')
    # the two stationary lists: cascade_sections_mono (single channel) and cascade_sections (dual)
    def fn_start(name):
        for i, l in enumerate(lines):
            if l.startswith('fn ' + name + '<'):
                return i
        raise SystemExit('no fn ' + name)
    changed = 0
    for name in ('cascade_sections_mono', 'cascade_sections'):
        s = fn_start(name)
        for i in range(s, s + 200):
            if 'section_state_is_flush_shaped(&' in lines[i]:
                lines[i] = lines[i].replace('section_state_is_flush_shaped(&', 'section_state_is_finite_without_negative_zero(&')
                changed += 1
                if name == 'cascade_sections_mono':
                    break
            if lines[i].startswith('}') and i > s:
                break
    assert changed == 3, changed
    text = '\n'.join(lines)
open(path, 'w').write(text)
print('applied', bug, path)
