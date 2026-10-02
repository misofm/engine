# Plan multicore bank rendering within browser and mobile deadlines

## Owner request and smallest outcome

On 2026-10-02 the owner requests completing #1154's removal of the old multi-lease arena, then designing multicore rendering afresh across browser WebAssembly and iOS/Android. Their browser example is 12 tracks in three four-track banks spread over three workers, then 16 tracks in four banks with one worker taking two. This is a research/design issue: deliver a concrete architecture recommendation, primary-source platform constraints, explicit owner choices, and the smallest bounded feasibility successor. Do not implement a worker pool or revive the removed native dependency-wave scheduler.

## Product and architectural boundaries

Bank jobs and execution workers are distinct: worker count is configured/bounded rather than one thread per bank, tracks remain arbitrary within resources, browser/NEON banks are four lanes and AVX2 banks eight. Compatible program cohorts and scalar/partial tails still apply. Bank assignment must consider cost/dependencies, while actual CPU-core placement is subject to platform scheduling; no promise that a worker equals a reserved physical core. Preserve each lane's DSP arithmetic/bits, deterministic node-ID reductions, explicit sends/sidechains/PDC, stable absolute event sample times, transactional ack/admission, generation-tagged seeks, fixed prepared latency/bypass, bounded streaming and off-render retirement.

The standing launch renderer remains single-threaded until a new implementation issue earns multicore. #1154 implements its safe exclusive arena; this plan must specify ownership/transfer for worker-private state and buffers rather than preserving unsafe foreign leases for future use. No shared mutable DSP state, unconstrained work stealing, heap jobs or unlimited stem/render cache. Render must not acquire OS locks, allocate/free, log, perform I/O or depend on unbounded waits/syscalls. Any proposed conflict with this policy is an explicit owner ruling, not a silent exception.

## Required analysis

- Compare one-quantum parallel dispatch/join with bounded render-ahead/pipelining. Establish which execution context owns the final deadline, what worker completion guarantees actually exist and how scheduling jitter is handled. At 48 kHz a specified 128-frame quantum is 2.667 ms, not a guaranteed amount of available CPU time; use actual host quantum and all four launch rates rather than assuming 128 forever.
- Specify persistent worker pool sizing and bank assignment with 12/16-track examples, unequal effect costs, graph dependencies/sends/sidechains, idle workers, scalar tails and over-subscription. Separate browser, iOS and Android scheduling capabilities from shared engine logic. No unmeasured speedup, latency or core-affinity claim.
- Specify immutable job metadata, exclusive state/buffer ownership, acquire/release publication, bounded queues, block/sample/plan/seek epochs, completion barriers and deterministic reductions. Define worker lateness, backlog, crash, recovery, cancellation and retirement; never mix output from different block/plan generations or race a late worker against callback fallback. Any state continuity limitation must be candid.
- Define any added fixed output/automation latency, admission horizon and PDC/source-ring implications. Show how controls/acks remain correct when frames are already rendered; distinguish silence/bypass/drop/retry behavior and which choices need owner approval. The engine is for stem mixing/mastering, not live recording; that supports investigating render-ahead but does not authorize a latency change.
- Browser: current AudioWorklet render/deadline/thread rules, SharedArrayBuffer and cross-origin-isolation deployment, Wasm shared memory/atomics, worker priority and suspension/visibility lifecycle, Chrome/Firefox/Safari evidence and unsupported-host fallback. Vendor examples that allow dropped/duplicated blocks are not product acceptance gates. Mobile: official iOS Audio Workgroups/render-block constraints and Android low-latency/worker guidance, thermal and heterogeneous-core limits.

## Evidence and qualification plan

Start with current repo research/rulings and official primary sources; do not inspect or inherit a legacy engine architecture. Cite each platform claim at the supporting primary page, distinguish dated examples, implementation details, current standard requirements and inference. Research may produce a model/diagram and frozen prototype workload/validator proposal, but launch no timing experiment, broad benchmark framework, new corpus or production changes.

Define a stateless smallest feasibility successor with representative real DSP banks, bounded memory, zero callback allocations/frees/locks, deterministic PCM/state, installed allocator controls, deadline completion percentiles/misses, worker-stall fault injection, actual rate/quantum and truthful browser/device execution limits. Choose workload and validator before any later timing; preserve raw failed evidence and do not optimize a descriptive number. Extensive target matrices and infrastructure belong to successors. A single platform proof is not cross-platform qualification or a guarantee on all hardware.

## Execution and decisions

Root Sol approves bounded research; the owner's two GPT-6.1 Sol xhigh agents continue: A implements #1154 and B provides independent safety review plus this read-only research. #1154's frozen implementation review has priority. B may edit only this numbered research spec after root confirms matching GitHub number/title and approves it. Root owns issue/body synchronization and final owner-facing recommendation. No third agent or worker implementation is authorized.

Record owner questions at completion: acceptable additional fixed latency/control horizon, worker late-block policy, supported platform fallback and resource/CPU budget. Do not ask again for already-authorized removal or routine research. The current issue is complete only when the recommendation, evidence, decisions requiring input and bounded successor brief are published and synchronized; a worker implementation is a separate issue.

## Evidence status

Brief only. #1154 removes the unsafe generality under the owner ruling; the new multicore feature has not been implemented, benchmarked or deadline-qualified. Primary-source research and concrete recommendation pending.
