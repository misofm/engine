use miso_engine_effect_contract::NativeEffectRegistry;
use miso_engine_session::CompiledSession;
#[derive(Clone, Copy, Debug)]
pub struct EffectCompileCaps {
    pub maximum_state_bytes: u64,
    pub maximum_scratch_bytes: u64,
    pub maximum_automation_spans: u32,
}
pub trait PackageStore {
    fn bytes_for(&self, cid: &crate::EffectCid) -> Option<&[u8]>;
}
pub struct EffectPreparedSession {
    pub compiled: CompiledSession,
}
#[derive(Debug)]
pub struct EffectPreparationFailure {
    pub code: &'static str,
}
#[derive(Debug)]
pub struct EffectPreparationError {
    pub session: CompiledSession,
    pub diagnostics: Vec<EffectPreparationFailure>,
}
pub fn prepare_session_effects(
    session: CompiledSession,
    _registry: &NativeEffectRegistry,
    _store: Option<&dyn PackageStore>,
    _caps: EffectCompileCaps,
) -> Result<EffectPreparedSession, Box<EffectPreparationError>> {
    if session
        .normalized_model()
        .tracks
        .iter()
        .flat_map(|t| [&t.simd1, &t.dynamic, &t.simd2])
        .flat_map(|r| &r.effects)
        .next()
        .is_some()
    {
        Err(Box::new(EffectPreparationError {
            session,
            diagnostics: vec![EffectPreparationFailure {
                code: "effect.prepare.unimplemented",
            }],
        }))
    } else {
        Ok(EffectPreparedSession { compiled: session })
    }
}
