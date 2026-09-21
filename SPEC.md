# HVE v0 Engineering Specification

Status: **draft / research**

## 1. Audio representation

Initial canonical PCM:

- mono;
- normalized `f32` samples in nominal range -1.0 to +1.0;
- 8 kHz through 48 kHz;
- bounded frame container;
- timestamp expressed in sample units;
- no implicit sample-rate conversion.

## 2. Pipeline

Initial DSP order:

1. input validation;
2. level measurement;
3. silence/noise-gate decision for AGC control;
4. adaptive gain;
5. limiter;
6. output measurement.

Future stages may add:

- high-pass/DC blocker;
- compressor;
- noise reduction;
- equalization;
- resampling;
- PLC.

Each stage must declare latency and CPU impact.

## 3. AGC goals

AGC must:

- raise low speech gradually;
- attenuate excessively high speech faster than it boosts;
- avoid boosting silence;
- cap total gain;
- avoid clipping;
- preserve transient intelligibility;
- maintain state per stream, never globally across unrelated callers.

## 4. Level consistency

Protocol profiles may define different spectral shaping or target levels only when justified by measured endpoint behavior.

Fixed per-protocol gain hacks are not the primary normalization mechanism.

## 5. Latency

Latency must be reported as:

- algorithmic latency;
- buffering latency;
- codec latency;
- network/jitter latency.

No release may claim "low latency" without measurement.

## 6. Codec boundary

The HVE API will distinguish:

- open codec implementation;
- external licensed codec backend;
- unsupported codec.

A protocol adapter must not pretend a codec exists when no legal implementation/backend is available.

## 7. HVC

HVC is a separate research track.

An experimental HVC v0 bitstream is now defined in `HVC_SPEC.md` and implemented by `helix-hvc`. It is a research format, not a standardized or production compatibility contract.

Canonical synthetic vectors live in `HVC_TEST_VECTORS.md`.

No interoperability claim should be made until a second independent implementation consumes the published specification/vectors successfully. No quality claim should be made until objective and level-matched listening evidence exists.
