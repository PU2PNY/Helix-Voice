# Third-Party Registry

No third-party code is currently linked into the initial Rust workspace.

Projects being evaluated as references or future optional integrations:

| Project | Purpose | Status |
|---|---|---|
| Rust | implementation language/toolchain | selected |
| XLXD | public reflector integration reference | reference only |
| Codec2 | public research context + future open digital-voice adapter candidate | reference only; no code copied/linked |
| M17 / libm17 | public protocol/codec integration context | reference only; no code copied/linked |
| Opus / RFC 6716 | public speech/audio coding context + future IP adapter candidate | reference only; no code copied/linked |
| RNNoise | optional denoise research | license/model review required |
| FreeDV / RADE | research reference for modern radio voice | reference/evaluation |

Before linking, vendoring or redistributing any dependency, record exact upstream URL, version/commit and verified license here.

The existence of an adapter or reference does not mean its implementation is part of Helix core.


## HVC v0 research provenance

The HVC v0 implementation was written independently. These public sources were used only to confirm general, published speech-coding concepts and open-radio context; no source code or bitstream layout was copied:

- RFC 6716 — https://www.rfc-editor.org/rfc/rfc6716
- Codec2 upstream — https://github.com/drowe67/codec2
- M17 specification — https://spec.m17project.org/

The HVC packet layout, quantization choices, version byte, CRC placement and synthetic canonical test vector are defined by Helix in `HVC_SPEC.md` / `HVC_TEST_VECTORS.md`.


## Patent/FTO research

Patent databases and official patent-office guidance are research sources, not software dependencies.

Required sources when applicable:
- WIPO PATENTSCOPE / WIPO FTO guidance;
- EPO Espacenet;
- USPTO;
- INPI Brasil;
- additional national offices for intended deployment/commercialization territories.

Per-feature results must be recorded under docs/patent-reviews/ using docs/PATENT_REVIEW_TEMPLATE.md.

A negative search does not establish freedom to operate.
