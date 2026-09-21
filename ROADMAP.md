# Roadmap

## Phase 0 — Repository foundation

- [x] clean-room policy
- [x] architecture documentation
- [x] Rust workspace
- [x] initial fixed-capacity PCM frame
- [x] prototype level meter
- [x] prototype adaptive gain
- [x] prototype limiter
- [x] CI definition
- [ ] choose permanent project license

## Phase 1 — DSP laboratory

- [ ] DC blocker / high-pass filter
- [ ] compressor with configurable knee
- [ ] improved limiter with measured distortion
- [ ] resampler evaluation
- [ ] per-stream DSP state
- [ ] WAV laboratory tool
- [ ] objective level/clipping reports
- [ ] CPU and memory benchmark

## Phase 2 — XLX laboratory adapter

- [ ] document public XLXD transcoder boundary
- [ ] implement observe-only connectivity
- [ ] packet fuzzing
- [ ] external codec-backend interface
- [ ] laboratory deployment
- [ ] A/B audio tests
- [ ] rollback test
- [ ] no production activation until acceptance gates pass

## Phase 3 — PU2PNY-OS

- [ ] optional systemd unit
- [ ] health endpoint/state
- [ ] resource budgets on Raspberry Pi
- [ ] UI status only after real backend detection
- [ ] physical RF tests
- [ ] fallback without Helix

## Phase 4 — Open codecs

- [ ] M17/Codec2 adapter review
- [ ] Opus/IP adapter review
- [ ] license/provenance matrix
- [ ] cross-codec quality suite

## Phase 5 — HVC research

- [ ] requirements document
- [ ] independently designed feature representation
- [ ] bitstream draft
- [ ] encoder/decoder prototype
- [ ] packet-loss robustness
- [ ] objective and listening tests
- [ ] independent interoperability implementation
