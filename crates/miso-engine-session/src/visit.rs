//! One public, schema-keyed emit-side walk over the complete session model.

use crate::StableId;

/// One schema field: canonical TOML key and BTLV field id.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldKey {
    /// Canonical TOML field name.
    pub name: &'static str,
    /// Stable BTLV field identifier.
    pub id: u16,
}

/// One closed-set token: canonical text and BTLV `u8`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    /// Canonical TOML token.
    pub text: &'static str,
    /// Stable nonzero BTLV token.
    pub wire: u8,
}

/// Model traversal order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WalkOrder {
    /// Model order (wire and estimator); never allocates.
    Declared,
    /// Entity sets by ID and parameters by `(parameter_id, channel)`.
    Canonical,
}

/// Consumer of the schema-keyed model event stream.
pub trait ModelVisitor {
    /// Visitor-specific failure.
    type Error;
    /// Begin a record. `key` is absent for the root and array elements; `wire_fields` is the
    /// emitted occurrence count, including repeated fields and any wire tag.
    fn record_begin(&mut self, key: Option<FieldKey>, wire_fields: u32) -> Result<(), Self::Error>;
    /// End the current record.
    fn record_end(&mut self) -> Result<(), Self::Error>;
    /// Begin an array with its exact item count.
    fn array_begin(&mut self, key: FieldKey, len: usize) -> Result<(), Self::Error>;
    /// End the current array.
    fn array_end(&mut self) -> Result<(), Self::Error>;
    /// Emit a discriminator present only on the wire, immediately after `record_begin`.
    fn wire_tag(&mut self, tag: Token) -> Result<(), Self::Error>;
    /// Emit a Boolean field.
    fn bool(&mut self, key: FieldKey, value: bool) -> Result<(), Self::Error>;
    /// Emit an unsigned eight-bit field.
    fn u8(&mut self, key: FieldKey, value: u8) -> Result<(), Self::Error>;
    /// Emit an unsigned 32-bit field.
    fn u32(&mut self, key: FieldKey, value: u32) -> Result<(), Self::Error>;
    /// Emit an unsigned 64-bit field.
    fn u64(&mut self, key: FieldKey, value: u64) -> Result<(), Self::Error>;
    /// Emit a finite 32-bit float field.
    fn f32(&mut self, key: FieldKey, value: f32) -> Result<(), Self::Error>;
    /// Emit a stable-ID field.
    fn id(&mut self, key: FieldKey, value: &StableId) -> Result<(), Self::Error>;
    /// Emit arbitrary UTF-8 text.
    fn text(&mut self, key: FieldKey, value: &str) -> Result<(), Self::Error>;
    /// Emit a closed token.
    fn token(&mut self, key: FieldKey, value: Token) -> Result<(), Self::Error>;
}

/// A model value exposing the shared emit-side walk.
pub trait VisitModel {
    /// Walk this value exactly once in the requested order.
    fn visit<V: ModelVisitor>(&self, order: WalkOrder, visitor: &mut V) -> Result<(), V::Error>;
}

macro_rules! fields { ($($name:ident=$text:literal:$id:literal),+ $(,)?) => {$(#[doc=concat!("Schema key `",$text,"`.")] pub const $name: FieldKey=FieldKey{name:$text,id:$id};)+}; }
macro_rules! key_module { ($name:ident,$doc:literal;$($field:tt)*) => {#[doc=$doc] pub mod $name {use super::*; fields!($($field)*);}}; }

/// Public field-key registry. IDs are transcribed from
/// `miso-engine-protocol/src/schema.rs` session specs (lines 764–1076 at #107 §6.5).
#[rustfmt::skip]
pub mod keys {
 use super::FieldKey;
 key_module!(session,"Session root fields.";SCHEMA_VERSION="schema_version":1,SESSION_ID="session_id":2,REVISION="revision":3,SAMPLE_RATE_HZ="sample_rate_hz":4,QUANTUM_FRAMES="quantum_frames":5,RENDER_PROFILE="render_profile":6,OUTPUT_PROFILE="output_profile":7,LIMITS="limits":8,SOURCES="sources":9,TRACKS="tracks":10,SUBMIXES="submixes":11,OUTPUTS="outputs":12,ROUTES="routes":13,AUTOMATION="automation":14);
 key_module!(render_profile,"Render-profile fields.";ID="id":1,MODE="mode":2);
 key_module!(output_profile,"Output-profile fields.";ID="id":1,CHANNELS="channels":2,SAMPLE_FORMAT="sample_format":3);
 key_module!(limits,"Session-limit fields.";PCM_RING_FRAMES="pcm_ring_frames":1,CONTROL_QUEUE_MESSAGES="control_queue_messages":2,MEMORY_BYTES="memory_bytes":3);
 key_module!(source,"Source fields.";ID="id":1,SAMPLE_RATE_HZ="sample_rate_hz":2,CONTENT="content":3,MAPPING="mapping":4);
 key_module!(content,"Source-content fields.";IDENTITY="identity":1,LOCATOR="locator":2);
 key_module!(mapping,"Source-mapping fields.";CHANNEL_COUNT="channel_count":1,REGION="region":2);
 key_module!(region,"Source-region fields.";START_SAMPLE="start_sample":1,LENGTH_SAMPLES="length_samples":2);
 key_module!(track,"Track fields.";ID="id":1,SOURCE_ID="source_id":2,LEFT_SOURCE_CHANNEL="left_source_channel":3,RIGHT_SOURCE_CHANNEL="right_source_channel":4,BUILTINS="builtins":5,SIMD1="simd1":6,DYNAMIC="dynamic":7,SIMD2="simd2":8,FADER="fader":9,PAN="pan":10,MATRIX="matrix":10);
 key_module!(builtins,"Dual-mono builtins fields.";LEFT="left":1,RIGHT="right":2);
 key_module!(channel_builtins,"Channel-builtin fields.";POLARITY_INVERT="polarity_invert":1,TRIM_DB="trim_db":2,HPF_HZ="hpf_hz":3,LPF_HZ="lpf_hz":4,DELAY_SAMPLES="delay_samples":5);
 key_module!(rack,"Rack fields.";EFFECTS="effects":1);
 key_module!(effect,"Effect fields.";ID="id":1,IDENTITY="identity":2,QUALITY="quality":3,BYPASS="bypass":4,LINK_MODE="link_mode":5,PARAMS="params":6,SIDECHAIN="sidechain":7);
 key_module!(identity,"Effect-identity fields.";KIND="kind":1,EFFECT_ID="effect_id":2,CID="cid":2);
 key_module!(param,"Effect-parameter fields.";PARAMETER_ID="parameter_id":1,CHANNEL="channel":2,UNIT="unit":3,VALUE="value":4);
 key_module!(sidechain,"Sidechain fields.";KIND="kind":1,SOURCE="source":2,PORT_ID="port_id":3);
 key_module!(fader,"Fader fields.";LEFT_DB="left_db":1,RIGHT_DB="right_db":2,LEFT_MUTE="left_mute":3,RIGHT_MUTE="right_mute":4);
 key_module!(matrix_or_pan,"Matrix-or-pan fields.";LEFT="left":2,RIGHT="right":3,PAN_SMOOTHING="smoothing_samples":4,LL="ll":2,LR="lr":3,RL="rl":4,RR="rr":5,MATRIX_SMOOTHING="smoothing_samples":6);
 key_module!(submix,"Submix fields.";ID="id":1);
 key_module!(output,"Output fields.";ID="id":1);
 key_module!(route,"Route fields.";ID="id":1,SOURCE="source":2,DESTINATION="destination":3,CHANNEL_MATRIX="channel_matrix":4,GAIN_DB="gain_db":5);
 key_module!(route_source,"Route-source fields.";KIND="kind":1,TRACK_ID="track_id":2,SUBMIX_ID="submix_id":2,TAP="tap":3);
 key_module!(route_destination,"Route-destination fields.";KIND="kind":1,SUBMIX_ID="submix_id":2,OUTPUT_ID="output_id":2);
 key_module!(channel_matrix,"Channel-matrix fields.";LL="ll":1,LR="lr":2,RL="rl":3,RR="rr":4);
 key_module!(automation,"Automation fields.";ID="id":1,TARGET="target":2,SEGMENTS="segments":3);
 key_module!(target,"Automation-target fields.";ENTITY_ID="entity_id":1,RACK="rack":2,EFFECT_ID="effect_id":3,PARAMETER_ID="parameter_id":4,CHANNEL="channel":5);
 key_module!(segment,"Automation-segment fields.";SHAPE="shape":1,START_SAMPLE="start_sample":2,END_SAMPLE="end_sample":3,START_VALUE="start_value":4,END_VALUE="end_value":5,UNIT="unit":6);
}

mod walk {
    use core::cmp::Ordering;

    use super::{FieldKey, ModelVisitor, Token, VisitModel, WalkOrder, keys};
    use crate::*;

    fn token(text: &'static str, wire: u8) -> Token {
        Token { text, wire }
    }
    trait Record {
        fn record<V: ModelVisitor>(
            &self,
            key: Option<FieldKey>,
            order: WalkOrder,
            v: &mut V,
        ) -> Result<(), V::Error>;
    }
    macro_rules! records { ($($ty:ty=>$m:ident |$s:ident,$v:ident,$o:ident,$f:ident| [$count:expr] {$($event:expr),*})+) => {$(
        impl Record for $ty { fn record<V: ModelVisitor>(&self, key: Option<FieldKey>, o: WalkOrder, v: &mut V) -> Result<(), V::Error> {
            use keys::$m as $f; let ($s,$v,$o)=(self,v,o);
            $v.record_begin(key,$count)?; $($event?;)* $v.record_end()
        } }
        impl VisitModel for $ty { fn visit<V: ModelVisitor>(&self, o: WalkOrder, v: &mut V) -> Result<(), V::Error> { self.record(None,o,v) } }
    )+}; }
    macro_rules! rec { ($v:ident,$key:expr,$n:expr;$($e:expr),*$(,)?) => {{ $v.record_begin($key,$n)?; $($e?;)* $v.record_end() }}; }
    macro_rules! variants { ($($ty:ty=>$m:ident |$v:ident,$f:ident,$o:ident| {$([$pat:pat]=>$n:literal {$($e:expr),*})+})+) => {$(
        impl Record for $ty {
            fn record<V: ModelVisitor>(&self, key: Option<FieldKey>, order: WalkOrder, visitor: &mut V) -> Result<(), V::Error> {
                use keys::$m as $f; let ($v,$o)=(visitor,order);
                match self {$($pat=>rec!($v,key,$n;$($e),*),)+}
            }
        }
        impl VisitModel for $ty { fn visit<V: ModelVisitor>(&self, o: WalkOrder, v: &mut V) -> Result<(), V::Error> { self.record(None,o,v) } }
    )+}; }
    trait CanonicalOrd {
        fn canonical_cmp(&self, other: &Self) -> Ordering;
    }
    macro_rules! ids { ($($t:ty),+) => {$(impl CanonicalOrd for $t { fn canonical_cmp(&self, other: &Self) -> Ordering { self.id.cmp(&other.id) } })+}; }
    ids!(Source, Track, Submix, Output, Route, Automation);
    impl CanonicalOrd for EffectParam {
        fn canonical_cmp(&self, other: &Self) -> Ordering {
            (self.parameter_id, self.channel).cmp(&(other.parameter_id, other.channel))
        }
    }
    fn array<T: Record, V: ModelVisitor>(
        key: FieldKey,
        xs: &[T],
        order: WalkOrder,
        visitor: &mut V,
    ) -> Result<(), V::Error> {
        visitor.array_begin(key, xs.len())?;
        xs.iter()
            .try_for_each(|item| item.record(None, order, visitor))?;
        visitor.array_end()
    }
    fn sorted_array<T: Record + CanonicalOrd, V: ModelVisitor>(
        key: FieldKey,
        xs: &[T],
        order: WalkOrder,
        visitor: &mut V,
    ) -> Result<(), V::Error> {
        visitor.array_begin(key, xs.len())?;
        if order == WalkOrder::Canonical
            && !xs.is_sorted_by(|a, b| a.canonical_cmp(b) != Ordering::Greater)
        {
            let mut sorted: Vec<_> = xs.iter().collect();
            sorted.sort_by(|a, b| a.canonical_cmp(b));
            sorted
                .into_iter()
                .try_for_each(|item| item.record(None, order, visitor))?;
        } else {
            xs.iter()
                .try_for_each(|item| item.record(None, order, visitor))?;
        }
        visitor.array_end()
    }

    records! {
        SessionToml=>session |s,v,o,f| [(8+s.sources.len()+s.tracks.len()+s.submixes.len()+s.outputs.len()+s.routes.len()+s.automation.len()) as u32] {
          v.u32(f::SCHEMA_VERSION,s.schema_version),v.id(f::SESSION_ID,&s.session_id),v.u64(f::REVISION,s.revision),v.u32(f::SAMPLE_RATE_HZ,s.sample_rate_hz),v.u32(f::QUANTUM_FRAMES,s.quantum_frames),
          s.render_profile.record(Some(f::RENDER_PROFILE),o,v),s.output_profile.record(Some(f::OUTPUT_PROFILE),o,v),s.limits.record(Some(f::LIMITS),o,v),
          sorted_array(f::SOURCES,&s.sources,o,v),sorted_array(f::TRACKS,&s.tracks,o,v),sorted_array(f::SUBMIXES,&s.submixes,o,v),
          sorted_array(f::OUTPUTS,&s.outputs,o,v),sorted_array(f::ROUTES,&s.routes,o,v),sorted_array(f::AUTOMATION,&s.automation,o,v)
        }
        RenderProfile=>render_profile |s,v,_o,f| [2] {v.id(f::ID,&s.id),v.token(f::MODE,token(s.mode.token(),s.mode.wire()))}
        OutputProfile=>output_profile |s,v,_o,f| [3] {v.id(f::ID,&s.id),v.u8(f::CHANNELS,s.channels),v.token(f::SAMPLE_FORMAT,token(s.sample_format.token(),s.sample_format.wire()))}
        SessionLimits=>limits |s,v,_o,f| [3] {v.u64(f::PCM_RING_FRAMES,s.pcm_ring_frames),v.u64(f::CONTROL_QUEUE_MESSAGES,s.control_queue_messages),v.u64(f::MEMORY_BYTES,s.memory_bytes)}
        Source=>source |s,v,o,f| [4] {v.id(f::ID,&s.id),v.u32(f::SAMPLE_RATE_HZ,s.sample_rate_hz),s.content.record(Some(f::CONTENT),o,v),s.mapping.record(Some(f::MAPPING),o,v)}
        SourceContent=>content |s,v,_o,f| [2] {v.text(f::IDENTITY,&s.identity),v.text(f::LOCATOR,&s.locator)}
        SourceMapping=>mapping |s,v,o,f| [2] {v.u8(f::CHANNEL_COUNT,s.channel_count),s.region.record(Some(f::REGION),o,v)}
        SourceRegion=>region |s,v,_o,f| [2] {v.u64(f::START_SAMPLE,s.start_sample),v.u64(f::LENGTH_SAMPLES,s.length_samples)}
        Track=>track |s,v,o,f| [10] {
          v.id(f::ID,&s.id),v.id(f::SOURCE_ID,&s.source_id),v.u8(f::LEFT_SOURCE_CHANNEL,s.left_source_channel),v.u8(f::RIGHT_SOURCE_CHANNEL,s.right_source_channel),
          s.builtins.record(Some(f::BUILTINS),o,v),s.simd1.record(Some(f::SIMD1),o,v),s.dynamic.record(Some(f::DYNAMIC),o,v),s.simd2.record(Some(f::SIMD2),o,v),s.fader.record(Some(f::FADER),o,v),
          {let k=match s.matrix_or_pan {MatrixOrPan::Pan{..}=>f::PAN,MatrixOrPan::Matrix{..}=>f::MATRIX};s.matrix_or_pan.record(Some(k),o,v)}
        }
        DualMonoBuiltins=>builtins |s,v,o,f| [2] {s.left.record(Some(f::LEFT),o,v),s.right.record(Some(f::RIGHT),o,v)}
        ChannelBuiltins=>channel_builtins |s,v,_o,f| [5] {v.bool(f::POLARITY_INVERT,s.polarity_invert),v.f32(f::TRIM_DB,s.trim_db),v.f32(f::HPF_HZ,s.hpf_hz),v.f32(f::LPF_HZ,s.lpf_hz),v.u32(f::DELAY_SAMPLES,s.delay_samples)}
        Rack=>rack |s,v,o,f| [s.effects.len() as u32] {array(f::EFFECTS,&s.effects,o,v)}
        Effect=>effect |s,v,o,f| [6+s.params.len() as u32] {
          v.id(f::ID,&s.id),s.identity.record(Some(f::IDENTITY),o,v),v.token(f::QUALITY,token(s.quality.token(),s.quality.wire())),v.bool(f::BYPASS,s.bypass),
          v.token(f::LINK_MODE,token(s.link_mode.token(),s.link_mode.wire())),sorted_array(f::PARAMS,&s.params,o,v),s.sidechain.record(Some(f::SIDECHAIN),o,v)
        }
        EffectParam=>param |s,v,_o,f| [4] {
          v.u32(f::PARAMETER_ID,s.parameter_id),v.token(f::CHANNEL,token(s.channel.token(),s.channel.wire())),v.token(f::UNIT,token(s.unit.token(),s.unit.wire())),v.f32(f::VALUE,s.value)
        }
        DualMonoFader=>fader |s,v,_o,f| [4] {v.f32(f::LEFT_DB,s.left_db),v.f32(f::RIGHT_DB,s.right_db),v.bool(f::LEFT_MUTE,s.left_mute),v.bool(f::RIGHT_MUTE,s.right_mute)}
        Submix=>submix |s,v,_o,f| [1] {v.id(f::ID,&s.id)}
        Output=>output |s,v,_o,f| [1] {v.id(f::ID,&s.id)}
        ChannelMatrix=>channel_matrix |s,v,_o,f| [4] {v.f32(f::LL,s.ll),v.f32(f::LR,s.lr),v.f32(f::RL,s.rl),v.f32(f::RR,s.rr)}
        Route=>route |s,v,o,f| [5] {
          v.id(f::ID,&s.id),s.source.record(Some(f::SOURCE),o,v),s.destination.record(Some(f::DESTINATION),o,v),s.channel_matrix.record(Some(f::CHANNEL_MATRIX),o,v),v.f32(f::GAIN_DB,s.gain_db)
        }
        Automation=>automation |s,v,o,f| [2+s.segments.len() as u32] {v.id(f::ID,&s.id),s.target.record(Some(f::TARGET),o,v),array(f::SEGMENTS,&s.segments,o,v)}
        AutomationTarget=>target |s,v,_o,f| [5] {
          v.id(f::ENTITY_ID,&s.entity_id),v.token(f::RACK,token(s.rack.token(),s.rack.wire())),v.id(f::EFFECT_ID,&s.effect_id),
          v.u32(f::PARAMETER_ID,s.parameter_id),v.token(f::CHANNEL,token(s.channel.token(),s.channel.wire()))
        }
        AutomationSegment=>segment |s,v,_o,f| [6] {
          v.token(f::SHAPE,token(s.shape.token(),s.shape.wire())),v.u64(f::START_SAMPLE,s.start_sample),v.u64(f::END_SAMPLE,s.end_sample),
          v.f32(f::START_VALUE,s.start_value),v.f32(f::END_VALUE,s.end_value),v.token(f::UNIT,token(s.unit.token(),s.unit.wire()))
        }
    }

    variants! {
      EffectIdentity=>identity |v,f,_o| {
        [Self::Native { effect_id }]=>2 {v.token(f::KIND,token("native",1)),v.id(f::EFFECT_ID,effect_id)}
        [Self::ThirdPartyCid { cid }]=>2 {v.token(f::KIND,token("cid",2)),v.text(f::CID,cid)}
      }
      SidechainDeclaration=>sidechain |v,f,o| {
        [Self::None]=>1 {v.token(f::KIND,token("none",1))}
        [Self::Routed(s)]=>3 {v.token(f::KIND,token("routed",2)),s.source.record(Some(f::SOURCE),o,v),v.id(f::PORT_ID,&s.port_id)}
      }
      MatrixOrPan=>matrix_or_pan |v,f,_o| {
        [Self::Pan { left, right, smoothing_samples }]=>4 {v.wire_tag(token("pan",1)),v.f32(f::LEFT,*left),v.f32(f::RIGHT,*right),v.u32(f::PAN_SMOOTHING,*smoothing_samples)}
        [Self::Matrix { ll, lr, rl, rr, smoothing_samples }]=>6 {
          v.wire_tag(token("matrix",2)),v.f32(f::LL,*ll),v.f32(f::LR,*lr),v.f32(f::RL,*rl),v.f32(f::RR,*rr),v.u32(f::MATRIX_SMOOTHING,*smoothing_samples)
        }
      }
      RouteSource=>route_source |v,f,_o| {
        [Self::Track { track_id, tap }]=>3 {v.token(f::KIND,token("track",1)),v.id(f::TRACK_ID,track_id),v.token(f::TAP,token(tap.token(),tap.wire()))}
        [Self::SubmixOutput { submix_id }]=>2 {v.token(f::KIND,token("submix_output",2)),v.id(f::SUBMIX_ID,submix_id)}
      }
      RouteDestination=>route_destination |v,f,_o| {
        [Self::SubmixInput { submix_id }]=>2 {v.token(f::KIND,token("submix_input",1)),v.id(f::SUBMIX_ID,submix_id)}
        [Self::OutputInput { output_id }]=>2 {v.token(f::KIND,token("output_input",2)),v.id(f::OUTPUT_ID,output_id)}
      }
    }
}
