# Effect state V1

The envelope binds effect identity, prepared configuration, exact latency/tail, and a payload hash.
The payload begins with a 32-byte lane directory followed by common, left, and right sections.
Lane sections have equal lengths and retain lane-local history; a linked detector may live only in
the common section. State migration is control-plane work and never substitutes defaults silently.
