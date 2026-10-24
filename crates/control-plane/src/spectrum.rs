//! The session's spectrum monitor state (issue #1593 D1).
//!
//! A session's spectrum taps are monitor state (ruling C9): they are not in the session model, are
//! never saved and never advance the revision. The session gives its configuration to every plan
//! it prepares, and the monitor owns the service that reads the prepared captures and the one
//! analysis that smooths its stream.

use host_core::{
    PreparedSpectrumCaptures, SpectrumAnalysis, SpectrumConfiguration, SpectrumPreparation,
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
        }
    }

    /// The preparation every plan the session prepares carries, or `None`.
    pub(crate) fn preparation(&self) -> Option<&SpectrumPreparation> {
        self.configuration
            .as_ref()
            .map(|configuration| &configuration.preparation)
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
