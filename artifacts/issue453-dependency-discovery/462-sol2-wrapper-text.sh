#!/usr/bin/env bash
printf 'cwd=%q args=' "$PWD" >>/tmp/462-sol2-relocation/trace.log
printf ' %q' "$@" >>/tmp/462-sol2-relocation/trace.log
printf '\n' >>/tmp/462-sol2-relocation/trace.log
exec /home/bl/.cargo/bin/cargo "$@"
