/// Reviewer's adversarial models (not the design's).
#[cfg(all(test, loom))]
mod review_models {
    use super::*;
    use loom::sync::Arc;
    use loom::thread;

    const CELLS: usize = 3;

    struct Render {
        stage: StageReader,
        readers: [CellReader; CELLS],
        applied: [Option<(u64, u32)>; CELLS],
        superseded: u64,
        applications: u64,
    }

    impl Render {
        fn new() -> Self {
            Self {
                stage: StageReader { deferred: 0 },
                readers: [CellReader::new(), CellReader::new(), CellReader::new()],
                applied: [None; CELLS],
                superseded: 0,
                applications: 0,
            }
        }
        fn drain(&mut self, cells: &[LatestCell<2>], dirty: &DirtyWord, snapshot: LiveSnapshot) {
            let pending = self.stage.pending(dirty);
            for cell in 0..cells.len() {
                if pending & (1 << cell) == 0 {
                    continue;
                }
                let (read, newer) = self.readers[cell].read(&cells[cell], snapshot);
                if newer {
                    self.stage.deferred |= 1 << cell;
                }
                if let CellRead::Applied { words, superseded, .. } = read {
                    let decoded = decode(words);
                    assert!(decoded.0 <= snapshot.0, "applied {decoded:?} above S {}", snapshot.0);
                    self.applied[cell] = Some(decoded);
                    self.superseded += superseded;
                    self.applications += 1;
                }
            }
        }
    }

    /// R1: render claims a candidate while control stamps a transaction with the candidate's
    /// unpinned pin (it was rearmed and never announced), then laps it.
    #[test]
    fn spsc_loom_review_claim_races_an_unpinned_stamp() {
        let mut builder = loom::model::Builder::new();
        builder.preemption_bound = Some(3);
        builder.check(|| {
            struct Shared {
                mailbox: AtomicU64,
                gates: [RevisionGate; 2],
                dirty: [DirtyWord; 2],
                cells: [[LatestCell<2>; 1]; 2],
            }
            let shared = Arc::new(Shared {
                mailbox: AtomicU64::new(0),
                gates: [RevisionGate::new(1), RevisionGate::new(0)],
                dirty: [DirtyWord::new(), DirtyWord::new()],
                cells: [[LatestCell::new()], [LatestCell::new()]],
            });
            // P (plan 0) holds X at revision 1, published before render starts.
            let mut writer_p = CellWriter::new();
            writer_p.write(&shared.cells[0][0], &shared.dirty[0], 0, LiveStamp { revision: 1, pinned: None }, value_words(1, 0));
            // C (plan 1): retarget at revision 2 before publication, then rearm.
            let mut writer_c = CellWriter::new();
            let mut pin_c = GatePin::default();
            writer_c.write(&shared.cells[1][0], &shared.dirty[1], 0, LiveStamp { revision: 2, pinned: None }, value_words(2, 0));
            shared.gates[1].publish(&mut pin_c, 2);
            pin_c.pinned = None; // rearm
            let control_shared = Arc::clone(&shared);
            let control = thread::spawn(move || {
                let s = &control_shared;
                // Publish C (Empty -> Full), a release compare-and-swap.
                s.mailbox.compare_exchange(0, 1, Ordering::Release, Ordering::Relaxed).unwrap();
                for revision in 3..=5 {
                    // Routed to C's cell whether it is Full or Active.
                    let stamp = pin_c.stamp(revision);
                    writer_c.write(&s.cells[1][0], &s.dirty[1], 0, stamp, value_words(revision, 0));
                    s.gates[1].publish(&mut pin_c, revision);
                }
            });
            let mut renders = [Render::new(), Render::new()];
            let mut active = 0usize;
            let mut block = |active: &mut usize, renders: &mut [Render; 2]| -> u64 {
                if *active == 0 && shared.mailbox.load(Ordering::Acquire) == 1 {
                    if shared.mailbox.compare_exchange(1, 2, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                        *active = 1;
                    }
                }
                let snapshot = shared.gates[*active].snapshot();
                renders[*active].drain(&shared.cells[*active], &shared.dirty[*active], snapshot);
                let applied = renders[*active].applied[0].map(|(r, _)| r);
                if *active == 0 {
                    assert_eq!((snapshot.0, applied), (1, Some(1)));
                } else {
                    assert_eq!(applied, Some(snapshot.0), "C applied {applied:?} at S {}", snapshot.0);
                }
                snapshot.0
            };
            block(&mut active, &mut renders);
            block(&mut active, &mut renders);
            control.join().unwrap();
            let s = block(&mut active, &mut renders);
            assert_eq!((active, s), (1, 5));
            assert_eq!(renders[1].superseded + renders[1].applications, 4, "C's writes 2..=5 resolved once each");
        });
    }

    /// R2: in-place rewrites of the in-flight revision race render's reads.
    #[test]
    fn spsc_loom_review_in_place_rewrite_races_reads() {
        let mut builder = loom::model::Builder::new();
        builder.preemption_bound = Some(3);
        builder.check(|| {
            let cells = Arc::new(([LatestCell::<2>::new()], DirtyWord::new(), RevisionGate::new(0)));
            let mut pin = GatePin::default();
            let mut writer = CellWriter::new();
            writer.write(&cells.0[0], &cells.1, 0, pin.stamp(1), value_words(1, 0));
            cells.2.publish(&mut pin, 1);
            let c = Arc::clone(&cells);
            let control = thread::spawn(move || {
                for revision in 2..=3 {
                    for k in 0..2 {
                        writer.write(&c.0[0], &c.1, 0, pin.stamp(revision), value_words(revision, k));
                    }
                    c.2.publish(&mut pin, revision);
                }
            });
            let mut render = Render::new();
            let expected = |s: u64| if s == 1 { (1, 0) } else { (s, 1) };
            for _ in 0..2 {
                let snapshot = cells.2.snapshot();
                render.drain(&cells.0, &cells.1, snapshot);
                assert_eq!(render.applied[0], Some(expected(snapshot.0)));
            }
            control.join().unwrap();
            let snapshot = cells.2.snapshot();
            render.drain(&cells.0, &cells.1, snapshot);
            assert_eq!(render.applied[0], Some((3, 1)));
            assert_eq!(render.superseded + render.applications, 5);
        });
    }

    /// R3: partial transactions over three cells in one dirty word: every block equals the
    /// committed model at its snapshot.
    #[test]
    fn spsc_loom_review_partial_transactions_match_the_model_at_s() {
        let mut builder = loom::model::Builder::new();
        builder.preemption_bound = Some(2);
        builder.check(|| {
            // T1 writes X, Y; T2 writes Y; T3 writes X, Z.
            const PLAN: [(u64, &[usize]); 3] = [(1, &[0, 1]), (2, &[1]), (3, &[0, 2])];
            let shared = Arc::new(([LatestCell::<2>::new(), LatestCell::new(), LatestCell::new()], DirtyWord::new(), RevisionGate::new(0)));
            let c = Arc::clone(&shared);
            let control = thread::spawn(move || {
                let mut pin = GatePin::default();
                let mut writers = [CellWriter::new(), CellWriter::new(), CellWriter::new()];
                for (revision, cells) in PLAN {
                    for &cell in cells {
                        writers[cell].write(&c.0[cell], &c.1, cell as u32, pin.stamp(revision), value_words(revision, 0));
                    }
                    c.2.publish(&mut pin, revision);
                }
            });
            let model = |s: u64| -> [Option<(u64, u32)>; 3] {
                let mut state = [None; 3];
                for (revision, cells) in PLAN {
                    if revision <= s {
                        for &cell in cells {
                            state[cell] = Some((revision, 0));
                        }
                    }
                }
                state
            };
            let mut render = Render::new();
            for _ in 0..2 {
                let snapshot = shared.2.snapshot();
                render.drain(&shared.0, &shared.1, snapshot);
                assert_eq!(render.applied, model(snapshot.0), "S = {}", snapshot.0);
            }
            control.join().unwrap();
            let snapshot = shared.2.snapshot();
            render.drain(&shared.0, &shared.1, snapshot);
            assert_eq!(render.applied, model(3));
            assert_eq!(render.superseded + render.applications, 5);
        });
    }
}
