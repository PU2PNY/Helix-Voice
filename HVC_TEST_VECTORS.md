# HVC v0 — Canonical Test Vectors

Status: experimental / version 0

These vectors are generated specifically for Helix and may be redistributed with the project. They do not contain captured proprietary codec data.

## TV-HVC-0001 — Digital silence

Input:
- sample rate: 8000 Hz;
- frame: 160 mono `f32` samples;
- every sample: exactly `0.0`;
- timestamp: not encoded in HVC v0.

Expected HVC v0 packet, hexadecimal:

```text
48 00 00 00 00 20 08 82 20 08 82 7E
```

Expected fields:
- magic: `0x48`;
- version: `0`;
- voiced: false;
- energy index: `0`;
- pitch lag: `0`;
- pitch confidence: `0`;
- quantized neutral reflection block: `20 08 82 20 08 82`;
- CRC-8: `0x7E`.

Expected decoded PCM:
- 160 samples;
- all samples exactly `0.0` in the v0 decoder silence path.

Covered by Rust test:
`silence_has_stable_reference_vector`.

## TV-HVC-0002 — Single-bit corruption rejection

Start from any valid encoder-produced HVC v0 frame and flip exactly one bit in any of the 12 packet bytes.

Expected result:
- `HvcPacket::from_bytes` rejects the packet.

The workspace test iterates all 96 possible single-bit positions. It checks packet field validation plus CRC behavior and is covered by:
`every_single_bit_corruption_is_rejected`.

## TV-HVC-0003 — Stateful stability

Generate 500 consecutive voiced synthetic frames with frequency varying from 85 Hz upward through a bounded deterministic sequence, encode and decode using one persistent decoder state.

Expected result:
- every decoded sample is finite;
- every decoded sample remains within normalized full scale `[-1.0, +1.0]`.

Covered by:
`long_stateful_round_trip_stays_finite_and_bounded`.

## Conformance rule

Changing TV-HVC-0001 without incrementing an incompatible bitstream version is prohibited. A future second implementation must reproduce/consume the canonical vectors before HVC interoperability may be claimed.
