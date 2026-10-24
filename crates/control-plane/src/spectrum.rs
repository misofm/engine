//! The session's spectrum monitor state (issue #1593 D1).
//!
//! A session's spectrum taps are monitor state (ruling C9): they are not in the session model, are
//! never saved and never advance the revision. The session gives its configuration to every plan
//! it prepares, and the monitor owns the service that reads the prepared captures and the one
//! analysis that smooths its stream.

use host_core::{
    PreparedSpectrumCaptures, SpectrumAnalysis, SpectrumConfiguration, SpectrumHop, SpectrumPair,
    SpectrumService,
};

/// One session's spectrum monitor: its configuration, the epochs that tell a reader which
/// configuration the captures it reads were prepared with, the service over the running plan's
/// captures and the stream's analysis.
///
/// A monitor with no configuration holds no service and no analysis, so a session without
/// spectrum allocates nothing for it (#1593 gate 7).
pub struct SpectrumMonitor {
    configuration: Option<SpectrumConfiguration>,
    /// 0 with no configuration at boot; one more at every change.
    configured_epoch: u64,
    /// The configuration epoch of the captures the service reads.
    adopted_epoch: u64,
    service: Option<SpectrumService>,
    /// The stream's one analysis, made with the service.
    analysis: Option<SpectrumAnalysis>,
    /// The session's rate and quantum, for a service a promotion makes.
    sample_rate_hz: u32,
    quantum_frames: u32,
}

/// The spectrum half of one provider epoch (#1593 D5).
#[derive(Default)]
#[allow(
    clippy::large_enum_variant,
    reason = "one per held epoch; boxing would allocate at promotion"
)]
pub(crate) enum EpochSpectrum {
    /// Nothing: the boot epoch, or an epoch whose promotion displaced nothing.
    #[default]
    None,
    /// A candidate's captures (`None` when it was prepared with no configuration), the stream hop
    /// of the configuration it was prepared with, and that configuration's epoch. Its promotion installs them in the service,
    /// paired by the candidate's own pairing list
    /// ([`host_core::PlanStateInventory::spectrum_pairing`]).
    Candidate {
        captures: Option<PreparedSpectrumCaptures>,
        hop: Option<SpectrumHop>,
        epoch: u64,
    },
    /// What the promotion of this epoch's successor displaced: dropped with the parked epoch,
    /// on the control thread, when its plan is reclaimed.
    Displaced(
        #[allow(dead_code, reason = "held only to be dropped with the parked epoch")]
        SpectrumDisplaced,
    ),
}

/// The captures, or the whole service, a promotion displaced (#1593 D6).
pub(crate) struct SpectrumDisplaced {
    #[allow(dead_code, reason = "held only to be dropped with the parked epoch")]
    captures: Option<PreparedSpectrumCaptures>,
    #[allow(dead_code, reason = "held only to be dropped with the parked epoch")]
    service: Option<(SpectrumService, Option<SpectrumAnalysis>)>,
}

impl SpectrumMonitor {
    /// The boot monitor (#1593 D2): `configuration` and the captures the first plan prepared for
    /// it, at the session's rate and quantum. Both epochs start at 1 with a configuration and at 0
    /// without one.
    pub(crate) fn new(
        configuration: Option<SpectrumConfiguration>,
        captures: Option<PreparedSpectrumCaptures>,
        sample_rate_hz: u32,
        quantum_frames: u32,
    ) -> Self {
        let epoch = u64::from(configuration.is_some());
        let service = configuration.as_ref().and_then(|configuration| {
            captures.map(|captures| {
                SpectrumService::new(captures, sample_rate_hz, quantum_frames, configuration.hop)
            })
        });
        let analysis = service.as_ref().map(|_| SpectrumAnalysis::new());
        Self {
            configuration,
            configured_epoch: epoch,
            adopted_epoch: epoch,
            service,
            analysis,
            sample_rate_hz,
            quantum_frames,
        }
    }

    /// The configuration a structural successor of `model` is prepared with (#1593 D4): every
    /// entry whose target `model` still has, or `None`. Nothing is stored: the session stores it
    /// with [`Self::commit`] after the protocol commit. Allocates the kept entries' copy.
    pub(crate) fn successor_configuration(
        &self,
        model: &session::SessionModel,
    ) -> Option<SpectrumConfiguration> {
        self.configuration
            .as_ref()
            .and_then(|configuration| configuration.retain_present(model))
    }

    /// Store a committed successor's configuration (#1593 D4), beside the candidate push: the
    /// configuration epoch advances by one when it differs. Returns the epoch the successor's
    /// captures were prepared for. Allocates nothing.
    pub(crate) fn commit(&mut self, configuration: Option<SpectrumConfiguration>) -> u64 {
        if configuration != self.configuration {
            self.configured_epoch = self.configured_epoch.saturating_add(1);
            self.configuration = configuration;
        }
        self.configured_epoch
    }

    /// Install a promoted candidate's captures (#1593 D5, D6), on the control thread after render
    /// adopted its plan: the service pairs every capture `pairing` names (the observers the swap
    /// block moved) and settles the others; a service it makes takes `hop`, the stream hop of the
    /// configuration the candidate was prepared with; with no captures the service is removed, so every
    /// read answers `Unprepared`. `adopted_epoch` becomes `epoch`. Returns what it displaced, for
    /// the caller to drop off the audio thread.
    ///
    /// The session publishes a candidate only against the plan render runs, so the adoption
    /// carries by construction (as the source producers' `adopt_persisting` relies on).
    pub(crate) fn adopt(
        &mut self,
        captures: Option<PreparedSpectrumCaptures>,
        pairing: &[SpectrumPair],
        hop: Option<SpectrumHop>,
        epoch: u64,
    ) -> SpectrumDisplaced {
        self.adopted_epoch = epoch;
        let Some(captures) = captures else {
            let service = self
                .service
                .take()
                .map(|service| (service, self.analysis.take()));
            return SpectrumDisplaced {
                captures: None,
                service,
            };
        };
        let Some(service) = self.service.as_mut() else {
            // A configuration made after boot or after every tap was removed (#1594): the first
            // captures make the service, with the candidate's own hop.
            self.service = Some(SpectrumService::new(
                captures,
                self.sample_rate_hz,
                self.quantum_frames,
                hop,
            ));
            self.analysis = Some(SpectrumAnalysis::new());
            return SpectrumDisplaced {
                captures: None,
                service: None,
            };
        };
        // The next stream starts with the hop of the configuration the candidate carries; a
        // stream the swap keeps runs on with its own cadence (#1594 D5).
        service.set_hop(hop);
        let displaced = service.adopt_successor(captures, pairing, true, self.analysis.as_mut());
        SpectrumDisplaced {
            captures: Some(displaced),
            service: None,
        }
    }

    /// Store a configuration that changes only the hop or the budget of the session's (#1594 D5):
    /// no plan is prepared. The configuration epoch advances by one, and so does the adopted
    /// epoch when the running plan's captures are of the configuration replaced (they hold the
    /// same taps); the service's next stream starts with the new hop. Returns the epoch replaced,
    /// for the caller to move a pending candidate of it to the new one. Allocates nothing.
    pub(crate) fn retune(&mut self, configuration: SpectrumConfiguration) -> u64 {
        let replaced = self.configured_epoch;
        self.configured_epoch = replaced.saturating_add(1);
        if self.adopted_epoch == replaced {
            self.adopted_epoch = self.configured_epoch;
            if let Some(service) = self.service.as_mut() {
                service.set_hop(configuration.hop);
            }
        }
        self.configuration = Some(configuration);
        replaced
    }

    /// The session's configuration, or `None`.
    #[must_use]
    pub const fn configuration(&self) -> Option<&SpectrumConfiguration> {
        self.configuration.as_ref()
    }

    /// The epoch of the configuration: 0 with none at boot, one more at every change.
    #[must_use]
    pub const fn configured_epoch(&self) -> u64 {
        self.configured_epoch
    }

    /// The configuration epoch of the captures the service reads.
    #[must_use]
    pub const fn adopted_epoch(&self) -> u64 {
        self.adopted_epoch
    }

    /// The service over the running plan's captures, or `None` with no configuration.
    #[must_use]
    pub const fn service(&self) -> Option<&SpectrumService> {
        self.service.as_ref()
    }

    /// The stream's analysis, or `None` with no service.
    #[must_use]
    pub const fn analysis(&self) -> Option<&SpectrumAnalysis> {
        self.analysis.as_ref()
    }

    /// The service and the analysis at once, as the stream calls take them, or `None` with no
    /// configuration.
    pub fn service_and_analysis_mut(
        &mut self,
    ) -> Option<(&mut SpectrumService, &mut SpectrumAnalysis)> {
        self.service.as_mut().zip(self.analysis.as_mut())
    }
}
