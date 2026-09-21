# Contributing to Helix Voice

Contributions are welcome during the research phase, but the clean-room boundary is mandatory.

Read **CLEAN_ROOM.md** before submitting code.

## Development principles

- Prefer small, reviewable commits.
- No silent behavioral changes.
- No unbounded queues.
- No allocation in the real-time audio hot path unless justified and benchmarked.
- No root requirement in normal operation.
- No external network dependency for core audio processing.
- Treat protocol parsing as untrusted input.
- Preserve rollback for integration changes.
- Measure before claiming an audio or performance improvement.

## Rust quality gate

Before submitting:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## DSP changes

DSP pull requests should state:

- expected audible effect;
- sample rates tested;
- frame sizes tested;
- added latency;
- CPU impact;
- clipping behavior;
- test corpus provenance;
- objective measurements used.

## Third-party dependencies

New dependencies require an entry in `THIRD_PARTY.md` before merge.

Avoid dependencies when a small, well-tested implementation is practical.

## Compatibility changes

Do not claim compatibility with D-STAR, DMR, YSF, P25, NXDN, M17 or any other system unless the exact implemented path has been tested.

A protocol name in a roadmap is not a compatibility claim.
