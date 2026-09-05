# Establish explicit application-sample admission for concurrent live builtin controls and enable safe bank pairing

Queued/unbriefed retained native outcome of #430 and audit #349 RT-4. No implementation authorization.

The current public HostConsoleHandles exposes independent Send SPSC producer endpoints with untimestamped fader/matrix records. Consumer::try_pop refreshes the producer cursor while draining. Thus records may arrive between the current fader and matrix process calls; browser-exclusive submission/render ownership does not prove native concurrent equivalence.

Deliver an explicit application-sample/epoch admission contract with bounded render cutoff, atomic batch admission, typed backpressure and reliable completion/cancellation semantics, then enable safe native live bank pairing under that earned contract. Before coding, Astra must determine whether admission and pairing require independently closable children and number them. Preserve every existing acknowledged application guarantee; unsupported capabilities require explicit contract decisions, never silent drops/delays. Ask: can an ack precede a drop?

Do not substitute queue emptiness, a length snapshot, arbitrary pop cap, earlier while-pop, or host policy declaration for the missing admission contract. Establish ownership, resource reservation, concurrency/late-record behavior and revision/retirement semantics from current source in a full stateless brief. Keep arithmetic integration off render control work; no allocation, locks, syscalls, unbounded refill-driven draining or shared consumers.

Eventual discriminating gates must drive real concurrent admission/application, distinguish before/after cutoff behavior and queue saturation, prove accepted record loss/cancellation behavior, and compare actual paired versus separate PCM/state plus post-fader observations. No timing authority is granted. #430 serialized bank integration and the scalar successor do not close this retained native outcome or #349 RT-4.

Astra briefs/reviews, Luna1, Sol2/3 then explicit hard-stop rebrief; root owns exact-path checkpoints and synchronized issue/PR delivery with exact-head Astra PASS and required CI.

## Numbered accounting

This is #444. #442 owns the immutable delivery-policy prerequisite; #430 owns serialized live bank pairing; #443 retains scalar pairing; #444 retains concurrent-native admission and pairing. #431 owns separately briefed measurement. None alone closes audit RT-4/#349.
