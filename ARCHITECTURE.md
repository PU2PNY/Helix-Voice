# Helix Voice Architecture

## Design goal

Helix separates protocol transport, codec implementation and audio processing.

```text
network/RF transport
        |
        v
protocol adapter
        |
        v
codec adapter
        |
        v
canonical PCM frame
        |
        v
Helix DSP pipeline
        |
        v
canonical PCM frame
        |
        v
codec adapter
        |
        v
protocol adapter
```

## Canonical frame

The initial internal frame container supports mono PCM up to 48 kHz / 20 ms without heap allocation.

This is an internal engineering container, not the HVC bitstream specification.

## Crates

- `helix-core` — real-time-safe shared data types and interfaces.
- `helix-dsp` — audio level measurement, AGC and limiter prototypes.
- `helix-xlx` — XLX integration boundary; no proprietary codec implementation.
- `helix-daemon` — future service process.

Planned later:

- codec/plugin ABI;
- resampler;
- jitter buffer;
- packet-loss concealment;
- metrics/export;
- M17/Codec2 adapter after license/API review;
- optional Opus adapter;
- laboratory legacy-codec backend boundary;
- HVC research crate kept separate from interoperability code.

## Real-time rules

The audio hot path should:

- avoid heap allocation;
- use bounded buffers;
- avoid blocking I/O;
- avoid logging per audio sample;
- avoid global locks when possible;
- make latency measurable;
- fail closed on malformed packets.

## Process boundary

XLXD integration should initially use a separate process so an HVE crash cannot crash the reflector and rollback remains trivial.

PU2PNY-OS integration should likewise use an optional systemd service and must not prevent boot if Helix is unavailable.
