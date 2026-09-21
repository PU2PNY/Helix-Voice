# HVC v0 — Helix Voice Codec Research Specification

Status: experimental / research
Bitstream version: 0
Compatibility promise: none yet

## 1. Purpose

HVC v0 is the first independently designed Helix speech-codec prototype. It exists to establish a complete, testable encode -> fixed bitstream -> decode path while preserving the clean-room boundary.

It is not AMBE, AMBE+, AMBE+2, Codec2, Opus, SILK, CELT, or a compatible clone of any of those codecs.

## 2. Public research basis

The design uses general, published speech-coding concepts such as linear prediction, periodic/noise excitation, energy quantization and bounded framing.

Public references used for high-level engineering context only:

- RFC 6716, Definition of the Opus Audio Codec: https://www.rfc-editor.org/rfc/rfc6716
- Codec2 project overview: https://github.com/drowe67/codec2
- M17 protocol specification: https://spec.m17project.org/

No source code, private tables, proprietary constants, firmware or closed codec implementation is copied into HVC.

## 3. v0 operating point

- mono speech;
- sample rate: 8000 Hz;
- frame duration: 20 ms;
- samples per frame: 160;
- fixed packet: 12 bytes;
- payload rate: 4800 bit/s;
- LPC/reflection order: 8;
- pitch search: 20 through 114 samples, approximately 400 Hz through 70 Hz;
- stateful decoder synthesis;
- no heap allocation required by the codec hot path.

This is a research operating point, not a final quality target.

## 4. Packet format

All fields are HVC-defined. Multi-bit packed reflection fields are LSB-first inside the 48-bit reflection block.

| Byte(s) | Field | Meaning |
|---|---|---|
| 0 | magic | 0x48, ASCII H |
| 1 | version | 0 for HVC v0 |
| 2 | voiced + energy | bit 7 voiced flag; bits 0..6 logarithmic RMS index |
| 3 | pitch lag | 20..114 when voiced; 0 when unvoiced |
| 4 | pitch confidence | bits 0..5, range 0..63; bits 6..7 reserved zero |
| 5..10 | reflection coefficients | eight 6-bit independently quantized reflection coefficients |
| 11 | CRC-8 | CRC-8 polynomial 0x07 over bytes 0..10 |

A 12-byte packet every 20 ms produces exactly 4800 bit/s before transport framing or FEC.

## 5. Encoder

The v0 encoder:

1. validates 8000 Hz / 160-sample input;
2. measures RMS energy;
3. estimates pitch using normalized autocorrelation;
4. classifies voiced/unvoiced using energy and correlation;
5. applies a Hamming analysis window;
6. computes autocorrelation;
7. derives stable reflection coefficients using Levinson-Durbin recursion;
8. quantizes energy, pitch confidence and reflection coefficients;
9. packs the HVC-specific fixed frame;
10. appends CRC-8.

## 6. Decoder

The v0 decoder:

1. validates magic, version, reserved bits, pitch semantics and CRC;
2. reconstructs reflection coefficients;
3. converts reflection coefficients to an LPC synthesis filter;
4. generates a deterministic periodic/noise excitation mixture;
5. performs LPC synthesis with per-stream state;
6. normalizes decoded RMS to the transmitted energy target;
7. clamps output to normalized PCM full scale.

## 7. Deliberate limitations

HVC v0 does not yet provide:

- wideband audio;
- perceptual weighting;
- vector quantization;
- entropy coding;
- forward error correction;
- packet-loss concealment;
- comfort noise;
- post-filtering;
- multi-stage pitch prediction;
- independent interoperable second implementation;
- objective or ABX quality evidence.

No statement that HVC v0 is better than AMBE or another codec is permitted until comparative tests exist.

## 8. Evolution rule

Any future HVC bitstream change must increment the bitstream version or define an explicitly backward-compatible extension. Test vectors must be generated from Helix-owned or redistributable synthetic material.

Canonical v0 vectors are published in `HVC_TEST_VECTORS.md`.
