# PU2PNY-OS Integration Plan

## Objective

Add Helix as an optional audio engine without making the hotspot depend on it for boot, networking, RF setup or the web panel.

## Service model

Proposed service:

`helix-voice.service`

Properties:

- disabled until a supported function is configured;
- unprivileged user;
- systemd hardening;
- bounded memory;
- restart policy with rate limiting;
- local IPC;
- explicit health state.

## UI states

The UI must never infer support.

Allowed states:

- unavailable;
- disabled;
- starting;
- ready;
- degraded;
- error.

## Resource gate

Before enabling by default on any Raspberry Pi family, measure:

- idle RSS;
- active transcoding RSS;
- idle CPU;
- active CPU;
- SoC temperature;
- end-to-end audio latency;
- underruns/overruns.

Original Pi Zero/ARMv6 requires separate validation and may not support all Helix features.
