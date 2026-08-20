include "protocol-benchmark-record-validator";

length == 216 and
([.[] | .round] | sort | unique) == [1, 2] and
all(protocol_benchmark_record_valid) and
([.[] | .corpus_checksum] | unique | length) == 1 and
([.[] | .frame_label] | unique | length) == 54 and
([.[] | select(.round == 1) | .format] | .[0:54] | unique) == ["btlv"] and
([.[] | select(.round == 1) | .format] | .[54:108] | unique) == ["flatbuffers"] and
([.[] | select(.round == 2) | .format] | .[0:54] | unique) == ["flatbuffers"] and
([.[] | select(.round == 2) | .format] | .[54:108] | unique) == ["btlv"]
