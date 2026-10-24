//! Issue #1400 D4: a plan carries host data that render never touches.
//!
//! The attachment is set on the control thread, rides inside the plan through publication and the
//! swap, and drops with the plan on the retirement path. The tests hold two claims: render never
//! calls the attachment, and the drop runs on the retirer's thread, not the render thread's. That
//! render allocates nothing with an attachment present needs an allocator, which `engine` cannot
//! install (it forbids `unsafe`): `crates/host-core/tests/host_attachment_render.rs` holds it.

#![allow(missing_docs)]

use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::ThreadId;

use engine::realtime::{
    HostAttachment, PlanExchangeConfig, PlanarBufferMut, PrepareRenderPlan, PreparedRenderPlan,
    RenderEnvelope, RenderIo, RenderTime, SwapOutcome, plan_exchange,
};
use engine::{QuantumFrames, SampleRateHz};

/// Counts every call into it and records where it dropped.
struct Probe {
    calls: Arc<AtomicUsize>,
    dropped_on: Arc<Mutex<Option<ThreadId>>>,
}

impl HostAttachment for Probe {
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self
    }
}

impl Drop for Probe {
    fn drop(&mut self) {
        *self.dropped_on.lock().expect("drop record") = Some(std::thread::current().id());
    }
}

fn plan(id: u64) -> PreparedRenderPlan {
    PreparedRenderPlan::prepare(PrepareRenderPlan {
        plan_id: id,
        envelope: RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(2),
            output_channels: NonZeroUsize::new(1).expect("one"),
        },
        scratch: &[],
    })
    .expect("plan")
}

fn probe() -> (Probe, Arc<AtomicUsize>, Arc<Mutex<Option<ThreadId>>>) {
    let calls = Arc::new(AtomicUsize::new(0));
    let dropped_on = Arc::new(Mutex::new(None));
    (
        Probe {
            calls: Arc::clone(&calls),
            dropped_on: Arc::clone(&dropped_on),
        },
        calls,
        dropped_on,
    )
}

#[test]
fn the_attachment_downcasts_to_its_type_and_to_nothing_else() {
    struct Other;
    let mut plan = plan(1);
    assert!(plan.host_attachment_mut::<Probe>().is_none());
    let (attachment, _, _) = probe();
    plan.set_host_attachment(Box::new(attachment));
    assert!(plan.host_attachment_mut::<Probe>().is_some());
    assert!(plan.host_attachment_mut::<Other>().is_none());
}

/// Test value: red if render reads, calls or drops the attachment: a render path that reached for
/// the attachment (to hand it over at the swap, say) would count a call or drop the displaced
/// plan's attachment on the render thread. Existing tests carry no attachment, so none of them
/// sees it.
#[test]
fn render_and_swap_never_touch_the_attachment_and_retirement_drops_it_off_render() {
    let (first, first_calls, first_dropped) = probe();
    let (second, second_calls, second_dropped) = probe();
    let mut initial = plan(1);
    initial.set_host_attachment(Box::new(first));
    let mut successor = plan(2);
    successor.set_host_attachment(Box::new(second));
    let (mut publisher, mut owner, mut retirer) = plan_exchange(
        initial,
        PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        },
    )
    .expect("exchange");
    publisher
        .publish(successor, 0)
        .unwrap_or_else(|_| panic!("publish"));

    let render_thread = std::thread::current().id();
    let mut output = [1.0_f32; 2];
    let mut swap = None;
    let mut sample = 0_u64;
    // The first block applies the swap; the rest run on the successor.
    {
        for block in 0..4 {
            let io = RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
            };
            let report = owner
                .render(
                    io,
                    RenderTime {
                        absolute_sample: sample,
                    },
                )
                .expect("render");
            if block == 0 {
                swap = Some(report.swap);
            }
            sample += 2;
        }
    }
    assert_eq!(swap, Some(SwapOutcome::Applied));
    assert_eq!(first_calls.load(Ordering::SeqCst), 0);
    assert_eq!(second_calls.load(Ordering::SeqCst), 0);
    assert!(
        first_dropped.lock().expect("record").is_none(),
        "the displaced plan's attachment survived the swap until retirement"
    );

    // Retirement runs on its own thread, off the render thread.
    let retirement = std::thread::spawn(move || {
        let retired = retirer.try_reclaim().unwrap_or_else(|_| panic!("reclaim"));
        drop(retired);
        std::thread::current().id()
    });
    let retirer_thread = retirement.join().expect("retirer");
    assert_ne!(retirer_thread, render_thread);
    assert_eq!(*first_dropped.lock().expect("record"), Some(retirer_thread));
    assert!(second_dropped.lock().expect("record").is_none());
    drop(owner);
}
