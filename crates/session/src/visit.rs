//! One public, schema-keyed emit-side walk over the complete session model.

use crate::StableId;

/// One schema field: canonical JSON key and BTLV field id.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldKey {
    /// Canonical JSON field name.
    pub name: &'static str,
    /// Stable BTLV field identifier.
    pub id: u16,
}

/// One closed-set token: canonical text and BTLV `u8`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    /// Canonical JSON token.
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
    /// Name the record just begun: the key module its keys belong to and, for a variant record,
    /// the arm. The walk calls it right after [`record_begin`](Self::record_begin) and before any
    /// field or [`wire_tag`](Self::wire_tag), exactly once per record. `module` is a name in
    /// [`keys::MODULES`]; `arm` is the variant arm's token text (`"pan"`, `"routed"`), and `None`
    /// for a record with one shape. A visitor that does not need the schema's names keeps this
    /// default.
    fn record_schema(
        &mut self,
        _module: &'static str,
        _arm: Option<&'static str>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    /// End the current record.
    fn record_end(&mut self) -> Result<(), Self::Error>;
    /// Begin an array with its exact item count.
    fn array_begin(&mut self, key: FieldKey, len: usize) -> Result<(), Self::Error>;
    /// End the current array.
    fn array_end(&mut self) -> Result<(), Self::Error>;
    /// Emit a discriminator present only on the wire, immediately after `record_schema`.
    fn wire_tag(&mut self, tag: Token) -> Result<(), Self::Error>;
    /// Emit a Boolean field.
    fn bool(&mut self, key: FieldKey, value: bool) -> Result<(), Self::Error>;
    /// Emit an unsigned eight-bit field.
    fn u8(&mut self, key: FieldKey, value: u8) -> Result<(), Self::Error>;
    /// Emit an unsigned 32-bit field.
    fn u32(&mut self, key: FieldKey, value: u32) -> Result<(), Self::Error>;
    /// Emit an unsigned 64-bit field.
    fn u64(&mut self, key: FieldKey, value: u64) -> Result<(), Self::Error>;
    /// Emit the mixed JSON number/string source bit-depth token.
    fn source_bit_depth(
        &mut self,
        key: FieldKey,
        value: crate::SourceBitDepth,
    ) -> Result<(), Self::Error>;
    /// Emit a finite 32-bit float field.
    fn f32(&mut self, key: FieldKey, value: f32) -> Result<(), Self::Error>;
    /// Emit a stable-ID field.
    fn id(&mut self, key: FieldKey, value: &StableId) -> Result<(), Self::Error>;
    /// Emit one stable ID as an element of the open array (a VCA's `members`, #1240): the
    /// schema's one array of scalars. Each item is a repeated field of the array's key.
    fn id_item(&mut self, value: &StableId) -> Result<(), Self::Error>;
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

macro_rules! key_modules { ($(($name:ident,$doc:literal;$($field:ident=$text:literal:$id:literal),+ $(,)?))+) => {
    $(#[doc=$doc] pub mod $name {
        use super::*;
        $(#[doc=concat!("Schema key `",$text,"`.")] pub const $field: FieldKey=FieldKey{name:$text,id:$id};)+
        /// Every field of this record, in declaration order.
        pub const ALL: &[FieldKey]=&[$($field),+];
    })+
    /// Every record module's name and fields, in declaration order.
    pub const MODULES: &[(&str,&[FieldKey])]=&[$((stringify!($name),$name::ALL)),+];
}; }

/// Public field-key registry. The IDs are the wire schema's: `protocol/src/schema.rs`'s session
/// specs own the wire types, and `session_keys_and_wire_fields_have_equal_ids` in that file holds
/// each record's key IDs equal to its wire field IDs.
#[rustfmt::skip]
pub mod keys {
 use super::FieldKey;
 key_modules! {
 (session,"Session root fields.";SCHEMA_VERSION="schema_version":1,SESSION_ID="session_id":2,REVISION="revision":3,SAMPLE_RATE_HZ="sample_rate_hz":4,QUANTUM_FRAMES="quantum_frames":5,RENDER_PROFILE="render_profile":6,OUTPUT_PROFILE="output_profile":7,SOURCES="sources":9,CONSOLE="console":15,TRACKS="tracks":10,SUBMIXES="submixes":11,VCAS="vcas":16,CONTROL_SMOOTHING="control_smoothing":17,OUTPUTS="outputs":12,ROUTES="routes":13,AUTOMATION="automation":14)
 (console,"Session console fields (decision 12).";PRE_INSERT="pre_insert":1,POST_INSERT="post_insert":2)
 (console_slot,"Console slot declaration fields.";SLOT="slot":1,IDENTITY="identity":2,QUALITY="quality":3,LINK_MODE="link_mode":4)
 (console_entry,"Per-track console entry fields.";SLOT="slot":1,BYPASS="bypass":2,PARAMS="params":3)
 (render_profile,"Render-profile fields.";ID="id":1,MODE="mode":2)
 (output_profile,"Output-profile fields.";ID="id":1,CHANNELS="channels":2,SAMPLE_FORMAT="sample_format":3)
 (source,"Source fields.";ID="id":1,CONTENT="content":2,CHANNELS="channels":3,BIT_DEPTH="bit_depth":4,FRAMES="frames":5)
 // Track fields 6 (`simd1`) and 8 (`simd2`) are retired by decision 12 and never reallocated;
 // `inserts` keeps the retired `dynamic` rack's field 7 and `console` is appended as 11.
 (track,"Track fields.";ID="id":1,SOURCE_ID="source_id":2,LEFT_SOURCE_CHANNEL="left_source_channel":3,RIGHT_SOURCE_CHANNEL="right_source_channel":4,BUILTINS="builtins":5,CONSOLE="console":11,INSERTS="inserts":7,FADER="fader":9,PAN="pan":10,MATRIX="matrix":10)
 (builtins,"Dual-mono builtins fields.";LEFT="left":1,RIGHT="right":2)
 (channel_builtins,"Channel-builtin fields.";POLARITY_INVERT="polarity_invert":1,TRIM_DB="trim_db":2,HPF_HZ="hpf_hz":3,LPF_HZ="lpf_hz":4,DELAY_SAMPLES="delay_samples":5)
 (rack,"Rack fields.";EFFECTS="effects":1)
 (effect,"Effect fields.";ID="id":1,IDENTITY="identity":2,QUALITY="quality":3,BYPASS="bypass":4,LINK_MODE="link_mode":5,PARAMS="params":6,SIDECHAIN="sidechain":7)
 (identity,"Effect-identity fields.";KIND="kind":1,EFFECT_ID="effect_id":2,CID="cid":2)
 (param,"Effect-parameter fields.";PARAMETER_ID="parameter_id":1,CHANNEL="channel":2,UNIT="unit":3,VALUE="value":4)
 (sidechain,"Sidechain fields.";KIND="kind":1,SOURCE="source":2,PORT_ID="port_id":3)
 (fader,"Fader fields.";LEFT_DB="left_db":1,RIGHT_DB="right_db":2,LEFT_MUTE="left_mute":3,RIGHT_MUTE="right_mute":4)
 (matrix_or_pan,"Matrix-or-pan fields.";LEFT="left":2,RIGHT="right":3,PAN_SMOOTHING="smoothing_samples":4,LL="ll":2,LR="lr":3,RL="rl":4,RR="rr":5,MATRIX_SMOOTHING="smoothing_samples":6)
 // Field 3 is `console`, the reserved field #1202 appends; pan and matrix share tagged field 6,
 // exactly as the track's field 10 (#1199 D1).
 (submix,"Submix strip fields.";ID="id":1,BUILTINS="builtins":2,CONSOLE="console":3,INSERTS="inserts":4,FADER="fader":5,PAN="pan":6,MATRIX="matrix":6)
 // Root field 8 was `limits` (removed in 04d291dd); it is retired and never reallocated. `vcas`
 // is appended as 16 (#1240).
 (vca,"VCA group fields (#1240).";ID="id":1,FADER="fader":2,MEMBERS="members":3)
 // `control_smoothing` is appended as root field 17 (#1054); field 8 stays retired.
 (control_smoothing,"Control-smoothing fields (#1054).";KIND="kind":1,MUTE_MS="mute_ms":2,FADER_MS="fader_ms":3,PAN_MS="pan_ms":4)
 (output,"Output fields.";ID="id":1)
 (route,"Route fields.";ID="id":1,SOURCE="source":2,DESTINATION="destination":3,CHANNEL_MATRIX="channel_matrix":4,GAIN_DB="gain_db":5,MUTE="mute":6,FOLLOWS_MUTE="follows_mute":7)
 (route_source,"Route-source fields.";KIND="kind":1,TRACK_ID="track_id":2,SUBMIX_ID="submix_id":2,TAP="tap":3)
 (route_destination,"Route-destination fields.";KIND="kind":1,SUBMIX_ID="submix_id":2,OUTPUT_ID="output_id":2)
 (channel_matrix,"Channel-matrix fields.";LL="ll":1,LR="lr":2,RL="rl":3,RR="rr":4)
 (automation,"Automation fields.";ID="id":1,TARGET="target":2,SEGMENTS="segments":3)
 (target,"Automation-target fields.";ENTITY_ID="entity_id":1,RACK="rack":2,EFFECT_ID="effect_id":3,PARAMETER_ID="parameter_id":4,CHANNEL="channel":5)
 (segment,"Automation-segment fields.";SHAPE="shape":1,START_SAMPLE="start_sample":2,END_SAMPLE="end_sample":3,START_VALUE="start_value":4,END_VALUE="end_value":5,UNIT="unit":6)
 }
}

pub use walk::VARIANT_ARMS;

mod walk {
    use core::cmp::Ordering;

    use super::{FieldKey, ModelVisitor, Token, VisitModel, WalkOrder, keys};
    use crate::*;

    fn token(text: &'static str, wire: u8) -> Token {
        Token { text, wire }
    }
    /// A strip's record shape follows its pan-or-matrix field: the key list differs by arm.
    fn strip_arm(matrix_or_pan: &MatrixOrPan) -> &'static str {
        match matrix_or_pan {
            MatrixOrPan::Pan { .. } => "pan",
            MatrixOrPan::Matrix { .. } => "matrix",
        }
    }
    trait Record {
        fn record<V: ModelVisitor>(
            &self,
            key: Option<FieldKey>,
            order: WalkOrder,
            v: &mut V,
        ) -> Result<(), V::Error>;
    }
    macro_rules! records { ($($ty:ty=>$m:ident |$s:ident,$v:ident,$o:ident,$f:ident| [$count:expr $(; $arm:expr)?] {$($event:expr),*})+) => {$(
        impl Record for $ty { fn record<V: ModelVisitor>(&self, key: Option<FieldKey>, o: WalkOrder, v: &mut V) -> Result<(), V::Error> {
            use keys::$m as $f; let ($s,$v,$o)=(self,v,o);
            $v.record_begin(key,$count)?; $v.record_schema(stringify!($m),None $(.or(Some($arm)))?)?; $($event?;)* $v.record_end()
        } }
        impl VisitModel for $ty { fn visit<V: ModelVisitor>(&self, o: WalkOrder, v: &mut V) -> Result<(), V::Error> { self.record(None,o,v) } }
    )+}; }
    macro_rules! rec { ($v:ident,$key:expr,$n:expr,$m:ident,$arm:literal;$($e:expr),*$(,)?) => {{ $v.record_begin($key,$n)?; $v.record_schema(stringify!($m),Some($arm))?; $($e?;)* $v.record_end() }}; }
    macro_rules! variants { ($($ty:ty=>$m:ident |$v:ident,$f:ident,$o:ident| {$([$pat:pat]=>$n:literal $arm:literal {$($e:expr),*})+})+) => {
        /// Every variant record's `(key module, arm)`, so a consumer of [`ModelVisitor::record_schema`] can tell an arm it never saw from one that does not exist.
        pub const VARIANT_ARMS: &[(&str, &str)] = &[$($((stringify!($m), $arm)),+),+];
        $(
        impl Record for $ty {
            fn record<V: ModelVisitor>(&self, key: Option<FieldKey>, order: WalkOrder, visitor: &mut V) -> Result<(), V::Error> {
                use keys::$m as $f; let ($v,$o)=(visitor,order);
                match self {$($pat=>rec!($v,key,$n,$m,$arm;$($e),*),)+}
            }
        }
        impl VisitModel for $ty { fn visit<V: ModelVisitor>(&self, o: WalkOrder, v: &mut V) -> Result<(), V::Error> { self.record(None,o,v) } }
    )+}; }
    trait CanonicalOrd {
        fn canonical_cmp(&self, other: &Self) -> Ordering;
    }
    macro_rules! ids { ($($t:ty),+) => {$(impl CanonicalOrd for $t { fn canonical_cmp(&self, other: &Self) -> Ordering { self.id.cmp(&other.id) } })+}; }
    ids!(Source, Track, Submix, Vca, Output, Route, Automation);
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

    /// An array of stable IDs, sorted by ID in canonical order (a VCA's `members`, #1240).
    fn id_array<V: ModelVisitor>(
        key: FieldKey,
        ids: &[StableId],
        order: WalkOrder,
        visitor: &mut V,
    ) -> Result<(), V::Error> {
        visitor.array_begin(key, ids.len())?;
        if order == WalkOrder::Canonical && !ids.is_sorted() {
            let mut sorted: Vec<_> = ids.iter().collect();
            sorted.sort();
            sorted.into_iter().try_for_each(|id| visitor.id_item(id))?;
        } else {
            ids.iter().try_for_each(|id| visitor.id_item(id))?;
        }
        visitor.array_end()
    }

    records! {
        SessionModel=>session |s,v,o,f| [(9+s.sources.len()+s.tracks.len()+s.submixes.len()+s.vcas.len()+s.outputs.len()+s.routes.len()+s.automation.len()) as u32] {
          v.u32(f::SCHEMA_VERSION,s.schema_version),v.id(f::SESSION_ID,&s.session_id),v.u64(f::REVISION,s.revision),v.u32(f::SAMPLE_RATE_HZ,s.sample_rate_hz),v.u32(f::QUANTUM_FRAMES,s.quantum_frames),
          s.render_profile.record(Some(f::RENDER_PROFILE),o,v),s.output_profile.record(Some(f::OUTPUT_PROFILE),o,v),
          sorted_array(f::SOURCES,&s.sources,o,v),s.console.record(Some(f::CONSOLE),o,v),sorted_array(f::TRACKS,&s.tracks,o,v),sorted_array(f::SUBMIXES,&s.submixes,o,v),sorted_array(f::VCAS,&s.vcas,o,v),s.control_smoothing.record(Some(f::CONTROL_SMOOTHING),o,v),
          sorted_array(f::OUTPUTS,&s.outputs,o,v),sorted_array(f::ROUTES,&s.routes,o,v),sorted_array(f::AUTOMATION,&s.automation,o,v)
        }
        RenderProfile=>render_profile |s,v,_o,f| [2] {v.id(f::ID,&s.id),v.token(f::MODE,token(s.mode.token(),s.mode.wire()))}
        OutputProfile=>output_profile |s,v,_o,f| [3] {v.id(f::ID,&s.id),v.u8(f::CHANNELS,s.channels),v.token(f::SAMPLE_FORMAT,token(s.sample_format.token(),s.sample_format.wire()))}
        Source=>source |s,v,_o,f| [5] {v.id(f::ID,&s.id),v.text(f::CONTENT,&s.content),v.u8(f::CHANNELS,s.channels),v.source_bit_depth(f::BIT_DEPTH,s.bit_depth),v.u64(f::FRAMES,s.frames)}
        Console=>console |s,v,o,f| [(s.pre_insert.len()+s.post_insert.len()) as u32] {array(f::PRE_INSERT,&s.pre_insert,o,v),array(f::POST_INSERT,&s.post_insert,o,v)}
        ConsoleSlot=>console_slot |s,v,o,f| [4] {
          v.id(f::SLOT,&s.slot),s.identity.record(Some(f::IDENTITY),o,v),v.token(f::QUALITY,token(s.quality.token(),s.quality.wire())),
          v.token(f::LINK_MODE,token(s.link_mode.token(),s.link_mode.wire()))
        }
        ConsoleEntry=>console_entry |s,v,o,f| [2+s.params.len() as u32] {v.id(f::SLOT,&s.slot),v.bool(f::BYPASS,s.bypass),sorted_array(f::PARAMS,&s.params,o,v)}
        Track=>track |s,v,o,f| [8+s.console.len() as u32; strip_arm(&s.matrix_or_pan)] {
          v.id(f::ID,&s.id),v.id(f::SOURCE_ID,&s.source_id),v.u8(f::LEFT_SOURCE_CHANNEL,s.left_source_channel),v.u8(f::RIGHT_SOURCE_CHANNEL,s.right_source_channel),
          s.builtins.record(Some(f::BUILTINS),o,v),array(f::CONSOLE,&s.console,o,v),s.inserts.record(Some(f::INSERTS),o,v),s.fader.record(Some(f::FADER),o,v),
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
        Submix=>submix |s,v,o,f| [5+s.console.len() as u32; strip_arm(&s.matrix_or_pan)] {
          v.id(f::ID,&s.id),s.builtins.record(Some(f::BUILTINS),o,v),array(f::CONSOLE,&s.console,o,v),s.inserts.record(Some(f::INSERTS),o,v),s.fader.record(Some(f::FADER),o,v),
          {let k=match s.matrix_or_pan {MatrixOrPan::Pan{..}=>f::PAN,MatrixOrPan::Matrix{..}=>f::MATRIX};s.matrix_or_pan.record(Some(k),o,v)}
        }
        Vca=>vca |s,v,o,f| [2+s.members.len() as u32] {v.id(f::ID,&s.id),s.fader.record(Some(f::FADER),o,v),id_array(f::MEMBERS,&s.members,o,v)}
        Output=>output |s,v,_o,f| [1] {v.id(f::ID,&s.id)}
        ChannelMatrix=>channel_matrix |s,v,_o,f| [4] {v.f32(f::LL,s.ll),v.f32(f::LR,s.lr),v.f32(f::RL,s.rl),v.f32(f::RR,s.rr)}
        Route=>route |s,v,o,f| [7] {
          v.id(f::ID,&s.id),s.source.record(Some(f::SOURCE),o,v),s.destination.record(Some(f::DESTINATION),o,v),s.channel_matrix.record(Some(f::CHANNEL_MATRIX),o,v),v.f32(f::GAIN_DB,s.gain_db),v.bool(f::MUTE,s.mute),v.bool(f::FOLLOWS_MUTE,s.follows_mute)
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
        [Self::Native { effect_id }]=>2 "native" {v.token(f::KIND,token("native",1)),v.id(f::EFFECT_ID,effect_id)}
        [Self::ThirdPartyCid { cid }]=>2 "cid" {v.token(f::KIND,token("cid",2)),v.text(f::CID,cid)}
      }
      SidechainDeclaration=>sidechain |v,f,o| {
        [Self::None]=>1 "none" {v.token(f::KIND,token("none",1))}
        [Self::Routed(s)]=>3 "routed" {v.token(f::KIND,token("routed",2)),s.source.record(Some(f::SOURCE),o,v),v.id(f::PORT_ID,&s.port_id)}
      }
      MatrixOrPan=>matrix_or_pan |v,f,_o| {
        [Self::Pan { left, right, smoothing_samples }]=>4 "pan" {v.wire_tag(token("pan",1)),v.f32(f::LEFT,*left),v.f32(f::RIGHT,*right),v.u32(f::PAN_SMOOTHING,*smoothing_samples)}
        [Self::Matrix { ll, lr, rl, rr, smoothing_samples }]=>6 "matrix" {
          v.wire_tag(token("matrix",2)),v.f32(f::LL,*ll),v.f32(f::LR,*lr),v.f32(f::RL,*rl),v.f32(f::RR,*rr),v.u32(f::MATRIX_SMOOTHING,*smoothing_samples)
        }
      }
      Option<ControlSmoothing> => control_smoothing |v,f,_o| {
        [None]=>1 "default" {v.token(f::KIND,token("default",1))}
        [Some(ControlSmoothing { mute_ms, fader_ms, pan_ms })]=>4 "explicit" {
          v.token(f::KIND,token("explicit",2)),v.f32(f::MUTE_MS,*mute_ms),v.f32(f::FADER_MS,*fader_ms),v.f32(f::PAN_MS,*pan_ms)
        }
      }
      RouteSource=>route_source |v,f,_o| {
        [Self::Track { track_id, tap }]=>3 "track" {v.token(f::KIND,token("track",1)),v.id(f::TRACK_ID,track_id),v.token(f::TAP,token(tap.token(),tap.wire()))}
        [Self::Submix { submix_id, tap }]=>3 "submix" {v.token(f::KIND,token("submix",2)),v.id(f::SUBMIX_ID,submix_id),v.token(f::TAP,token(tap.token(),tap.wire()))}
      }
      RouteDestination=>route_destination |v,f,_o| {
        [Self::SubmixInput { submix_id }]=>2 "submix_input" {v.token(f::KIND,token("submix_input",1)),v.id(f::SUBMIX_ID,submix_id)}
        [Self::OutputInput { output_id }]=>2 "output_input" {v.token(f::KIND,token("output_input",2)),v.id(f::OUTPUT_ID,output_id)}
      }
    }
}
