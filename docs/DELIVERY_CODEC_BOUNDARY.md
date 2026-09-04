# Delivery codec boundary

Engine owns canonical session declarations, canonical PCM identity, generic PCM ingress, and
rendering. It does not ship a delivery codec, transport policy, catalog migration utility, or
platform publisher.

Callers resolve and authenticate transport bytes outside this repository, decode them with their
own chosen package or platform facility, verify the declared PCM shape and canonical identity, and
submit bounded decoded PCM through the existing generic ingress APIs. Browser OPFS, resolver,
ring, and AudioWorklet seams remain generic; none selects or embeds a delivery format.

The only in-repository source-file reader remains the native WAVE/RF64 control-worker path. It is
not a browser delivery contract and does not add a packaged codec asset. New delivery support
requires a separately reviewed external-package or platform-tool issue; do not restore removed
code here.
