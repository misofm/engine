# Protected one-shot spectrum delivery through the observation owner

## Scheduling and scope

Deferred successor to #822, scheduled after the first working protected browser EQ. #822 intentionally implements protected Continuous only and atomically refuses protected OneShot; the legacy permanent one-shot API remains unchanged. This issue owns the single independently useful result: request one prepared target, capture one window, remove demand through the reserved path, and deliver that result without losing an accepted obligation. No general continuous result cache, new graph scheduler, FFT worker, browser UI or simultaneous-job work.

## Contract to freeze before implementation

Extend the existing private HostObservationController and paired controlled slots. Preserve owner/observation-generation/selection/history identity boundaries, complete meter/spectrum selections, Free-only staging and exact retirement receipts. One-shot completion must request automatic reserved removal; no graph rebuild or render cleanup. Retain at most one undelivered generation-tagged result in prepared owner storage, charged by actual containing layout. Freeze read/delivery ordering, pending automatic removal when reserved credit is busy, retry after a refused removal, accepted replacement policy while a result is undelivered, stop and terminal closure behavior, and checked sequence exhaustion. No ack may precede a silent drop. Admission refusal preserves active state and previous committed result.

Root with Astra XHIGH must amend this stateless body with the exact API/state table and objective gates before a bounded Luna MAX implementation starts; this scheduling record does not authorize speculative semantics. Representative gates must exercise capture completion during reserved-slot contention, read retry, replacement/stop, failed render and terminal closure, no dormant work after applied removal, checked exact storage/work budgets, and existing realtime allocation guards. Reuse #822 fixtures and transport.

## Delivery

Sequential compiling exact-path checkpoints with root commit/push and remote evidence. Fresh Astra XHIGH review, maximum five attempts, required qualification, merged evidence and issue closure. User preference remains working browser EQ first; this successor must not delay that milestone.
