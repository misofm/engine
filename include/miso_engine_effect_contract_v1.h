#ifndef MISO_ENGINE_EFFECT_CONTRACT_V1_H
#define MISO_ENGINE_EFFECT_CONTRACT_V1_H
#include <stdint.h>
typedef struct { uint32_t struct_size,parameter_id,unit,domain,mapping,automation_rate,channel_policy,flags; float minimum,maximum,default_value; uint32_t smoothing_samples,name_offset,name_len,display_unit_offset,display_unit_len,enum_start,enum_count,smoothing_rule,reserved; } MisoEngineEffectParameterDescriptorV1;
typedef struct { float value; uint32_t label_offset,label_len,reserved; } MisoEngineEffectEnumChoiceV1;
typedef struct { uint32_t id_offset,id_len,kind,required,lane_layout,flags,reserved0,reserved1; } MisoEngineEffectPortDescriptorV1;
typedef struct { uint32_t quality,sample_rate; uint64_t latency; uint32_t tail_kind,flags; uint64_t tail_samples; uint32_t maximum_state_bytes,scratch_fixed_bytes,scratch_bytes_per_frame,reserved; } MisoEngineEffectQualityDescriptorV1;
_Static_assert(sizeof(MisoEngineEffectParameterDescriptorV1)==80,"parameter layout");
_Static_assert(sizeof(MisoEngineEffectEnumChoiceV1)==16,"enum layout");
_Static_assert(sizeof(MisoEngineEffectPortDescriptorV1)==32,"port layout");
_Static_assert(sizeof(MisoEngineEffectQualityDescriptorV1)==48,"quality layout");
#endif
