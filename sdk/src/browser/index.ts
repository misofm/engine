/** Package-owned browser boot, with independent context, scratch and host overrides. */
export * from "./engine.ts";
export * from "./live-controls.ts";
export { EngineLiveControls } from "../core/live-controls.ts";
export { TransactionBuilder, transaction } from "../core/session-transaction.ts";
export type { SessionEditSpec, SessionTransaction } from "../core/session-transaction.ts";
export type { EditOutcome, EditPath, WatermarkSnapshot } from "../core/session-edit.ts";
export type { ControlEditSet, LiveControlTransport, SetControlOptions } from "../core/live-controls.ts";
export { ControlRequestIds, ControlTransactionQueue } from "../core/control-queue.ts";
export type {
  ControlAppliedReport,
  ControlQueueOptions,
  ControlRefusedReport,
  ControlReport,
  ControlSend,
} from "../core/control-queue.ts";
export {
  CONTROL_LIST_PAGE_ROWS,
  CONTROL_LIST_RESTARTS,
  CONTROL_LIST_STALE,
  decodeControlListPage,
  decodeControlStatePage,
  encodeControlListRequest,
  readControlList,
  readControlState,
} from "../core/control-list.ts";
export type {
  ControlChoice,
  ControlClassName,
  ControlFamilyName,
  ControlHeader,
  ControlHeaderAddress,
  ControlList,
  ControlListEntry,
  ControlListFilter,
  ControlListPage,
  ControlListRequest,
  ControlListTransport,
  ControlOwnerHeader,
  ControlOwnerHeaderKind,
  ControlPageRefusal,
  ControlPageResult,
  ControlRampRowName,
  ControlReasonName,
  ControlRouteEnds,
  ControlScopeName,
  ControlSectionHeader,
  ControlState,
  ControlStatePage,
  ControlTaperName,
  ControlTapName,
  ControlUnitName,
  ControlValueKind,
} from "../core/control-list.ts";
export {
  COUNTER_IDS,
  ProtocolClient,
  SESSION_SNAPSHOT_PAGE_BYTES,
  SESSION_SNAPSHOT_RESTARTS,
  decodeCapabilitiesReply,
  decodeCountersReply,
  decodeDiagnosticsReply,
  decodeProtocolEvent,
  decodeSessionSnapshotReply,
  decodeTelemetryReply,
  decodeTransportReply,
  encodeCapabilitiesRequest,
  encodeCountersRequest,
  encodeDiagnosticsRequest,
  encodeSessionSnapshotRequest,
  encodeTelemetryConfigureRequest,
  encodeTransportGetRequest,
  encodeTransportSetRequest,
} from "../core/protocol-messages.ts";
export type {
  CommittedDocument,
  CounterName,
  CounterSnapshot,
  CounterValue,
  DiagnosticPathSegment,
  DiagnosticSeverityName,
  DiagnosticsPage,
  DiagnosticsQuery,
  EngineCapabilities,
  MeterComponentName,
  MeterRecord,
  ProtocolDiagnostic,
  ProtocolEvent,
  ProtocolEventKind,
  ProtocolRefusal,
  ProtocolReply,
  ProtocolTransport,
  ProtocolVersion,
  SessionSnapshotPage,
  TelemetryConfiguration,
  TransportRequest,
  TransportSnapshot,
  TransportStateName,
} from "../core/protocol-messages.ts";
export { MisoControlRefusal, MisoEngineError, MisoUsageError, SessionSeekProducerError } from "../core/errors.ts";
export type {
  MasterMeter,
  MeterListener,
  MeterUpdate,
  TelemetryListener,
  TelemetryUpdate,
  TrackMeter,
} from "./measurement.ts";
export * from "./policy.ts";
// The `BootOptions` -> `MisoWebBootOptions` adapter. It belongs to whoever mounts the shipped
// worklet host directly instead of going through `createEngine`, which is a browser consumer, so
// it is barrel surface rather than a deep import.
export * from "./host-mirror.ts";
export * from "./pcm-ring.ts";
// Issue #2102: the WAV header reader, the canonical PCM pump and the engine's BLAKE3 identity.
export * from "./stems.ts";
// By name: `linkFeedEngine` and `FeedRetirement` are `createEngine`'s seam (#1297b), not public.
export { PcmFeedError, PcmRunwayError, attachEngineFeed, prepareEngineFeed, waitForPcmRunway } from "./pcm-feed.ts";
export type {
  EngineFeed,
  FeedContext,
  FeedEngine,
  FeedNode,
  FeedNodeOptions,
  FeedOptions,
  FeedPort,
  FeedSource,
  FeedStart,
  FeedStartSeek,
  PcmFeedOperation,
  PcmRunwayReason,
  PcmRunwaySource,
  WaitForPcmRunwayOptions,
} from "./pcm-feed.ts";
export { BrowserResponsePreview, createResponsePreview } from "./response.ts";
export type { BrowserResponsePreviewOptions } from "./response.ts";
export * from "../core/observation.ts";
export { DEFAULT_OBSERVATION_SUBSCRIPTION_LIMITS } from "../core/observation-subscriptions.ts";
export type {
  ObservationSubscription,
  ObservationSubscriptionBounds,
  ObservationSubscriptionConfiguration,
  ObservationSubscriptionLimits,
  ObservationSubscriptionEnd,
  ObservationSubscriptionNotification,
  ObservationSubscriptionReceipt,
  ObservationSubscriptionRequest,
} from "../core/observation-subscriptions.ts";
export { DEFAULT_TRACK_RESPONSE_SUBSCRIPTION_LIMITS } from "../core/observation-subscriptions.ts";
export type {
  TrackResponseSubscription,
  TrackResponseSubscriptionBounds,
  TrackResponseSubscriptionConfiguration,
  TrackResponseSubscriptionLimits,
  TrackResponseSubscriptionNotification,
  TrackResponseSubscriptionReceipt,
  TrackResponseSubscriptionRequest,
  SpectrumSubscription,
  SpectrumSubscriptionBounds,
  SpectrumSubscriptionConfiguration,
  SpectrumSubscriptionLimits,
  SpectrumSubscriptionNotification,
  SpectrumSubscriptionReceipt,
  SpectrumSubscriptionRequest,
} from "../core/observation-subscriptions.ts";
export { DEFAULT_SPECTRUM_SUBSCRIPTION_LIMITS } from "../core/observation-subscriptions.ts";
export type {
  TrackResponseGrid,
  TrackResponseLimits,
  TrackResponseMember,
  TrackResponseQuery,
  TrackResponseResult,
} from "../core/live-response.ts";
export type {
  SpectrumCollection,
  SpectrumCollectionEntry,
  SpectrumLimits,
  SpectrumQuery,
  SpectrumResult,
  SpectrumTarget,
} from "../core/spectrum.ts";
export type {
  SpectrumStreamMetadata,
  SpectrumStreamRead,
  SpectrumStreamSelection,
  SpectrumStreamStart,
  SpectrumStreamStatus,
} from "../core/spectrum.ts";

export { scratchBootWithWorker, prepareBrowserSessionWithWorker } from "./scratch.ts";
export type { ScratchWorker, ScratchWorkerFactory, ScratchBootWorkerOptions, PreparedBrowserSession } from "./scratch.ts";
export { createDefaultHost, BrowserBootError } from "./default-host.ts";
