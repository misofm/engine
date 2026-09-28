# Delivery codec boundary

Engine owns canonical session declarations, canonical PCM identity, generic PCM ingress, and
rendering. It does not ship a delivery codec, transport policy, catalog migration utility, or
platform publisher.

Callers resolve and authenticate transport bytes outside this repository, decode them with their
own chosen package or platform facility, verify the declared PCM shape and canonical identity, and
submit bounded decoded PCM through the existing generic ingress APIs. Browser OPFS, resolver,
ring, and AudioWorklet seams remain generic; none selects or embeds a delivery format.

There is no in-repository source-file reader: #1035 removed the native WAVE/RF64 decode workers
and #1033 the native WAV/RF64 parser and the native PCM runner that used it. Every host, the C ABI
included, submits decoded planar PCM. New delivery support requires a separately reviewed
external-package or platform-tool issue; do not restore removed code here.
